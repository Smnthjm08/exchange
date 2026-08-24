-- Add migration script here
CREATE TABLE
    IF NOT EXISTS user_balances (
        id UUID PRIMARY KEY,
        user_id UUID NOT NULL UNIQUE REFERENCES users (id) ON DELETE CASCADE,
        -- usdc only for now
        available NUMERIC(38, 18) NOT NULL DEFAULT 0 CHECK (available >= 0),
        locked NUMERIC(38, 18) NOT NULL DEFAULT 0 CHECK (locked >= 0),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );