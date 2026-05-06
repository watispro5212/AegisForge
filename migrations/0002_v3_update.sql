-- ============================================================
-- AegisForge v3 Update — Economy, Leveling, and Advanced Automod
-- ============================================================

-- ── Economy System ───────────────────────────────────────────
CREATE TABLE IF NOT EXISTS users_economy (
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    user_id         BIGINT      NOT NULL,
    balance         BIGINT      NOT NULL DEFAULT 0,
    bank            BIGINT      NOT NULL DEFAULT 0,
    last_daily      TIMESTAMPTZ,
    last_work       TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (guild_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_economy_guild_balance ON users_economy (guild_id, balance DESC);

-- ── Leveling System ──────────────────────────────────────────
CREATE TABLE IF NOT EXISTS users_leveling (
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    user_id         BIGINT      NOT NULL,
    xp              BIGINT      NOT NULL DEFAULT 0,
    level           INT         NOT NULL DEFAULT 0,
    last_msg        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (guild_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_leveling_guild_xp ON users_leveling (guild_id, xp DESC);

-- ── Level Roles ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS level_roles (
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    level           INT         NOT NULL,
    role_id         BIGINT      NOT NULL,
    PRIMARY KEY (guild_id, level)
);

-- ── Advanced Automod Blacklist ───────────────────────────────
CREATE TABLE IF NOT EXISTS automod_blacklist (
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    phrase          TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (guild_id, phrase)
);

-- ── Update Trigger for new tables ────────────────────────────
CREATE OR REPLACE TRIGGER users_economy_updated_at
    BEFORE UPDATE ON users_economy
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE TRIGGER users_leveling_updated_at
    BEFORE UPDATE ON users_leveling
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ── Add Leveling/Economy toggles to guild_configs ────────────
ALTER TABLE guild_configs 
ADD COLUMN IF NOT EXISTS economy_enabled BOOLEAN NOT NULL DEFAULT TRUE,
ADD COLUMN IF NOT EXISTS leveling_enabled BOOLEAN NOT NULL DEFAULT TRUE,
ADD COLUMN IF NOT EXISTS level_up_message TEXT NOT NULL DEFAULT 'GG {user}, you reached level {level}! 🚀';
