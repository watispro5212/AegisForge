-- ============================================================
-- AegisForge v4.1 - Global Shop Inventory
-- ============================================================

CREATE TABLE IF NOT EXISTS economy_inventory (
    guild_id           BIGINT      NOT NULL,
    user_id            BIGINT      NOT NULL,
    item_id            TEXT        NOT NULL,
    item_name          TEXT        NOT NULL,
    quantity           BIGINT      NOT NULL DEFAULT 1,
    first_purchased_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_purchased_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (guild_id, user_id, item_id),
    FOREIGN KEY (guild_id, user_id)
        REFERENCES users_economy(guild_id, user_id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_economy_inventory_user
    ON economy_inventory (user_id, guild_id);

CREATE INDEX IF NOT EXISTS idx_economy_inventory_item
    ON economy_inventory (item_id);
