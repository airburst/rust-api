# Rust API

A simple REST API server built with Actix Web that returns JSON data.

## Running the application

```bash
cargo run
```

The server will start on configured host:port, initially `http://127.0.0.1:8080`

## API Endpoints

- `GET /` - Returns JSON: `{ "data": "hello world" }`
- `GET /health` - Returns string: `"OK"`

## Dependencies

- `actix-web` - Web framework for Rust
- `tokio` - Async runtime
- `serde` - Serialization framework
- `serde_json` - JSON support for serde
