-- ============================================================
-- AegisForge v4.2 - Shadow Ban System
-- ============================================================

CREATE TABLE IF NOT EXISTS shadow_bans (
    guild_id     BIGINT      NOT NULL,
    user_id      BIGINT      NOT NULL,
    moderator_id BIGINT      NOT NULL,
    reason       TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (guild_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_shadow_bans_guild
    ON shadow_bans (guild_id);
