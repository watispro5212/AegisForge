# Security Policy

## Supported Versions

Security updates are prioritized for the latest public release.

| Version | Supported |
| ------- | --------- |
| v4.x    | Yes       |
| v3.x    | No        |
| < v3.0  | No        |

## Reporting a Vulnerability

Please do not disclose suspected vulnerabilities publicly.

1. Open a private report through GitHub security advisories if available.
2. If GitHub advisories are unavailable, contact the maintainer through the official support server: https://discord.gg/HbmafcgjNa
3. Include the affected component, reproduction steps, expected impact, logs or screenshots if useful, and whether the issue is already being exploited.

We aim to acknowledge valid reports within 48 hours and provide a fix, mitigation, or status update within 7 days.

## Scope

Security reports are most useful when they affect:

- Discord command authorization or permission checks.
- Moderation, warning, or audit-log integrity.
- Economy balances, inventory, vote rewards, or transaction accounting.
- SQL queries, migrations, or database access.
- Public API routes such as `/api/stats`, `/api/health`, and `/api/vote`.
- Secrets, tokens, deployment configuration, or webhook verification.

## Hardening Notes

- SQL is executed through SQLx with parameterized queries.
- Runtime database traffic should use pooled Neon/PgBouncer connections.
- Schema migrations should use the direct Neon connection string.
- Top.gg vote rewards should be protected with `TOPGG_WEBHOOK_SECRET` when enabled.
- Never commit `.env`, Discord tokens, database URLs, webhook URLs, or deployment secrets.

## Dependency Checks

Run these before release when possible:

```bash
cargo audit
cargo clippy --all-targets -- -D warnings
cd web && npm audit
```
