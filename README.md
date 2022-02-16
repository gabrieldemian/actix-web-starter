# Actix Web Boilerplate
Simple boilerplate with very few dependencies.

## Features
- Good error handling with correct status codes
- Responses in JSON format
- Easy CORS configuration, header already set up
- Modular routing
- The tests uses the real HTTP server handlers (functions/endpoints)

## Building && Running
```
cargo build
cargo watch -x run
```

## Testing
Open your terminal and paste the command:
```
curl -v --url localhost:8080/gabriel
```
