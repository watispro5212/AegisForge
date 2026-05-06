# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

### Fixed

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
