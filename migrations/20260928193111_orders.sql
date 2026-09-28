-- Add migration script here
CREATE TABLE IF NOT EXISTS orders(
        id UUID PRIMARY KEY,
        user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
        market_id UUID NOT NULL REFERENCES markets (id),
        side TEXT NOT NULL CHECK (side IN ('buy', 'sell')),
        type TEXT NOT NULL CHECK (type IN ('limit', 'market')),
        price NUMERIC(38,18) NOT NULL CHECK (price > 0),
        qty NUMERIC(38,18) NOT NULL CHECK (qty > 0),
        filled_qty NUMERIC(38,18) NOT NULL CHECK (filled_qty <= qty AND filled_qty >= 0) DEFAULT 0,
        status TEXT NOT NULL CHECK (status IN ('open', 'filled', 'cancelled')) DEFAULT 'open',
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
)