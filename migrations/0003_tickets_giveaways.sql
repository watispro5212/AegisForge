-- ============================================================
-- AegisForge v3.1 Update — Tickets, Giveaways, and Extended Integrations
-- ============================================================

-- ── Tickets System ───────────────────────────────────────────
CREATE TABLE IF NOT EXISTS tickets_config (
    guild_id        BIGINT      NOT NULL PRIMARY KEY REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    channel_id      BIGINT      NOT NULL,
    category_id     BIGINT,
    staff_role_id   BIGINT,
    message_title   TEXT        NOT NULL DEFAULT 'Support Ticket',
    message_desc    TEXT        NOT NULL DEFAULT 'Click the button below to open a support ticket.',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tickets (
    ticket_id       UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    channel_id      BIGINT      NOT NULL,
    creator_id      BIGINT      NOT NULL,
    status          TEXT        NOT NULL DEFAULT 'open', -- open, closed, archived
    reason          TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at       TIMESTAMPTZ
);

-- ── Giveaways System ──────────────────────────────────────────
CREATE TABLE IF NOT EXISTS giveaways (
    message_id      BIGINT      PRIMARY KEY,
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    channel_id      BIGINT      NOT NULL,
    host_id         BIGINT      NOT NULL,
    winners_count   INT         NOT NULL DEFAULT 1,
    prize           TEXT        NOT NULL,
    ends_at         TIMESTAMPTZ NOT NULL,
    status          TEXT        NOT NULL DEFAULT 'active', -- active, ended, rerolled
    winners_list    BIGINT[]    -- Array of user IDs who won
);

-- ── Role Reaction Menus ───────────────────────────────────────
CREATE TABLE IF NOT EXISTS reaction_roles (
    message_id      BIGINT      NOT NULL,
    guild_id        BIGINT      NOT NULL REFERENCES guild_configs(guild_id) ON DELETE CASCADE,
    emoji           TEXT        NOT NULL,
    role_id         BIGINT      NOT NULL,
    PRIMARY KEY (message_id, emoji)
);
