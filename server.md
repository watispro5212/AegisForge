# AegisForge Support Server Blueprint

This guide is the setup plan for the official **AegisForge Support** Discord server. It covers the server structure, permissions, roles, suggested bots, launch checklist, and ready-to-post messages for each important channel.

## Server Purpose

The server should feel like the control room for AegisForge:

- Help server owners install, configure, and troubleshoot the bot.
- Give beta testers a clean place to report issues.
- Let users test commands without cluttering their own servers.
- Announce releases, incidents, migrations, and maintenance.
- Keep bugs, feature requests, and feedback organized.
- Build a helpful, sharp, practical, fully SFW community.

## Server Identity

**Server name:** `AegisForge Support`  
**Short description:** `Support, updates, testing, and feedback for the AegisForge Discord bot.`  
**Website:** `https://aegisforge-vert.vercel.app`  
**Support invite:** `https://discord.gg/HbmafcgjNa`  
**Tone:** Clean, helpful, confident, practical, and SFW.

## Category Layout

| Order | Category | Purpose |
|---:|---|---|
| 1 | `START HERE` | Rules, welcome, links, FAQ, and onboarding. |
| 2 | `OFFICIAL UPDATES` | Announcements, status, changelog, incidents, and maintenance. |
| 3 | `SUPPORT DESK` | Setup help, bug reports, beta feedback, and feature requests. |
| 4 | `TESTING LAB` | Bot command testing, economy testing, leveling testing, and AutoMod testing. |
| 5 | `COMMUNITY` | General chat, showcases, memes, shop talk, and leaderboards. |
| 6 | `DEVELOPMENT` | Contributor and staff-facing development channels. |
| 7 | `STAFF` | Private moderation, triage, audit, and operations channels. |
| 8 | `VOICE` | Optional voice channels for live support and hangouts. |

## Roles

Keep higher-trust roles above lower-trust roles. Keep bot roles high enough to manage the roles they need, but below the owner/admin staff roles.

| Order | Role | Purpose | Recommended Color | Key Permissions |
|---:|---|---|---|---|
| 1 | `Owner` | Project/server owner. | Gold | Administrator. |
| 2 | `Core Developer` | Code, deploys, infrastructure, release authority. | Cyan | Manage Server, Manage Channels, Manage Webhooks, Manage Messages, Manage Roles below them. |
| 3 | `Staff` | Moderation and support operations. | Blue | Manage Messages, Timeout Members, Manage Threads. Kick/Ban only if trusted. |
| 4 | `Support Helper` | Trusted helpers for setup questions. | Green | Manage Threads, Pin Messages, no server-wide moderation by default. |
| 5 | `Beta Tester` | Users testing beta builds. | Purple | Access beta/staging channels. |
| 6 | `Contributor` | Code, docs, design, and testing contributors. | Teal | Access development channels. |
| 7 | `Supporter` | Helpful community members, voters, boosters. | Pink | Cosmetic role, optional media permissions. |
| 8 | `Member` | Default community member. | Gray | Standard public access. |
| 9 | `AegisForge` | Production bot role. | Cyan | Bot permissions listed below. |
| 10 | `AegisForge Beta` | Staging/test bot role. | Purple | Testing-only bot permissions. |

## Bot Stack

Use AegisForge as the main server operations bot. Add only the outside bots that cover jobs AegisForge should not own yet.

| Bot | Use It For | Where It Should Post | Notes |
|---|---|---|---|
| `AegisForge` | Main moderation, Sentinel anti-raid, AutoMod, warnings, cases, economy, leveling, utility commands, support server testing. | `mod-alerts`, `case-log`, `command-testing`, `economy-testing`, `leveling-testing`, `automod-testing`. | Primary bot. Put its role above mute/test roles and below staff/admin roles. |
| `AegisForge Beta` | Staging command tests before public release. | `beta-feedback`, `staging-notes`, `testing-lab` channels. | Keep separate from production so beta behavior does not confuse normal users. |
| `Carl-bot` or `Sapphire` | Reaction roles, welcome embeds, sticky messages, simple automations. | `welcome`, `rules`, optional `roles`. | Use only if AegisForge does not yet handle the specific onboarding automation you want. |
| `Ticket Tool` | Private support tickets for account-specific or sensitive help. | Creates ticket channels under a private ticket category. | Optional. Public support threads are better for normal setup questions. |
| `GitHub` | Repository commit, issue, pull request, and release notifications. | `deploy-log`, `docs-work`, `dev-chat`. | Good for contributor visibility. Keep public noise low. |
| `Pingcord` or `MonitoRSS` | Website, changelog, GitHub release, or Top.gg notification feeds. | `announcements`, `changelog`, `deploy-log`. | Use webhooks where possible so the bot list stays lean. |
| `Statbot` or `Sesh` | Community analytics or scheduled events. | Staff-only analytics, event planning channels. | Optional. Do not add if you will not actively use the data. |

