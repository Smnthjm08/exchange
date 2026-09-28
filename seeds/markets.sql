-- Idempotent: safe to re-run. Assets first, markets reference them.
INSERT INTO assets (id, name, symbol, decimals) VALUES
    (gen_random_uuid(), 'Solana', 'SOL', 9),
    (gen_random_uuid(), 'Tether USD', 'USDT', 6)
ON CONFLICT (symbol) DO NOTHING;

INSERT INTO markets (id, symbol, base_asset_id, quote_asset_id, tick_size, min_qty)
SELECT gen_random_uuid(), 'SOL_USDT', b.id, q.id, 0.01, 0.001
FROM assets b, assets q
WHERE b.symbol = 'SOL' AND q.symbol = 'USDT'
ON CONFLICT (symbol) DO NOTHING;
