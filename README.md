# Perps

A perpetuals exchange in Rust — in-memory orderbook + matching engine, Postgres for
persistence, Binance price feed. Built off the deck in [`docs/phase-01.pdf`](docs/phase-01.pdf),
which holds the full spec and the phase-by-phase breakdown.

Rust surface area this exercises:

- **Ownership & borrowing** — orderbook/position state shared across async tasks
- **`Arc<RwLock<T>>`** — the in-memory orderbook and price state, shared between the ws task, engine, and handlers
- **`tokio`** — task spawning, `select!`, intervals (the liquidation loop)
- **Channels** — `mpsc` (feed → engine) and `broadcast` (engine → ws clients)
- **Error handling** — `Result`, `?`, `thiserror` for typed errors, `anyhow` for glue
- **`rust_decimal`** — never `f64` for money/price/qty

## Tech stack

| Layer                  | Choice                                   |
| ---------------------- | ---------------------------------------- |
| Web framework          | `axum`                                   |
| Async runtime          | `tokio`                                  |
| DB access              | `sqlx` (async, compile-time checked SQL) |
| DB                     | Postgres                                 |
| WS client (price feed) | `tokio-tungstenite`                      |
| Serialization          | `serde`, `serde_json`                    |
| Decimal math           | `rust_decimal`                           |
| IDs                    | `uuid`                                   |
| Password hashing       | `argon2`                                 |
| Auth tokens            | `jsonwebtoken` (HS256)                   |
| Migrations             | `sqlx-cli`                               |

## Workspace structure

```text
perp-v1/
├── crates/
│   ├── api/       # axum HTTP server, route handlers
│   ├── engine/    # matching engine + liquidation loop (in-memory)
│   ├── feed/      # binance/backpack ws price feed
│   ├── db/        # sqlx models + queries
│   └── common/    # shared types: Order, Position, Fill, Market
├── migrations/
└── .env
```

- `common` holds the core structs so every crate shares one source of truth
- `engine` owns the live orderbook + positions, and is the only thing allowed to mutate them
- `api` calls into `engine` and `db` — no business logic in handlers
- `feed` pushes price ticks into a channel that `engine` consumes

## Status

- [x] Shared types
- [x] DB pool, `users` migration, user queries
- [x] Auth — argon2 + JWT
- [ ] Price feed *(next)*
- [ ] Orderbook
- [ ] Matching engine
- [ ] Liquidation
- [ ] Trading REST routes
- [ ] Auth middleware

## API

Everything is nested under `/api/v1`. Success bodies share one envelope:
`{ message, success, data }`. Errors are currently a bare plaintext string + status code.

| Route                | Success | Errors                                       |
| -------------------- | ------- | -------------------------------------------- |
| `GET /health`        | `200`   | —                                            |
| `POST /auth/signup`  | `201`   | `409` email taken                            |
| `POST /auth/login`   | `200`   | `401` unknown email or bad password          |

```jsonc
// POST /auth/signup  { "username": "sam", "email": "sam@example.com", "password": "hunter2" }
// POST /auth/login   { "email": "sam@example.com", "password": "hunter2" }
{
  "message": "user created successfully",
  "success": true,
  "data": { "access_token": "<jwt>", "id": "<uuid>", "username": "sam", "email": "sam@example.com" }
}
```

Rough edges:

- error responses don't match the JSON envelope
- a non-PHC `password` row fails login with `500`, not `401`
- `encode_jwt`/`decode_jwt` `unwrap()` a missing `JWT_SECRET`
- no rate limiting

## Running it

```bash
cp .env.example .env    # DATABASE_URL, JWT_SECRET
cargo run -p api        # runs migrations on boot, binds 0.0.0.0:3000
```

`sqlx::query!` is compile-time checked, so `DATABASE_URL` must point at a reachable database
for `cargo check` to pass.
