# Security Policy

## Supported Versions

Security updates are prioritized for the latest major version.

| Version | Supported          |
| ------- | ------------------ |
| v4.x    | ✅ Yes              |
| v3.x    | ❌ No               |
| < v3.0  | ❌ No               |

## Reporting a Vulnerability

We take the security of AegisForge seriously. If you discover a vulnerability, please do not disclose it publicly.

1. **Email**: Send a report to `security@aegisforge.io` (if available) or contact **watispro1** directly via the [Official Support Server](https://discord.gg/8p5Epc8Qd8).
2. **Details**: Include a description of the issue, steps to reproduce, and potential impact.
3. **Response**: We aim to acknowledge all reports within 48 hours and provide a fix or mitigation within 7 days.

## Security Hardening
AegisForge utilizes `native-tls` and strictly validated SQL queries via `sqlx` to prevent injection and DoS attacks. All data is encrypted in transit.
