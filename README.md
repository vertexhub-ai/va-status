# va-status

Status Page SaaS backend built with Rust, Axum, and PostgreSQL.

## Features
- REST API with Axum
- Database integration with SQLx
- Production-ready error handling and logging
- Configuration management with `config` crate
- Dockerized for easy deployment

## Getting Started

### Prerequisites
- Rust (latest stable)
- PostgreSQL
- `sqlx-cli` (optional, for migrations)

### Setup
1. Create a `.env` file based on `.env.example`.
2. Run migrations:
   ```bash
   sqlx migrate run
   ```
3. Start the server:
   ```bash
   cargo run
   ```

### Testing
```bash
cargo test
```

### API Endpoints
- `GET /health_check`: Check API health.
- `GET /services`: List all services and their statuses.