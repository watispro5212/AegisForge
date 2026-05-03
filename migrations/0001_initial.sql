-- ============================================================
-- AegisForge — Initial PostgreSQL Schema
-- Designed to serve hundreds of thousands of guilds.
--
-- Design principles:
--   · Discord IDs (snowflakes) stored as BIGINT (i64 in Rust)
--   · All timestamps are TIMESTAMPTZ (UTC)
--   · Indexes on every FK and every column used in WHERE clauses
--   · Partial indexes used where data is sparse (e.g. active=true)
--   · BRIN indexes on append-only timestamp columns at scale
--   · Case numbers are per-guild, managed with a counter table
-- ============================================================

-- ── Guild configuration ─────────────────────────────────────
-- One row per guild. Inserted lazily on first bot interaction.
CREATE TABLE IF NOT EXISTS guild_configs (
    guild_id            BIGINT      PRIMARY KEY,
    prefix              TEXT        NOT NULL DEFAULT '!',

    -- Logging
    mod_log_channel     BIGINT,
    message_log_channel BIGINT,
    member_log_channel  BIGINT,

    -- Welcome / goodbye
    welcome_channel     BIGINT,
    welcome_message     TEXT        NOT NULL DEFAULT 'Welcome to the server, {user}! 🎉',
    goodbye_channel     BIGINT,
    goodbye_message     TEXT        NOT NULL DEFAULT 'Goodbye, {user}. We will miss you.',

    -- Auto-role on join
    auto_role_id        BIGINT,

    -- Mute role (for manual mutes)
    mute_role_id        BIGINT,

    -- Feature toggles
    automod_enabled     BOOLEAN     NOT NULL DEFAULT FALSE,
    automod_spam        BOOLEAN     NOT NULL DEFAULT FALSE,
    automod_invites     BOOLEAN     NOT NULL DEFAULT FALSE,
    automod_caps        BOOLEAN     NOT NULL DEFAULT FALSE,
    automod_mentions    BOOLEAN     NOT NULL DEFAULT FALSE,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ── Per-guild case number counter ───────────────────────────
-- Allows atomic case number generation without table scans.
CREATE TABLE IF NOT EXISTS guild_case_counters (
    guild_id    BIGINT  PRIMARY KEY REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    next_case   INT     NOT NULL DEFAULT 1
);

-- ── Moderation cases ────────────────────────────────────────
CREATE TABLE IF NOT EXISTS mod_cases (
    id              BIGSERIAL   PRIMARY KEY,
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    case_number     INT         NOT NULL,   -- per-guild sequential number
    target_id       BIGINT      NOT NULL,   -- the user being actioned
    moderator_id    BIGINT      NOT NULL,
    action          TEXT        NOT NULL,
    reason          TEXT,
    duration_secs   BIGINT,                 -- for timed actions (timeout/tempban)
    expires_at      TIMESTAMPTZ,
    active          BOOLEAN     NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_action CHECK (action IN (
        'ban', 'unban', 'kick', 'timeout', 'untimeout',
        'warn', 'note', 'mute', 'unmute', 'tempban'
    )),
    -- Case numbers must be unique within a guild
    CONSTRAINT unique_guild_case UNIQUE (guild_id, case_number)
);

CREATE INDEX IF NOT EXISTS idx_mod_cases_guild          ON mod_cases (guild_id);
CREATE INDEX IF NOT EXISTS idx_mod_cases_target         ON mod_cases (guild_id, target_id);
CREATE INDEX IF NOT EXISTS idx_mod_cases_moderator      ON mod_cases (guild_id, moderator_id);
-- Partial index — only active cases need expiry polling
CREATE INDEX IF NOT EXISTS idx_mod_cases_expiry         ON mod_cases (expires_at) WHERE active = TRUE;
-- BRIN is efficient for append-heavy timestamp columns
CREATE INDEX IF NOT EXISTS idx_mod_cases_created_brin   ON mod_cases USING BRIN (created_at);

-- ── Warnings (lightweight; also logged as mod_cases) ────────
CREATE TABLE IF NOT EXISTS warnings (
    id              BIGSERIAL   PRIMARY KEY,
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    user_id         BIGINT      NOT NULL,
    moderator_id    BIGINT      NOT NULL,
    reason          TEXT        NOT NULL,
    case_id         BIGINT      REFERENCES mod_cases(id) ON DELETE SET NULL,
    pardoned        BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_warnings_guild_user  ON warnings (guild_id, user_id);
CREATE INDEX IF NOT EXISTS idx_warnings_guild       ON warnings (guild_id);

-- ── Warning escalation thresholds ───────────────────────────
-- Defines what happens when a user reaches N warnings in a guild.
CREATE TABLE IF NOT EXISTS warn_thresholds (
    guild_id        BIGINT  NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    warn_count      INT     NOT NULL,
    action          TEXT    NOT NULL,
    duration_secs   BIGINT,
    PRIMARY KEY (guild_id, warn_count),
    CONSTRAINT valid_threshold_action CHECK (action IN ('timeout', 'kick', 'ban', 'tempban'))
);

-- ── Temporary punishments (active tracking for expiry worker) ─
CREATE TABLE IF NOT EXISTS temp_punishments (
    id          BIGSERIAL   PRIMARY KEY,
    guild_id    BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    user_id     BIGINT      NOT NULL,
    type        TEXT        NOT NULL,
    expires_at  TIMESTAMPTZ NOT NULL,
    active      BOOLEAN     NOT NULL DEFAULT TRUE,
    case_id     BIGINT      REFERENCES mod_cases(id) ON DELETE SET NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT valid_punishment_type CHECK (type IN ('ban', 'mute', 'timeout', 'tempban'))
);

CREATE INDEX IF NOT EXISTS idx_temp_punishments_expiry      ON temp_punishments (expires_at) WHERE active = TRUE;
CREATE INDEX IF NOT EXISTS idx_temp_punishments_guild_user  ON temp_punishments (guild_id, user_id);

-- ── Auto-roles ───────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS auto_roles (
    guild_id    BIGINT  NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    role_id     BIGINT  NOT NULL,
    PRIMARY KEY (guild_id, role_id)
);

CREATE INDEX IF NOT EXISTS idx_auto_roles_guild ON auto_roles (guild_id);

-- ── Reaction roles ───────────────────────────────────────────
CREATE TABLE IF NOT EXISTS reaction_roles (
    id          BIGSERIAL   PRIMARY KEY,
    guild_id    BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    channel_id  BIGINT      NOT NULL,
    message_id  BIGINT      NOT NULL,
    emoji       TEXT        NOT NULL,   -- e.g. "✅" or "custom:123456"
    role_id     BIGINT      NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_reaction_role UNIQUE (guild_id, message_id, emoji)
);

CREATE INDEX IF NOT EXISTS idx_reaction_roles_message   ON reaction_roles (message_id);
CREATE INDEX IF NOT EXISTS idx_reaction_roles_guild     ON reaction_roles (guild_id);

-- ── Reminders ────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS reminders (
    id          BIGSERIAL   PRIMARY KEY,
    user_id     BIGINT      NOT NULL,
    channel_id  BIGINT      NOT NULL,
    guild_id    BIGINT,
    message     TEXT        NOT NULL,
    fire_at     TIMESTAMPTZ NOT NULL,
    fired       BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Partial index for the expiry worker — only rows that need firing
CREATE INDEX IF NOT EXISTS idx_reminders_pending    ON reminders (fire_at) WHERE fired = FALSE;
CREATE INDEX IF NOT EXISTS idx_reminders_user       ON reminders (user_id);

-- ── User notes (admin-only, not infractions) ─────────────────
CREATE TABLE IF NOT EXISTS user_notes (
    id              BIGSERIAL   PRIMARY KEY,
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    user_id         BIGINT      NOT NULL,
    moderator_id    BIGINT      NOT NULL,
    content         TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_notes_guild_user ON user_notes (guild_id, user_id);

-- ── Welcome/goodbye message history (optional audit log) ─────
CREATE TABLE IF NOT EXISTS member_events (
    id          BIGSERIAL   PRIMARY KEY,
    guild_id    BIGINT      NOT NULL,
    user_id     BIGINT      NOT NULL,
    event       TEXT        NOT NULL,   -- 'join' | 'leave'
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT valid_event CHECK (event IN ('join', 'leave'))
);

CREATE INDEX IF NOT EXISTS idx_member_events_guild      ON member_events (guild_id);
-- BRIN index for efficient range scans on time-series data
CREATE INDEX IF NOT EXISTS idx_member_events_time_brin  ON member_events USING BRIN (created_at);

-- ── Trigger: auto-update updated_at on guild_configs ─────────
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE OR REPLACE TRIGGER guild_configs_updated_at
    BEFORE UPDATE ON guild_configs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
