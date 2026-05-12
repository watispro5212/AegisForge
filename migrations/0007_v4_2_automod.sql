-- ============================================================
-- AegisForge v4.2 - AutoMod Suite & Constraint Fixes
-- ============================================================

-- Fix mod_cases CHECK constraint to include all action types
ALTER TABLE mod_cases DROP CONSTRAINT IF EXISTS valid_action;
ALTER TABLE mod_cases ADD CONSTRAINT valid_action CHECK (action IN (
    'ban', 'unban', 'kick', 'timeout', 'untimeout',
    'warn', 'note', 'mute', 'unmute', 'tempban', 'softban',
    'shadowban', 'shadowunban'
));

-- Add goodbye and logging channel columns if not present
ALTER TABLE guild_configs
    ADD COLUMN IF NOT EXISTS goodbye_channel     BIGINT,
    ADD COLUMN IF NOT EXISTS goodbye_message     TEXT NOT NULL DEFAULT 'Goodbye, {user}. We will miss you.',
    ADD COLUMN IF NOT EXISTS message_log_channel BIGINT,
    ADD COLUMN IF NOT EXISTS member_log_channel  BIGINT;
