# ⚙ AegisForge

![AegisForge Banner](web/assets/banner.png)

<p align="center">
  <img src="web/assets/logo.jpg" alt="AegisForge Logo" width="128" height="128" style="border-radius: 50%">
</p>

> A fast, secure, and customizable Rust-powered Discord bot for moderation, automation, and server utilities. Built for performance, designed for precision.

---

## Features

| Category | Commands |
| :--- | :--- |
| **Moderation** | `/mod ban` `/mod unban` `/mod kick` `/mod timeout` `/mod warn` `/mod purge` |
| **Economy** | `/economy balance` `/economy work` `/economy daily` `/economy pay` `/economy leaderboard` |
| **Leveling** | `/leveling rank` `/leveling leaderboard` |
| **Utility** | `/util ping` `/util server` `/util user` `/util avatar` `/util uptime` `/util timestamp` |
| **Roles** | `/role add` `/role remove` `/role list` |
| **Config** | `/config logs` `/config welcome` `/config autorole` `/config prefix` |
| **Reminders** | `/remind create` |

---

## Stack

- **Language:** Rust 🦀
- **Framework:** [Poise](https://github.com/serenity-rs/poise) + [Serenity](https://github.com/serenity-rs/serenity)
- **Networking:** Secure Native-TLS stack
- **Async:** Tokio
- **Database:** SQLx + Neon PostgreSQL (Serverless)
- **Logging:** Tracing

---

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (1.75+)
- A Discord bot token from the [Developer Portal](https://discord.com/developers/applications)

### Setup

1. **Clone & Environment**

   ```bash
   git clone https://github.com/your-username/AegisForge.git
   cd AegisForge
   cp .env.example .env
   # Fill in DISCORD_TOKEN and both DATABASE URLs (see below)
   ```

2. **Database Migrations**

   AegisForge v3 uses Neon PostgreSQL. You must run migrations against the **Direct** URL before starting.

   ```bash
   # Ensure DATABASE_URL is set in your shell or .env
   sqlx migrate run
   ```

3. **Build & Run**

   ```bash
   cargo run --release
   ```

### Environment Variables

| Variable | Required | Description |
| :--- | :--- | :--- |
| `DISCORD_TOKEN` | ✅ | Your bot's token |
| `DATABASE_URL` | ✅ | **Direct** Neon URL — used only for migrations at startup |
| `DATABASE_POOL_URL` | ✅ | **Pooled** Neon URL — used for all bot queries |
| `DB_MAX_CONNECTIONS` | ❌ | SQLx pool size to PgBouncer (default: `10`) |
| `RUST_LOG` | ❌ | Log level, e.g. `aegisforge=info,sqlx=warn` |

> **Why two URLs?** Neon runs PgBouncer in transaction mode for pooled connections.
> Transaction mode doesn't support DDL statements (`CREATE TABLE`, etc.),
> so migrations **must** use the direct URL. Normal queries use the pooled URL
> which handles up to 10,000 client connections through Neon's built-in PgBouncer.

### Getting Your Neon URLs

The two URLs differ by only `-pooler` in the hostname:

```text
# Direct (DATABASE_URL) — for migrations:
postgresql://USER:PASS@ep-your-endpoint.region.aws.neon.tech/neondb?sslmode=require

# Pooled (DATABASE_POOL_URL) — for the bot:
postgresql://USER:PASS@ep-your-endpoint-pooler.region.aws.neon.tech/neondb?sslmode=require
```

Get both from the Neon Console → your project → **Connect** → toggle **Connection pooling**.

---

## Project Structure

```text
AegisForge/
├── src/
│   ├── main.rs              # Entry point, bot setup
│   ├── handler.rs           # Gateway event handler
│   ├── commands/
│   │   ├── mod.rs           # Module exports
│   │   ├── utility.rs       # Utility commands
│   │   ├── moderation.rs    # Moderation commands
│   │   ├── role.rs          # Role management
│   │   ├── config.rs        # Server configuration
│   │   └── remind.rs        # Reminder system
│   └── models/
│       ├── mod.rs
│       └── config.rs        # Data models
├── migrations/
│   └── 0001_initial.sql     # Database schema
├── web/
│   ├── index.html           # Landing page
│   ├── style.css
│   └── script.js
├── .env.example
├── .gitignore
├── Cargo.toml
└── context.md               # Project brief & design spec
```

---

## Permissions

AegisForge requests only what it needs:

- `Read Messages` / `Send Messages`
- `Embed Links`
- `Manage Messages`
- `Moderate Members`
- `Manage Roles`
- `View Audit Log`
- `Use Application Commands`

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Security

For security vulnerability reports, please refer to our [Security Policy](SECURITY.md).

---

Built with 🦀 and precision by the AegisForge Team.
