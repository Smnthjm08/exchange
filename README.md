# exchange

backend for crypto exchange

## workspace

- apps - api and engine
- lib - shared

## setup

1. git clone
2. cp .env.example .env
3. cargo run --bin api
   Apply:

sqlx migrate run

Reset everything:

sqlx database reset

Check status:

sqlx migrate info
