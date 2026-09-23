# Roadmap — Spot CEX

**Goal:** a working spot exchange — deposit, place limit/market orders, match them in an
in-memory orderbook, settle trades against per-asset balances. Perps come after, reusing
the same book, matching, and balance-locking primitives.

Supersedes the perps-first plan in [`phase-01.pdf`](phase-01.pdf) and the status table in
the README. Deposit/onramp detail lives in [`balances.md`](balances.md).

---

## Done

- [X] Workspace: `api` / `db` / `common` / `engine` / `feed`, axum + tokio
- [X] Postgres pool, migrations run on boot
- [X] `users` table + queries (get by email, get by id, create)
- [X] Auth — argon2 hashing, HS256 JWT, signup + login
- [X] `AuthUser` extractor (`FromRequestParts`) — protected routes work
- [X] `GET /user/profile`
- [X] `assets` + `user_assets` schema (per-asset `available` / `locked`)
- [X] `GET /user/balances` handler *(staged, uncommitted — reads `user_assets`, returns a list of
  per-asset balances)*
- [X] Dropped `user_balances` table + migration — `user_assets` is the only balance store now

---

## Open decisions — settle before writing more

- [X] **`user_balances` vs `user_assets`.** Resolved — collapsed onto `user_assets`,
  USDC is just another asset row. `user_balances` table, migration, and query layer removed.
- [ ] **Ledger vs. mutable balances.** Trades, fees, and deposits all mutate the same rows.
  Without an append-only entry log you can't answer "why is my balance this number."
  Cheapest to add before the engine starts writing.
- [X] **`Side::Long/Short` → `Buy/Sell`.** Long/short is a position concept; spot has no
  positions. Rename now or carry both?
- [ ] **Order rejection on insufficient balance** — reject at the API boundary, or let the
  engine reject and return async?

---

## M1 — Schema foundation

- [ ] `markets` table: base asset, quote asset, tick size, min qty, status
- [ ] `orders` table: no `margin` column — spot orders are `(market, side, type, price, qty, filled_qty, status)`
- [ ] `trades` table: maker order, taker order, price, qty, fee, timestamp
- [ ] Resolve the balances decision above; migrate accordingly
- [ ] `updated_at` triggers — `DEFAULT NOW()` only fires on insert
- [ ] Align `users` timestamps to `TIMESTAMPTZ` (rest of the schema already is)
- [ ] Seed assets (USDC, BTC, ETH) and at least one market

## M2 — Balances API

- [X] `db` query layer for `user_assets` (`get_user_assets_by_user_id`) — `assets` registry
  query layer still missing
- [X] Return balances as a **list keyed by asset**, not a fixed-shape object
- [ ] Lazy row creation on first credit (upsert) — keeps signup non-transactional
- [ ] `GET /markets` and `GET /assets` — clients need the registry before they can order

## M3 — Order placement + balance locking

- [ ] `POST /orders` — validate against market tick/min qty
- [ ] Lock on placement: buy locks quote, sell locks base (`available` → `locked`, one txn)
- [ ] `DELETE /orders/:id` — cancel, unlock the remainder
- [ ] `GET /orders` (open) and `GET /orders/history`
- [ ] Reject when `available` is short — and make sure the CHECK constraint is the backstop,
  not the primary guard

## M4 — Orderbook + matching engine

- [ ] Replace the `cargo new` stub in `crates/engine`
- [ ] Price-level book: `BTreeMap<Decimal, VecDeque<Order>>`, bids descending / asks ascending
- [ ] Price-time priority matching
- [ ] Limit orders — rest on the book when unmatched
- [ ] Market orders — walk the book, handle partial fills and empty-book
- [ ] `Arc<RwLock<T>>` around the book; `mpsc` in from the API, `broadcast` out
- [ ] Load open orders from Postgres on boot — the book is in-memory and must rebuild
- [ ] Unit tests: partial fill, self-trade, crossing spread, empty book

## M5 — Trade settlement

- [ ] On fill: debit `locked`, credit counterparty `available`, both sides, one txn
- [ ] Persist the trade row in the same transaction as the balance move
- [ ] Fee model — flat maker/taker bps to start; decide where fees accrue
- [ ] Update order `filled_qty` / `status` atomically with the fill

## M6 — Market data

- [ ] `GET /markets/:symbol/depth` — aggregated book snapshot
- [ ] `GET /markets/:symbol/trades` — recent trades
- [ ] WS endpoint: depth + trade stream, fed by the engine's `broadcast` channel
- [ ] `feed` crate — **not needed for spot.** Price comes from your own book. Only wire
  Binance in if you want seeded liquidity or a reference price.

## M7 — Onramp / deposits

- [ ] Mock credit endpoint behind an env flag, shaped like a real provider's interface
- [ ] `deposits` table with a **unique `external_ref`** — the replay guard; the single most
  important constraint in the whole onramp
- [ ] Status as an enum (`pending` / `confirmed` / `failed`), not a boolean
- [ ] Credit only on transition to `confirmed`, status + balance in one transaction
- [ ] Idempotent end-to-end — a re-delivered `external_ref` is a no-op success
- [ ] `GET /deposits` history
- [ ] Withdrawals — the mirror, and the one carrying real risk. Pending state + approval step.

---

## Hardening (roll in as you go, don't batch to the end)

- [ ] Error responses match the `{ message, success, data }` envelope
- [ ] A non-PHC `password` row returns `401`, not `500`
- [ ] `encode_jwt` / `decode_jwt` stop `unwrap()`ing a missing `JWT_SECRET` — read it once at boot
- [ ] Rate limiting on auth routes
- [ ] `thiserror` types per crate instead of stringly-typed errors
- [ ] Structured request logging (`tower-http` trace layer)
- [ ] Integration tests against a throwaway Postgres

---

## Phase 2 — Perps

Deferred. Already in `crates/common` as unused surface — leave it, don't build on it:
`Position`, `Collateral`, `Order.margin`, `Position.liquidation_price`, `Side::Long/Short`.

- [ ] Mark price + the `feed` crate (mandatory here, unlike spot)
- [ ] Margin accounting on top of `user_assets`
- [ ] Positions: open, average price, unrealized PnL
- [ ] Funding rate loop
- [ ] Liquidation loop (`tokio::select!` + interval)
