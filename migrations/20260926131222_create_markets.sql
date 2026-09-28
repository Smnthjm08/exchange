-- Add migration script here
CREATE TABLE IF NOT EXISTS markets (
    id UUID PRIMARY KEY,
    symbol VARCHAR(32) NOT NULL UNIQUE,
    base_asset_id UUID NOT NULL REFERENCES assets (id),
    quote_asset_id UUID NOT NULL REFERENCES assets (id),
    tick_size NUMERIC(38, 18) NOT NULL CHECK (tick_size > 0),
    min_qty NUMERIC(38, 18) NOT NULL CHECK (min_qty > 0),
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'halted')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    UNIQUE (base_asset_id, quote_asset_id),
    CHECK (base_asset_id <> quote_asset_id)
);
