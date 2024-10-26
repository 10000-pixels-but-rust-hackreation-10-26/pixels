# Run locally
1. In `constants.js` set `production` to `false`
2. Serve the `frontend` directory on port `5500`
    1. A simple option is to use `python --version`
<!-- TODO update this to be rust-specific -->
3. Get the backend server running
    1. `docker run -it -p 8080:8080 $(docker build -q .)`
    2. Hehe oops actually the godotenv doesn't work if you do it like that.
4. Run redis with `docker run -d -p 6379:6379  redis/redis-stack-server`


# The `.env` file
The server expects a `.env` file in the `backend` folder. Create one in the following format:

```
REDIS_ADDRESS=localhost:6379/
REDIS_PASSWORD=
ENVIRONMENT=development
HCAPTCHA_SECRET=
```

python3 -m http.server 5500 