Recommended bot rule: if a bot does not have a clear job, do not invite it. Too many bots makes a support server look messy and harder to trust.

## Webhooks To Install

Use webhooks for automated updates that do not need a full Discord bot. Keep names consistent and give each webhook one job.

| Webhook Name | Channel | Purpose | Install From | Notes |
|---|---|---|---|---|
| `AegisForge Status` | `status` | Bot/API/website health, incidents, degraded service, recovery messages. | Fly.io app alerts, uptime monitor, or custom `STATUS_WEBHOOK_URL`. | This is the main operational webhook. Keep messages short and factual. |
| `AegisForge Votes` | `status` or `leaderboards` | Top.gg vote reward notifications. | Existing `STATUS_WEBHOOK_URL` or a separate vote webhook if you split it later. | Current code posts vote rewards through `STATUS_WEBHOOK_URL`. |
| `GitHub Releases` | `changelog` | New releases, tags, and release notes. | GitHub repository webhooks or GitHub Discord integration. | Post only releases/tags here, not every commit. |
| `GitHub Issues` | `triage-board` or `dev-chat` | New issues, reopened issues, and priority bug reports. | GitHub repository webhooks. | Keep public issue noise out of announcements. |
| `GitHub Pull Requests` | `dev-chat` | PR opened, reviewed, merged, or failed checks. | GitHub repository webhooks. | Useful for contributors and staff only. |
| `Deploy Log` | `deploy-log` | Fly.io deploys, Vercel deploys, rollbacks, and build failures. | Fly.io deploy hooks, Vercel webhooks, GitHub Actions webhook step. | Staff/dev visibility. Do not post this publicly unless you want transparent ops. |
| `Website Monitor` | `status` | Website uptime checks and SSL/domain problems. | Better Stack, UptimeRobot, Pulsetic, or Healthchecks.io. | Configure alerts only for confirmed downtime, not every slow response. |
| `API Monitor` | `status` | `GET /api/health` and `GET /api/stats` failures. | Better Stack, UptimeRobot, Pulsetic, or Healthchecks.io. | Point checks at the Fly.io bot API. |
| `Database Monitor` | `admin-ops` | Neon connection, branch, compute, or usage warnings. | Neon alerts or a small custom monitor. | Keep private because database alerts can reveal infrastructure details. |
| `Security Alerts` | `mod-alerts` | Sentinel, AutoMod, raid response, and urgent moderation events if you split logs later. | AegisForge custom webhook or Discord channel logs. | Only add if bot channel logging is not enough. |
| `Docs Updates` | `docs-work` | README, server guide, website copy, FAQ, and policy updates. | GitHub path-filtered workflow webhook. | Good for tracking user-facing docs changes. |
| `Support Intake` | `triage-board` | External form submissions or bug intake forms. | Tally, Typeform, Google Forms via Zapier/Make, or custom endpoint. | Use only if you want off-Discord reports. |
| `Top.gg Feed` | `announcements` or `leaderboards` | Milestones, vote goals, bot listing updates. | Top.gg webhook/integration where available. | Keep vote spam out of announcements unless it is a milestone. |
| `Community Events` | `announcements` | Scheduled test nights, giveaways, beta windows. | Sesh, Google Calendar automation, or manual Discord webhook. | Optional, only for planned events. |

Recommended webhook setup:

1. Create the channel first.
2. Create the webhook inside that channel.
3. Name the webhook after the job, not the service.
4. Use a clear avatar if the service supports it.
5. Store webhook URLs in `.env` or the provider dashboard, never in public chat or GitHub.
6. Test each webhook with one message before launch.

Suggested `.env` names if you split webhooks later:

