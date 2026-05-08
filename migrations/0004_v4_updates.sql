-- ============================================================
-- AegisForge v4 Update — Custom Rank Cards & Economy Expansion
-- ============================================================

-- ── Rank Card Customization ──────────────────────────────────
ALTER TABLE users_leveling 
ADD COLUMN IF NOT EXISTS rank_card_background   TEXT    DEFAULT 'default',
ADD COLUMN IF NOT EXISTS rank_card_color        TEXT    DEFAULT '#00E5FF',
ADD COLUMN IF NOT EXISTS rank_card_text_color   TEXT    DEFAULT '#FFFFFF';

-- ── Economy Expansion ───────────────────────────────────────
ALTER TABLE users_economy
ADD COLUMN IF NOT EXISTS last_rob               TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS last_crime             TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS last_fish              TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS last_hunt              TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS total_earned           BIGINT      NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS total_spent            BIGINT      NOT NULL DEFAULT 0;

-- ── Global Statistics (for v4 Dashboard) ──────────────────────
CREATE TABLE IF NOT EXISTS global_stats (
    stat_key    TEXT        PRIMARY KEY,
    stat_value  BIGINT      NOT NULL DEFAULT 0,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO global_stats (stat_key, stat_value) VALUES 
('total_commands_executed', 0),
('total_xp_gained', 0),
('total_economy_transactions', 0)
ON CONFLICT DO NOTHING;
