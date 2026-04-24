# LBRS (Load Balancer Rust System)

A simple, fast, round-robin TCP load balancer written in Rust.

## Features
- **Round-Robin Balancing**: Distributes connections evenly across all healthy backend servers.
- **Health Monitoring**: Periodically pings backends (every 5s) and routes traffic only to healthy ones.
- **Asynchronous & Non-Blocking**: Built on `tokio` for high-throughput and minimal overhead.
- **Dynamic Configuration**: Supports passing backend servers via CLI arguments.

## Usage
```bash
cargo run -- --port 127.0.0.1:8080 --backends 127.0.0.1:8081 127.0.0.1:8082
```