```env
STATUS_WEBHOOK_URL=
VOTE_WEBHOOK_URL=
APP_ENV=production
PUBLIC_API_URL=https://aegisforge-bot.fly.dev
DEPLOY_WEBHOOK_URL=
SECURITY_WEBHOOK_URL=
SUPPORT_INTAKE_WEBHOOK_URL=
```

Current bot behavior:

- On startup, `STATUS_WEBHOOK_URL` receives an operational status embed with version, environment, cached servers, cached users, registered commands, health link, stats link, and start time.
- Top.gg vote reward notifications use `VOTE_WEBHOOK_URL` when set.
- If `VOTE_WEBHOOK_URL` is not set, vote reward notifications fall back to `STATUS_WEBHOOK_URL`.

Suggested status webhook message:

```text
Status: Degraded
Affected: Bot API
Impact: Commands are working, but live website stats may be delayed.
Started: 2026-05-16 3:20 PM CT
Next update: 15 minutes
```

Suggested deploy webhook message:

```text
Deploy: AegisForge API
Environment: Production
Version/commit:
Result: Success
Notes: Stats endpoint and vote webhook checked after deploy.
```

## Bot Permission Requirements

For easiest setup, the current invite uses broad permissions. For a stricter production server, grant AegisForge:

### Required

- View Channels
- Send Messages
- Send Messages in Threads
- Embed Links
- Attach Files
- Read Message History
- Use Application Commands
- Add Reactions
- Manage Messages
- Manage Roles
- Moderate Members

### Needed for moderation commands

- Ban Members
- Kick Members
- Moderate Members
- Manage Channels
- Manage Messages

### Needed for configuration and automation

- Manage Roles for autorole and role commands.
- View Audit Log for richer moderation diagnostics.
- Manage Webhooks only if you use webhook-based status or log integrations.

### Role hierarchy warning

Place the `AegisForge` role:

- Above roles it needs to assign or remove.
- Below Owner, Core Developer, and Staff roles.
- Above muted/test roles if the bot manages them.

If role commands fail, role hierarchy is usually the reason.

## START HERE

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `rules` | Text | Rules and safety expectations. | View, Read History. No Send. | Send, Manage Messages. |
| `welcome` | Text | First message and quick orientation. | View, Read History. No Send. | Send, Manage Messages. |
| `links` | Text | Bot invite, website, GitHub, Top.gg, support links. | View, Read History. No Send. | Send, Manage Messages. |
| `faq` | Text | Common setup, permissions, slash sync, and economy answers. | View, Read History. No Send. | Send, Manage Messages. |
| `server-guide` | Text | Explains where to ask for help and test commands. | View, Read History. No Send. | Send, Manage Messages. |

### Message For `rules`

```text
Welcome to AegisForge Support.

Rules:
1. Keep everything SFW.
2. Be respectful. Criticism is fine; harassment is not.
3. Use the right channel so staff can help faster.
4. Do not spam commands outside the testing channels.
5. Do not post tokens, private keys, database URLs, or sensitive server logs.
6. Bug reports need steps to reproduce, screenshots/logs when possible, and the command or page affected.
7. Staff decisions are final. If something feels wrong, open a calm support thread.

By staying here, you agree to keep the server useful, safe, and focused.
```

### Message For `welcome`

```text
Welcome to AegisForge Support.

AegisForge is a Rust-powered Discord bot for moderation, Sentinel anti-raid, AutoMod, economy, leveling, giveaways, utility commands, and live telemetry.

Start here:
- Need setup help? Go to #support.
- Found a bug? Use #bug-reports.
- Want to test commands? Use #command-testing.
- Want updates? Watch #announcements and #status.
- Need links? Check #links.

When asking for help, include the command you ran, what happened, what you expected, and a screenshot or error message if you have one.
```

### Message For `links`

```text
Official AegisForge links:

Website: https://aegisforge-vert.vercel.app
Invite Bot: https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&scope=bot%20applications.commands
GitHub: https://github.com/watispro5212/AegisForge
Top.gg: https://top.gg/bot/1500582485367722004
Support Invite: https://discord.gg/HbmafcgjNa

Only trust links posted in this channel or by staff.
```

### Message For `faq`

