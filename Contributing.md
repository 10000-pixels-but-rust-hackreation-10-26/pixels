# Run locally
1. In `constants.js` set `production` to `false`
2. Serve the `frontend` directory on port `5500`
    1. A simple option is to use `python -m http.server 5500`
    2. Access the server with `http://127.0.0.1:5500`
<!-- TODO update this to be rust-specific -->
3. Run redis with `docker run -d -p 6379:6379  redis/redis-stack-server`
4. Get the backend server running
    1. `go run .`


# The `.env` file
The server expects a `.env` file in the `backend` folder. Create one in the following format:

```
REDIS_ADDRESS=localhost:6379
REDIS_PASSWORD=
ENVIRONMENT=development
HCAPTCHA_SECRET=
```

python3 -m http.server 5500 