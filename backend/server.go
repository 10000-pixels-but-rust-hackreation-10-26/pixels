package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"sync"
	"time"

	"github.com/go-redis/redis/v8"
	"github.com/gorilla/websocket"
)

type Server struct {
	broadcast  chan IncomingMessage
	register   chan *Client
	unregister chan *Client
	numclients chan int

	redisClient *redis.Client
	rateLimits  sync.Map
}

func NewServer() *Server {
	rdb := redis.NewClient(&redis.Options{
		Addr:     os.Getenv("REDIS_ADDRESS"),
		Password: os.Getenv("REDIS_PASSWORD"),
		DB:       0,
	})

	return &Server{
		broadcast:   make(chan IncomingMessage, 100000),
		register:    make(chan *Client, 10000),
		unregister:  make(chan *Client, 10000),
		numclients:  make(chan int),
		redisClient: rdb,
	}
}

func (server *Server) Run(ctx context.Context) {
	clients := make(map[*Client]struct{})

	for {
		select {
		case <-ctx.Done():
			return

		case update := <-server.broadcast:
			err := server.redisClient.SetRange(ctx, "pixels", int64(update.Data.Index), update.Data.Color).Err()
			if err != nil {
				log.Printf("error updating Redis: %v", err)
				continue
			}

			msg := OutgoingMessage{Type: "update", Data: update.Data, ClientCount: len(clients)}
			jsonMsg, err := json.Marshal(msg)
			if err != nil {
				log.Printf("error marshaling json: %v", err)
				continue
			}

			for client := range clients {
				err := client.WriteMessage(websocket.TextMessage, jsonMsg)
				if err != nil {
					log.Printf("error sending message to client: %v", err)
					client.Close()
					delete(clients, client)
				}
			}

		case client := <-server.register:
			clients[client] = struct{}{}

		case client := <-server.unregister:
			delete(clients, client)
			client.Close()

		case server.numclients <- len(clients):
		}
	}
}

var upgrader = websocket.Upgrader{
	ReadBufferSize:  64,
	WriteBufferSize: 10240,
	CheckOrigin: func(r *http.Request) bool {
		origin := r.Header.Get("Origin")
		environment := os.Getenv("ENVIRONMENT")
		if environment == "development" {
			return origin == "http://127.0.0.1:5500"
		} else {
			return origin == "https://tenthousandpixels.com"
		}
	},
}

func (server *Server) handleConnections(rw http.ResponseWriter, req *http.Request) {
	hCaptchaToken := req.Header.Get("Sec-WebSocket-Protocol")
	if err := verifyHCaptcha(hCaptchaToken); err != nil {
		log.Printf("error verifying hCaptcha: %v", err)
		http.Error(rw, "could not verify hCaptcha", http.StatusUnauthorized)
		return
	}

	conn, err := upgrader.Upgrade(rw, req, http.Header{"Sec-WebSocket-Protocol": {hCaptchaToken}})
	if err != nil {
		log.Printf("error upgrading connection: %v", err)
		http.Error(rw, "could not open websocket connection", http.StatusInternalServerError)
		return
	}
	client := NewClient(conn)

	ip := getIP(req)
	if !server.checkAndUpdateClientCount(ip, true) {
		client.WriteMessage(websocket.TextMessage, []byte("client limit exceeded"))
		client.Close()
		return
	}

	server.register <- client

	defer func() {
		server.unregister <- client
		server.checkAndUpdateClientCount(ip, false)
		client.Close()
	}()

	time.AfterFunc(30*time.Minute, func() {
		server.unregister <- client
		server.checkAndUpdateClientCount(ip, false)
		client.Close()
	})

	initialData, err := server.redisClient.Get(req.Context(), "pixels").Result()
	if err != nil {
		log.Printf("error getting initial data from Redis: %v", err)
		return
	}

	initialMsg := InitialMessage{Type: "initial", Data: initialData, ClientCount: <-server.numclients}
	jsonMsg, err := json.Marshal(initialMsg)
	if err != nil {
		log.Printf("error marshaling initial JSON: %v", err)
		return
	}

	err = client.WriteMessage(websocket.TextMessage, jsonMsg)
	if err != nil {
		log.Printf("error sending initial message: %v", err)
		return
	}

	for {
		_, msgBytes, err := client.ReadMessage()
		if err != nil {
			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
				log.Printf("error reading message: %v", err)
			}
			break
		}

		var update IncomingMessage
		err = json.Unmarshal(msgBytes, &update)
		if err != nil {
			log.Printf("error unmarshaling JSON: %v", err)
			client.WriteMessage(websocket.TextMessage, []byte("Invalid input type"))
			continue
		}

		if err := validateIncomingMessage(update); err != nil {
			log.Printf("Invalid update message from client: %v", err)
			client.WriteMessage(websocket.TextMessage, []byte(fmt.Sprintf("Error: %v", err)))
			continue
		}

		if !server.checkRateLimit(ip) {
			client.WriteMessage(websocket.TextMessage, []byte("rate limit exceeded"))
			continue
		}

		log.Printf("Pixel updated: index=%d, color=%s, ip=%s", update.Data.Index, update.Data.Color, ip)

		server.broadcast <- update
	}
}

func (server *Server) handleGetPixels(rw http.ResponseWriter, req *http.Request) {
	err := server.redisClient.SetRange(req.Context(), "pixels", 0, "0").Err()
	pixelsData, err := server.redisClient.Get(req.Context(), "pixels").Result()
	if err != nil {
		log.Printf("error getting pixels data from Redis: %v", err)
		http.Error(rw, "could not retrieve pixels data", http.StatusInternalServerError)
		return
	}

	rw.Header().Set("Content-Type", "application/json")
	rw.WriteHeader(http.StatusOK)
	rw.Write([]byte(pixelsData))
}

func getAllowedOrigin() string {
	environment := os.Getenv("ENVIRONMENT")
	if environment == "development" {
		return "http://127.0.0.1:5500"
	} else {
		return "https://tenthousandpixels.com"
	}
}

func corsMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", getAllowedOrigin())
		w.Header().Set("Access-Control-Allow-Methods", "GET")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
		next(w, r)
	}
}