```text
FAQ

Q: Slash commands are missing. What do I do?
A: Reinvite the bot with the applications.commands scope, wait a few minutes, then check that the bot has permission to use application commands in the channel.

Q: Moderation commands are failing.
A: Check role hierarchy first. AegisForge cannot manage users or roles above its own role.

Q: How do I set logs?
A: Use /logs, /msglogs, and /memberlogs with the channels you want events posted into.

Q: How do I enable Sentinel?
A: Use /sentinel enable, then tune the threshold if needed.

Q: How do I set up shadow bans?
A: Create a muted role, configure it with /muterole, then use /shadowban on the target user.

Q: Where can I test economy commands?
A: Use #economy-testing.
```

### Message For `server-guide`

```text
Where to go:

#support - Setup questions, permissions, and command help.
#bug-reports - Reproducible problems with commands, the website, API, or docs.
#beta-feedback - Feedback on beta builds and staging behavior.
#feature-requests - New command, dashboard, docs, shop, or moderation ideas.
#command-testing - General command testing.
#economy-testing - Economy commands only.
#leveling-testing - XP, rank, leaderboard, and rank card testing.
#automod-testing - Controlled AutoMod checks.
#general - Normal community chat.
```

## OFFICIAL UPDATES

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `announcements` | Announcement | Major updates, releases, and important notices. | View, Read History, Reactions. No Send. | Send, Manage Messages, Mention Everyone when needed. |
| `status` | Text | Bot/API/website uptime, incidents, and maintenance. | View, Read History. No Send. | Send, Manage Messages. |
| `changelog` | Text | Release notes and command changes. | View, Read History, Reactions. No Send. | Send, Manage Messages. |
| `maintenance` | Text | Scheduled downtime, migrations, API changes. | View, Read History. No Send. | Send, Manage Messages. |

### Message For `announcements`

```text
This channel is for important AegisForge announcements: releases, major fixes, security notes, and support server updates.

For live service health, use #status.
For detailed release notes, use #changelog.
For support, use #support.
```

### Message For `status`

```text
Status updates for AegisForge services will be posted here.

Format:
Status: Operational / Degraded / Outage / Maintenance
Affected: Bot / API / Website / Database / Commands
Started:
Current impact:
Next update:
```

### Message For `changelog`

```text
Release notes and command changes go here.

Recommended format:
Version:
Highlights:
Added:
Changed:
Fixed:
Known issues:
Docs:
```

### Message For `maintenance`

```text
Scheduled maintenance notices go here.

Recommended format:
Window:
Affected services:
Expected impact:
Reason:
Rollback plan:
Updates will continue in #status.
```

## SUPPORT DESK

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `support` | Text or Forum | General setup, commands, permissions, and config help. | View, Send/Create Posts, Read History, Attach Files, Embed Links. | Manage Messages, Manage Threads, Timeout Members. |
| `bug-reports` | Forum | Reproducible bugs for bot, website, API, or docs. | View, Create Posts, Attach Files, Embed Links, Read History. | Manage Posts/Threads, Tag Posts, Close Threads. |
| `beta-feedback` | Forum | Reports and usability feedback on beta builds. | View, Create Posts, Attach Files, Read History. | Manage Threads, Pin Messages, Apply Tags. |
| `feature-requests` | Forum | New command, shop, dashboard, docs, and moderation ideas. | View, Create Posts, Add Reactions, Read History. | Manage Threads, Apply Tags, Lock Duplicates. |
| `known-issues` | Text | Staff-maintained list of known bugs and workarounds. | View, Read History. No Send. | Send, Manage Messages. |

Recommended slowmode:

- `support`: 5 seconds.
- `bug-reports`: no slowmode if forum, 15 seconds if text.
- `feature-requests`: no slowmode if forum, 30 seconds if text.

### Message For `support`

```text
Need help with AegisForge? Post here.

Please include:
- What you are trying to do.
- The command you used.
- What happened instead.
- Whether the bot has the needed permissions.
- Whether the AegisForge role is above the role/user it is trying to manage.
- Screenshots or logs if useful.

Do not post bot tokens, database URLs, private keys, or sensitive user data.
```

### Message For `bug-reports`

```text
Bug reports should be reproducible.

Use this format:
Bug summary:
Command/page affected:
Steps to reproduce:
1.
2.
3.
Expected behavior:
Actual behavior:
Approximate time:
Screenshot/error:
```

### Message For `beta-feedback`

```text
Use this channel for beta/staging feedback.

Format:
Build/version tested:
Feature tested:
What worked:
What felt confusing:
What broke or felt unreliable:
Suggested improvement:
```

