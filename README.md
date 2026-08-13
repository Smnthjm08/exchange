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

**Phase 0 — Shared types** (slides 75–77)

- [ ] `common`: `Side::{Long,Short}`, `OrderType::{Limit,Market}`, `OrderStatus::{Open,Filled,Cancelled}`
- [ ] `Collateral { available, locked }` — margin moves available → locked on order placement
- [ ] `Order { order_id, market, side, qty, margin, order_type, price, status }`
- [ ] `Position { market, side, qty, margin, average_price, liquidation_price, pnl }`
- [ ] `Fill { maker, taker, market, qty, price, long, short }` — four user refs, not one
- [ ] `Decimal` everywhere for price/qty/margin; never `f64`

**Phase 1 — Price feed** (slide 78)

- [ ] `feed` crate: `tokio-tungstenite` connection to the Binance SOL ticker
- [ ] Deserialize into `common::Tick`, push into an `mpsc` channel
- [ ] Feeds `Orderbook::index_price` — kept distinct from `last_traded_price` (slide 76)
- [ ] `src/bin/probe.rs` to eyeball the stream before the engine exists

**Phase 2 — Orderbook state** (slide 76)

- [ ] Price-level aggregation, not a flat order list:
      `Level { available_qty, open_orders: Vec<OpenOrder> }`
- [ ] `Orderbook { bids: BTreeMap<Decimal, Level>, asks: BTreeMap<Decimal, Level>,
    last_traded_price, index_price }` — `BTreeMap` keeps levels sorted for walking
- [ ] `Orderbooks = HashMap<Market, Orderbook>` — SOL **and** ETH from the start
- [ ] `OpenOrder { user_id, qty, filled_qty, order_id, created_at }`

**Phase 3 — Matching engine** (slides 40–54)

- [ ] `POST /create` locks `equity` from the body as margin, then places long/short
- [ ] Match against the opposite side, walking levels by price using `available_qty`
- [ ] On match → emit `Fill`, update both users' `positions` + `collateral`
- [ ] Closing a position = opening the opposite side for the same qty (slides 44–45)
- [ ] Maintain the zero-sum invariant: open longs == open shorts at all times (slide 53)

**Phase 4 — Liquidation** (slides 55–66)

- [ ] Compute and store `liquidation_price` on the position when it opens (slide 55) —
      the tick loop compares against this instead of recomputing equity per position
- [ ] Background `tokio::time::interval` loop checks open positions against `index_price`
- [ ] On liquidation: market-close by walking the opposite side across levels
      (slide 61–62: `90.20*100 + 90.19*400 + 90.18*1285`), realize the loss
- [ ] Known edge case, deferred (slides 64–66): a thin book can close below the user's
      collateral and drive the balance negative — no insurance fund in V1

**Phase 5 — REST API** (slides 68–74)

- [ ] `POST /create` — body: `{ price, qty, equity, type: "LONG"|"SHORT", market }`
- [ ] `GET /positions/:market` (e.g. `SOL_PERP`)
- [ ] `GET /orders/open`
- [ ] `GET /fills`
- [ ] `GET /orders/:marketId`
- [ ] `GET /positions/closed/:marketId`

**Phase 6 — Persistence** (our addition, not in the deck)

- [ ] `db`: sqlx migrations for `users`, `orders`, `fills`, `closed_positions`
- [ ] Write-behind from the engine — never block matching on a DB round trip
- [ ] Hash passwords (the deck stores `password: 123123` plaintext as a teaching shortcut)

**Explicitly out of scope for V1** (slide 67): stop-loss/take-profit, funding rate,
insurance fund, ADL. In scope: margin and liquidation.
