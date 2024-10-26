# Run locally
1. In `constants.js` set `production` to `false`
2. Serve the `frontend` directory on port `5500`
    1. A simple option is to use `python -m http.server 5500`
<!-- TODO update this to be rust-specific -->
3. Get the backend server running
    1. `docker run -it -p 8080:8080 $(docker build -q .)`
4. Something something redis for data storage???