### Message For `feature-requests`

```text
Feature requests are welcome.

Format:
Feature name:
Problem it solves:
Who would use it:
Suggested command or page:
Priority: Low / Medium / High
Notes or examples:
```

### Message For `known-issues`

```text
Known issues and workarounds will be tracked here by staff.

Format:
Issue:
Affected area:
Status: Investigating / Confirmed / Fix ready / Fixed
Workaround:
Last updated:
```

## TESTING LAB

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `command-testing` | Text | General slash command testing. | View, Send, Read History, Use Application Commands. | Manage Messages. |
| `economy-testing` | Text | Test daily, work, slots, shop, fish, hunt, crime, inventory. | View, Send, Read History, Use Application Commands. | Manage Messages. |
| `leveling-testing` | Text | XP, rank, leaderboard, and rank card testing. | View, Send, Read History, Use Application Commands. | Manage Messages. |
| `automod-testing` | Text | Controlled AutoMod checks with harmless test phrases. | View, Send, Read History, Use Application Commands. | Manage Messages. |
| `embed-previews` | Text | Preview bot embeds, announcements, and docs copy. | View, Send, Read History, Attach Files, Embed Links. | Manage Messages. |

Recommended slowmode:

- `command-testing`: 3 seconds.
- `economy-testing`: 3 seconds.
- `leveling-testing`: 5 seconds.
- `automod-testing`: 10 seconds.

### Message For `command-testing`

```text
Use this channel to test AegisForge slash commands.

Good tests:
/ping
/stats
/help
/serverinfo
/userinfo
/poll

Keep command spam here instead of public chat.
```

### Message For `economy-testing`

```text
Economy command testing lives here.

Try:
/economy balance
/economy daily
/economy work
/economy fish
/economy hunt
/economy slots
/economy shop
/economy inventory
/economy leaderboard

Report weird balances, cooldown issues, missing inventory items, or confusing responses in #bug-reports.
```

### Message For `leveling-testing`

```text
Leveling and XP testing lives here.

Try:
/leveling rank
/leveling leaderboard
/leveling settings

Useful feedback:
- XP feels too fast or too slow.
- Rank cards look wrong.
- Leaderboards do not match expected data.
- Role rewards fail because of role hierarchy.
```

### Message For `automod-testing`

```text
Controlled AutoMod testing only.

Allowed tests:
- Harmless repeated text for spam checks.
- Safe fake invite/link examples.
- Caps testing with non-abusive text.
- Mention-count tests only when staff approves.

Do not post slurs, gore, sexual content, threats, token leaks, or real malicious links.
```

### Message For `embed-previews`

```text
Use this channel to preview announcement, changelog, docs, and bot embed copy before posting it publicly.

Staff can clean up previews after review.
```

## COMMUNITY

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `general` | Text | Normal community discussion. | View, Send, Read History, Reactions, Attach Files. | Manage Messages, Timeout Members. |
| `showcase` | Text | Server setups, rank cards, dashboards, embed designs. | View, Send, Read History, Attach Files, Embed Links. | Manage Messages. |
| `memes` | Text | Light, funny, SFW memes. | View, Send, Read History, Attach Files, Reactions. | Manage Messages. |
| `shop-talk` | Text | Global shop item ideas and economy balancing. | View, Send, Read History, Reactions. | Manage Messages. |
| `leaderboards` | Text | Economy/leveling screenshots and bragging rights. | View, Send, Read History, Attach Files. | Manage Messages. |

### Message For `general`

```text
General AegisForge community chat.

Keep it useful, chill, and SFW. For setup problems, use #support so your question does not get buried.
```

### Message For `showcase`

```text
Show off your AegisForge setup here.

Good posts:
- Mod log layouts.
- Rank cards.
- Economy screenshots.
- Dashboard/status setups.
- Clean server onboarding flows.
```

### Message For `shop-talk`

```text
Use this channel for global shop item ideas and economy balance discussion.

Item request format:
Item name:
Category: Profile / Cosmetic / Collectible / Boost / Utility / Community / Limited
Suggested price:
Rarity: Common / Rare / Epic / Legendary / Mythic
What should it do or represent?
Permanent, seasonal, or limited?
```

## DEVELOPMENT

