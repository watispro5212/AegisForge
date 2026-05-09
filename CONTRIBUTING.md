# Contributing to AegisForge

Thanks for helping improve AegisForge. The project is a Rust Discord bot plus a static website, so most changes should keep the bot, website, and docs in sync.

## Getting Started

1. Fork or clone the repository.
2. Install Rust from https://rustup.rs/.
3. Install SQLx CLI if you need to prepare query metadata or run migrations.
4. Copy `.env.example` to `.env` and fill in local development values.
5. Create a focused branch, for example `feature/shop-item` or `fix/live-stats`.

## Local Checks

Run the core Rust checks:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Run the website smoke test:

```bash
cd web
npm test
```

For offline SQLx builds:

```bash
SQLX_OFFLINE=true cargo check
```

## Development Guidelines

- Keep pull requests focused on one feature, fix, or documentation update.
- Run `cargo fmt` before submitting Rust changes.
- Avoid `unsafe` unless there is a clear, documented reason.
- Keep command behavior, `web/commands_data.js`, website copy, README, and changelog entries aligned.
- Add or update migrations when database schema changes.
- Keep secrets out of commits. Do not commit `.env`, tokens, database URLs, webhook URLs, or generated private credentials.
- Prefer clear, boring code over clever code in command handlers and database access.

## Website Guidelines

- The website should use real stats where possible.
- `shards.html` and `about.html` should read from the live stats path, not hardcoded counters.
- When changing shared website JavaScript, bump the `script.js?v=...` query string in HTML files so deployed browsers receive the update.
- Run a local static preview with:

```bash
cd web
python -m http.server 4173
```

## Pull Request Checklist

- Rust checks pass.
- Website smoke test passes.
- Relevant docs/changelog are updated.
- New commands are listed in `web/commands_data.js`.
- Database changes include migrations.
- No unrelated formatting churn or generated build artifacts are included.

## Community

For questions or support, join the Discord server: https://discord.gg/HbmafcgjNa
