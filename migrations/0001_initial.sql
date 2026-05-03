CREATE TABLE IF NOT EXISTS guild_config (
    guild_id         INTEGER PRIMARY KEY,
    mod_log_channel  INTEGER,
    welcome_channel  INTEGER,
    welcome_message  TEXT    DEFAULT 'Welcome, {user}! 🎉',
    auto_role_id     INTEGER,
    prefix           TEXT    DEFAULT '!'
);

CREATE TABLE IF NOT EXISTS mod_cases (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id     INTEGER NOT NULL,
    user_id      INTEGER NOT NULL,
    moderator_id INTEGER NOT NULL,
    action       TEXT    NOT NULL,   -- ban, kick, timeout, warn
    reason       TEXT,
    created_at   TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS reminders (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id    INTEGER NOT NULL,
    channel_id INTEGER NOT NULL,
    message    TEXT    NOT NULL,
    fire_at    TEXT    NOT NULL,
    fired      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS auto_roles (
    guild_id INTEGER NOT NULL,
    role_id  INTEGER NOT NULL,
    PRIMARY KEY (guild_id, role_id)
);
