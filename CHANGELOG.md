# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.1.2] - 2026-05-09

### Fixed

- Updated `web/api/stats.js` to proxy the live Rust bot stats API before falling back to Neon database totals.
- Updated `shards.html` to display real live shard, uptime, guild, user, economy, XP, command, inventory, and version telemetry.
- Updated `about.html` to fetch real live stats and use the command registry for its command count.
- Removed fabricated status fallback numbers from the website.

### Changed

- Bumped the shared website script cache key to `script.js?v=4.1.2`.
- Refreshed README, security policy, contributing guide, and license text.

## [4.1.0] - 2026-05-09

### Added

- **Hyperforge Market**: Added `/economy shop`, `/economy buy`, and `/economy inventory`.
- **Global Shop Catalog**: The shop is rendered from a single global catalog so new items appear automatically after deployment.
- **Persistent Inventory**: Added `economy_inventory` table for purchased item quantities per server economy.
- **Shop Categories**: Added Profile, Cosmetic, Collectible, Boost, Utility, Community, and Limited item groups.
- **Website Sync**: Updated the website command registry and economy guide for the v4.1 shop/inventory flow.

### Changed

- Bumped project version to `4.1.0`.
- Updated public docs and site copy to present v4.1 as the current release.

## [3.1.0] - 2026-05-06

### Added

- **Massive Command Expansion**: Added 20+ new commands across Fun, Utility, and Economy categories.
- **Advanced Economy**: New `rob`, `slots`, `beg`, and `search` modules with randomized logic.
- **Banking System**: Added `deposit` and `withdraw` functionality for secure credit storage.
- **Productivity Suite**: New `math`, `qr`, `crypto`, and `translate` utility commands.
- **Social Animals**: Expanded the animal command suite with `fox`, `panda`, and `bird` integrations.
- **Matchmaking Forge**: Added `/fun ship` to check compatibility between users.

### Fixed

- **Website Polish**: Refined Team page avatars and responsive command registry.
- **Infrastructure**: Synchronized SQLx query cache for new command modules.

## [3.0.0] - 2026-05-06

### Added in v3.0.0

- **Eternal Forge Update**: Major backend and feature overhaul.
- **Economy System**: Fully integrated economy with `/work`, `/daily`, `/pay`, and `/balance`. Includes server-wide leaderboards.
- **Leveling Framework**: XP-based leveling system with customizable level-up messages and automatic role rewards.
- **Moderation Audit Logs**: All moderation actions (ban, kick, warn, timeout, mute) are now permanently logged to the database with unique case IDs.
- **Warnings System**: Integrated warning system linked to moderation cases.
- **Custom Prefix Support**: Servers can now set a custom prefix for traditional message commands via `/config prefix`.
- **Fly.io Optimization**: Hardened production deployment with persistent machines and pooled database connectivity for Neon.
- **Security Hardening**: Migrated entire TLS stack to `native-tls` to resolve `RUSTSEC-2026-0104` (High DoS) and multiple `rustls-webpki` name constraint vulnerabilities.
- **SQLx Upgrade**: Upgraded to `v0.8.6` to patch critical binary protocol misinterpretation issues (`RUSTSEC-2024-0363`).

### Fixed in v3.0.0

- Resolved `23503` Foreign Key constraint violations during database writes for new servers.
- Fixed `config logs` and `config welcome` commands which previously failed to persist settings.
- Corrected various type-safety issues in Poise command handlers.
- Improved database cache invalidation logic for guild configurations.

### Changed

- Migrated from local SQLite to high-performance Neon PostgreSQL.
- Refactored `handler.rs` to handle complex v3 events (joins, leveling, auto-mod).
- Updated brand identity to "Eternal Forge" with new glassmorphic UI assets.

## [2.0.0] - 2026-05-03

### Added in v2.0.0

- Initial v2 feature set.
- Basic moderation and utility commands.
- Preliminary Fly.io and Vercel deployment support.

## [1.0.0] - 2026-04-30

- Initial public release.