| Channel | Type | Purpose | Member Permissions | Contributor Permissions | Staff Permissions |
|---|---|---|---|---|---|
| `dev-chat` | Text | Development discussion and implementation planning. | No View unless public dev is desired. | View, Send, Read History, Attach Files. | Manage Messages. |
| `deploy-log` | Text | Fly.io deploys, Vercel deploys, and build notes. | No View. | View, Read History. No Send unless trusted. | Send, Manage Messages. |
| `staging-notes` | Text | Beta/staging bot notes and test plans. | No View. | View, Send, Read History. | Manage Messages. |
| `docs-work` | Text | README, server guide, website copy, FAQ work. | No View unless public docs work is desired. | View, Send, Read History. | Manage Messages. |

### Message For `dev-chat`

```text
Development discussion for AegisForge.

Keep threads tied to a concrete task: bot command, website, API, database, docs, deployment, or testing.
```

### Message For `deploy-log`

```text
Deploy and build notes go here.

Recommended format:
Service:
Environment: Production / Staging
Version/commit:
Started:
Result:
Notes:
```

## STAFF

| Channel | Type | Purpose | Staff Permissions |
|---|---|---|
| `staff-chat` | Text | Private staff coordination. | View, Send, Read History, Attach Files, Embed Links. |
| `mod-alerts` | Text | Reports, AutoMod alerts, urgent moderation notes. | View, Send, Read History, Manage Messages. |
| `case-log` | Text | Manual moderation case notes and decisions. | View, Send, Read History, Manage Messages. |
| `triage-board` | Forum or Text | Staff triage for bugs, feature requests, and beta feedback. | View, Send/Create Posts, Manage Threads, Apply Tags. |
| `admin-ops` | Text | Owner/core developer operations only. | Owner/Core Developer only. |

### Message For `staff-chat`

```text
Staff coordination channel.

Use threads for support cases, moderation reviews, release planning, and user reports that need follow-up.
```

### Message For `mod-alerts`

```text
Moderation alerts from AegisForge and staff reports go here.

Staff should acknowledge urgent alerts with:
Claimed by:
Action taken:
Follow-up needed:
```

### Message For `case-log`

```text
Manual moderation decisions go here.

Format:
Case:
User:
Action:
Reason:
Evidence:
Duration:
Moderator:
Follow-up:
```

## VOICE

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `general-vc` | Voice | Casual voice chat. | View, Connect, Speak, Use Voice Activity. | Move Members, Mute Members. |
| `support-vc` | Voice | Live support calls when text is not enough. | View, Connect, Speak. | Move Members, Mute Members, Priority Speaker. |
| `testing-vc` | Voice | Beta test sessions and live debugging. | Beta Tester/Contributor only. | Move Members, Mute Members. |
| `staff-vc` | Voice | Staff-only calls. | Staff only. | Full voice moderation. |

Recommended voice settings:

- Disable Use Soundboard for everyone if it gets noisy.
- Allow Stream in support/testing channels if screen sharing helps.
- Keep Priority Speaker limited to Staff.

## Staff Triage Flow

1. Identify the area: bot command, website, API, deployment, database, docs, shop, or permissions.
2. Ask for exact command usage, page URL, screenshot, timestamp, and expected behavior.
3. Check common causes first: missing permissions, role hierarchy, slash command sync, cooldowns, or invalid input.
4. If reproducible, move it to `bug-reports` or create a GitHub issue.
5. If beta-specific, move it to `beta-feedback`.
6. If known, add it to `known-issues`.
7. If it affects uptime or deployment, post a short update in `status`.

## Launch Checklist

- Create all categories and channels in the listed order.
- Apply category-level permissions first.
- Lock `rules`, `announcements`, `status`, `changelog`, `maintenance`, `faq`, and `known-issues`.
- Add pinned templates to support, bug reports, beta feedback, feature requests, shop talk, and staff triage.
- Put `AegisForge` high enough in role hierarchy.
- Test `/help`, `/ping`, `/stats`, `/economy balance`, `/economy shop`, `/leveling rank`, and moderation commands.
- Configure `/logs`, `/msglogs`, `/memberlogs`, `/welcome`, `/autorole`, `/prefix`, and `/settings`.
- Run one test incident post in `status`.
- Invite beta testers only after testing channels and rules are ready.

## Final Standard

AegisForge should feel fast, practical, and trustworthy. The support server should match that: clear channels, sane permissions, useful templates, lean bot usage, and enough personality to feel alive without becoming chaotic.
