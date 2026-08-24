-- Add migration script here
CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    symbol VARCHAR(10) NOT NULL,
    decimals INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    UNIQUE (symbol)
);

CREATE TABLE IF NOT EXISTS user_assets (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    asset_id UUID NOT NULL REFERENCES assets (id) ON DELETE CASCADE,
    available NUMERIC(38, 18) NOT NULL DEFAULT 0 CHECK (available >= 0),
    locked NUMERIC(38, 18) NOT NULL DEFAULT 0 CHECK (locked >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    UNIQUE (user_id, asset_id)
);