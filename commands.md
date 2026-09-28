# Commands

## Setup

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
docker compose up -d                  # postgres on localhost:5434
cp .env.example .env                  # DATABASE_URL=postgres://postgres:postgres@localhost:5434/cex_db
```

`sqlx` reads `DATABASE_URL` from `.env`. Every command below needs it.

## Migrations

```bash
sqlx migrate add <name>               # new migrations/<timestamp>_<name>.sql
sqlx migrate add -r <name>            # reversible: <name>.up.sql + <name>.down.sql
sqlx migrate run                      # apply pending migrations
sqlx migrate info                     # list applied / pending
sqlx migrate revert                   # undo last migration (reversible ones only)
```

The `api` binary also runs pending migrations on boot.

## Database

```bash
sqlx database create                  # create the DB from DATABASE_URL
sqlx database drop                    # drop it
sqlx database reset                   # drop + create + run all migrations
sqlx database reset -y                # same, no confirmation prompt
```

Changed an already-applied migration? The checksum won't match, so run `sqlx database reset`.

## Seed

```bash
docker exec -i cex_db psql -U postgres -d cex_db < seeds/markets.sql   # SOL, USDT assets + SOL_USDT market
```

Safe to re-run (`ON CONFLICT DO NOTHING`). Run after migrations, including after every `reset`.

## Nuke everything

```bash
docker compose down -v                # stop postgres and delete its volume
docker compose up -d && sqlx database reset -y
```

## Run / check

```bash
cargo run -p api                      # binds 0.0.0.0:3000
cargo check                           # needs a reachable DB (query! is compile-time checked)
cargo sqlx prepare --workspace        # cache query metadata in .sqlx/ for offline builds
```
