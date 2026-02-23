# Lumina 

A High-Performance API Gateway & Reverse Proxy written in Rust.

**Lumina** is designed to be a lightweight, scalable, and memory-safe reverse proxy that routes traffic to backend microservices. Built with Tokio, Axum, and Hyper, it demonstrates modern asynchronous Rust handling real-world traffic patterns efficiently.

##  Key Features

* **High Performance**: Built on `tokio` and `hyper` for blazing-fast asynchronous I/O and near zero-cost abstractions.
* **Declarative Configuration**: Route rules and upstreams are configured via simple YAML.
* **Observability First**: Full structured logging via `tracing` and Prometheus metrics out of the box.
* **Resilient**: Robust error handling mapping upstream failures and latency to proper HTTP responses.
* **Cloud Native**: Shipped with Docker and `docker-compose` ready for Kubernetes deployments.

## 🛠️ Tech Stack

* **Language**: Rust 
* **Web Framework**: [`axum`](https://github.com/tokio-rs/axum)
* **Async Runtime**: [`tokio`](https://tokio.rs/)
* **HTTP Client/Server**: [`hyper`](https://hyper.rs/) & [`reqwest`](https://docs.rs/reqwest)
* **Serialization/Config**: `serde` & `serde_yaml`
* **Observability**: `tracing` & `metrics-exporter-prometheus`
* **Error Handling**: `thiserror` & `anyhow`

##  Getting Started

### Prerequisites

* Rust 1.75+
* Docker & Docker Compose (optional for testing)

### Running Locally

1. **Clone the repo**
   ```bash
   git clone https://github.com/Bhargi777/RUST.git
   cd RUST/lumina
   ```

2. **Configure Routes**
   Edit `config.yaml` to define your host and upstream routing map.
   ```yaml
   server:
     host: "127.0.0.1"
     port: 8080

   routes:
     - path: "payments"
       upstream: "http://localhost:8082"
   ```

3. **Start Lumina**
   ```bash
   cargo run --release
   ```

### Running with Docker

Lumina comes with a `docker-compose` setup that spawns the proxy and a mock python service to test routing immediately.

```bash
docker-compose up --build
```
*   The proxy runs on `http://localhost:8080/`
*   Prometheus metrics endpoint is available on `http://localhost:8080/metrics`
*   Healthcheck is ready on `http://localhost:8080/health`
*   Test proxying by curling `/api/payments/status`:
    ```bash
    curl http://localhost:8080/api/payments/status
    ```

##  Architecture

1. **Client -> Gateway**: Requests arrive at the `axum` router.
2. **Router -> Proxy Handler**: Depending on the prefix (e.g., `/api/payments/*`), the request is matched against the parsed YAML configuration.
3. **Gateway -> Upstream**: Proxies the request using `reqwest`, retaining HTTP method, most headers, and proxying binary bodies as streams.
4. **Upstream -> Gateway -> Client**: Upstream responses stream directly through `hyper` HTTP constructs back to the client, ensuring minimal memory overhead.

##  Application Structure

```text
src/
├── main.rs            # Entry point: tracing setup, args parsing, and server init
├── config.rs          # Loading YAML definitions with serde
├── error.rs           # `thiserror` enumerations converting deeply typed errors to HTTP Status Codes
├── metrics.rs         # Prometheus instrumentation
├── api/               # Standard Gateway endpoints (health, etc.)
└── proxy/             # Core Hyper/reqwest transparent proxy handler
```

##  Future Improvements (For Production)

1. Circuit breakers using `tower`.
2. Rate limiting per IP/Client.
3. OpenTelemetry exporting instead of raw tracing.
4. Hot-reloading of `config.yaml` using a file watcher mechanism.

---
_Designed to demonstrate robust, idiomatic, and systems-level network programming in Rust._
