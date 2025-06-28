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

## Performance

This is a very lightweight API.  Results below use `oha` to make 1 million requests.  95% of responses are served in < 0.34ms.

```bash
oha -n 1000000 http://127.0.0.1:8080

Summary:
  Success rate: 100.00%
  Total:        4585.9340 ms
  Slowest:      2.3127 ms
  Fastest:      0.0116 ms
  Average:      0.2277 ms
  Requests/sec: 218058.0900

  Total data:   20.98 MiB
  Size/request: 22 B
  Size/sec:     4.58 MiB

Response time histogram:
  0.012 ms [1]      |
  0.242 ms [658522] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.472 ms [331711] |■■■■■■■■■■■■■■■■
  0.702 ms [6069]   |
  0.932 ms [948]    |
  1.162 ms [514]    |
  1.392 ms [703]    |
  1.622 ms [1396]   |
  1.852 ms [131]    |
  2.083 ms [0]      |
  2.313 ms [5]      |

Response time distribution:
  10.00% in 0.1507 ms
  25.00% in 0.1831 ms
  50.00% in 0.2189 ms
  75.00% in 0.2581 ms
  90.00% in 0.3023 ms
  95.00% in 0.3360 ms
  99.00% in 0.4675 ms
  99.90% in 1.4838 ms
  99.99% in 1.6380 ms


Details (average, fastest, slowest):
  DNS+dialup:   0.5411 ms, 0.3722 ms, 0.6232 ms
  DNS-lookup:   0.0093 ms, 0.0004 ms, 0.0962 ms

Status code distribution:
  [200] 1000000 responses
```