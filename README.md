# Perps

- **Ownership & borrowing** — you'll share the orderbook/positions state across async tasks, so `&`, `&mut`, and move semantics need to be second nature
- **`Arc<RwLock<T>>` / `Arc<Mutex<T>>`** — for the in-memory orderbook and price state shared between the websocket task, matching engine, and API handlers
- **`tokio`** — async runtime: spawning tasks, `select!`, intervals (for the liquidation-check loop)
- **Channels** — `tokio::sync::mpsc` (price feed → engine) and `broadcast` (engine → multiple listeners, e.g. websocket clients later)
- **Traits & generics** — lightly, for abstracting exchange price sources if you support more than Binance later
- **Error handling** — `Result`, `?`, `thiserror` for typed errors, `anyhow` for glue code
- **`serde`** — serializing structs to JSON for the API and to/from Postgres rows
- **`rust_decimal`** — never use `f64` for money/price/qty; use `Decimal`

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
| Logging                | `tracing` + `tracing-subscriber`         |
| Config                 | `dotenvy`                                |
| Migrations             | `sqlx-cli` migrations                    |

## Workspace structure

```text
perp-v1/
├── Cargo.toml                # workspace root
├── crates/
│   ├── api/                  # axum HTTP server, route handlers
│   ├── engine/                # matching engine + liquidation loop (in-memory)
│   ├── feed/                  # binance/backpack ws price feed
│   ├── db/                    # sqlx models, queries, migrations
│   └── common/                 # shared types: Order, Position, Fill, Market
├── migrations/
└── .env
```

- `common` holds the core structs (`Order`, `Position`, `Fill`, `Side::{Long,Short}`) so every crate shares one source of truth
- `engine` owns the live orderbook + positions in memory (`Arc<RwLock<...>>`), and is the only thing allowed to mutate them
- `api` just calls into `engine` and `db` — no business logic in handlers
- `feed` pushes price ticks into a channel that `engine` consumes

## V1 task breakdown (mapped to slides)

**Phase 1 — Price feed** (foundation for everything else)

- [ ] `feed` crate: `tokio-tungstenite` connection to Binance SOL ticker
- [ ] Push ticks into an `mpsc` channel or shared `Arc<RwLock<Decimal>>`

**Phase 2 — Schema & DB layer** (slides 62–64)

- [ ] `users`: id, equity/balance
- [ ] `orders`: id, user_id, market, side, price, qty, margin, status (open/filled/closed)
- [ ] `fills`: id, order_id, price, qty, timestamp
- [ ] `positions`: derived from fills — market, side, qty, entry price, margin, status (open/closed/liquidated)
- [ ] sqlx migrations for all four

**Phase 3 — Matching engine** (slides 26–41)

- [ ] In-memory orderbook (`BTreeMap<Price, Vec<Order>>` per side)
- [ ] `POST /create` places long/short, matches against opposite side if available
- [ ] On match → write `fill`, update both users' `position` + `equity`

**Phase 4 — Live equity & liquidation** (slides 42–53)

- [ ] On every price tick, recompute equity per open position: `equity = margin + (current_price - entry_price) * qty * direction`
- [ ] Background `tokio::time::interval` loop checks all open positions for equity ≈ 0
- [ ] On liquidation: market-close by walking the opposite side of the book (slide 48–49's fill-across-levels logic), realize the loss

**Phase 5 — REST API** (slides 55–60)

- [ ] `POST /create`
- [ ] `GET /positions/:market`
- [ ] `GET /orders/open`
- [ ] `GET /fills`
- [ ] `GET /orders/:marketId`
- [ ] `GET /positions/closed/:marketId`

**Explicitly out of scope for V1** (slide 54): stop-loss/take-profit, funding rate, insurance fund, ADL.

Want me to scaffold this as an actual `cargo new` workspace with the crates and starter `Cargo.toml`/`main.rs` files so you have something running end to end?
