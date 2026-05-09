# 🛡️ AegisForge Support Server Blueprint

This is the full setup guide for the official **AegisForge Support** Discord server. It covers server identity, emoji categories, channel layout, role hierarchy, detailed permissions, support templates, staff workflow, and launch checks.

## 🎯 Server Purpose

The server should feel like the command center for AegisForge:

- Help server owners install, configure, and troubleshoot the bot.
- Give beta testers a structured place to report issues.
- Let users test commands without cluttering their own servers.
- Announce releases, incidents, migrations, and maintenance.
- Keep feature requests, bugs, and usability feedback organized.
- Build a community that is helpful, sharp, lightly funny, and fully SFW.

## 🏷️ Server Identity

**Server name:** `AegisForge Support`  
**Short description:** `Support, updates, testing, and feedback for the AegisForge Discord bot.`  
**Website:** `https://aegisforge-vert.vercel.app`  
**Support invite:** `https://discord.gg/8p5Epc8Qd8`  
**Tone:** Clean, helpful, confident, practical, and SFW.

Suggested welcome blurb:

> Welcome to **AegisForge Support**. Use this server for setup help, bug reports, beta feedback, command testing, release updates, and community discussion. Check the rules, pick the right channel, and include screenshots or logs when asking for help.

## 🧱 Category Layout

Use these category names exactly, or copy the structure and adjust the emoji style later.

| Order | Category | Purpose |
|---:|---|---|
| 1 | `📌 START HERE` | Rules, links, FAQ, and onboarding. |
| 2 | `📣 OFFICIAL UPDATES` | Announcements, status, changelog, incidents. |
| 3 | `🛠 SUPPORT DESK` | General support, bug reports, beta feedback, feature requests. |
| 4 | `🧪 TESTING LAB` | Bot command testing, economy testing, leveling testing, AutoMod testing. |
| 5 | `💬 COMMUNITY` | General chat, showcases, memes, bot discussion. |
| 6 | `🧑‍💻 DEVELOPMENT` | Contributor and staff-facing development channels. |
| 7 | `🔒 STAFF` | Private moderation, triage, audit, and operations channels. |
| 8 | `🔊 VOICE` | Optional voice channels for support calls and community hangouts. |

## 📌 START HERE

| Channel | Type | Purpose | @everyone Permissions | Staff Permissions |
|---|---|---|---|---|
| `📜┃rules` | Text | Server rules and safety expectations. | View Channel, Read Message History. No Send Messages, Reactions, Threads. | Manage Messages, Send Messages, Embed Links, Attach Files. |
| `👋┃welcome` | Text | Welcome message and quick orientation. | View Channel, Read Message History. No Send Messages. | Send Messages, Manage Messages. |
| `🔗┃links` | Text | Bot invite, website, GitHub, Top.gg, support links. | View Channel, Read Message History. No Send Messages. | Send Messages, Manage Messages. |
| `❓┃faq` | Text | Common setup, permissions, slash sync, and economy answers. | View Channel, Read Message History. No Send Messages. | Send Messages, Manage Messages. |
| `🧭┃server-guide` | Text | Explains where to ask for help and test commands. | View Channel, Read Message History. No Send Messages. | Send Messages, Manage Messages. |

Recommended category permissions:

- `@everyone`: ✅ View Channel, ✅ Read Message History, ❌ Send Messages.
- `Staff`: ✅ Send Messages, ✅ Manage Messages, ✅ Create Public Threads.
- `AegisForge`: ✅ View Channel, ✅ Send Messages, ✅ Embed Links, ✅ Attach Files, ✅ Read Message History.

## 📣 OFFICIAL UPDATES

| Channel | Type | Purpose | @everyone Permissions | Staff Permissions |
|---|---|---|---|---|
| `📢┃announcements` | Announcement | Major updates, releases, and important notices. | View Channel, Read Message History, Add Reactions. No Send Messages. | Send Messages, Manage Messages, Mention Everyone when needed. |
| `🟢┃status` | Text | Bot/API/website uptime, incidents, and maintenance. | View Channel, Read Message History. No Send Messages. | Send Messages, Manage Messages. |
| `🧾┃changelog` | Text | Detailed release notes and command changes. | View Channel, Read Message History, Add Reactions. No Send Messages. | Send Messages, Manage Messages. |
| `🚧┃maintenance` | Text | Scheduled downtime, migrations, API changes. | View Channel, Read Message History. No Send Messages. | Send Messages, Manage Messages. |

Recommended category permissions:

- `@everyone`: ✅ View Channel, ✅ Read Message History, ✅ Add Reactions, ❌ Send Messages.
- `Staff`: ✅ Send Messages, ✅ Manage Messages, ✅ Mention Everyone.
- `AegisForge`: ✅ Send Messages, ✅ Embed Links, ✅ Attach Files.

## 🛠 SUPPORT DESK

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `🆘┃support` | Text | General help with setup, commands, permissions, and config. | View, Send, Read History, Attach Files, Embed Links, Create Public Threads. | Manage Messages, Manage Threads, Timeout Members. |
| `🐞┃bug-reports` | Forum or Text | Reproducible bugs for bot, website, API, or docs. | View, Send/Create Posts, Attach Files, Embed Links, Read History. | Manage Posts/Threads, Tag Posts, Close Threads. |
| `🧪┃beta-feedback` | Forum or Text | Detailed reports and usability feedback on beta builds. | View, Send/Create Posts, Attach Files, Read History. | Manage Threads, Pin Messages, Apply Tags. |
| `💡┃feature-requests` | Forum or Text | New command, shop, dashboard, docs, and moderation ideas. | View, Send/Create Posts, Add Reactions, Read History. | Manage Threads, Apply Tags, Lock Duplicates. |
| `🧷┃known-issues` | Text | Staff-maintained list of known bugs/workarounds. | View, Read History. No Send Messages. | Send Messages, Manage Messages. |

Recommended support permissions:

- `@everyone`: ✅ View Channel, ✅ Send Messages, ✅ Attach Files, ✅ Embed Links, ✅ Read Message History, ✅ Create Public Threads.
- `Member`: same as `@everyone`.
- `Beta Tester`: same as Member, plus priority access to `🧪┃beta-feedback`.
- `Support Helper`: ✅ Manage Threads, ✅ Pin Messages, ✅ Use External Emojis/Stickers.
- `Staff`: ✅ Manage Messages, ✅ Manage Threads, ✅ Timeout Members.
- `AegisForge`: ✅ Send Messages, ✅ Embed Links, ✅ Attach Files, ✅ Add Reactions.

Recommended slowmode:

- `🆘┃support`: 5 seconds.
- `🐞┃bug-reports`: 15 seconds if text channel, none if forum.
- `💡┃feature-requests`: 30 seconds if text channel, none if forum.

## 🧪 TESTING LAB

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `🤖┃command-testing` | Text | General slash command testing. | View, Send, Read History, Use Application Commands. | Manage Messages. |
| `💰┃economy-testing` | Text | Test `/economy daily`, `/economy work`, `/economy slots`, `/economy shop`, `/economy fish`, `/economy hunt`, `/economy crime`. | View, Send, Read History, Use Application Commands. | Manage Messages. |
| `📈┃leveling-testing` | Text | XP, rank, leaderboard, and rank card testing. | View, Send, Read History, Use Application Commands. | Manage Messages. |
| `🛡️┃automod-testing` | Text | Controlled AutoMod checks with harmless test phrases. | View, Send, Read History, Use Application Commands. | Manage Messages, Manage Webhooks if needed. |
| `🧾┃embed-previews` | Text | Preview bot embeds, announcements, and docs copy. | View, Send, Read History, Attach Files, Embed Links. | Manage Messages. |

Recommended testing permissions:

- `@everyone`: ✅ View Channel, ✅ Send Messages, ✅ Read Message History, ✅ Use Application Commands.
- `AegisForge`: ✅ View Channel, ✅ Send Messages, ✅ Use Application Commands, ✅ Embed Links, ✅ Attach Files, ✅ Manage Messages if AutoMod cleanup is tested.
- `Staff`: ✅ Manage Messages.

Recommended slowmode:

- `🤖┃command-testing`: 3 seconds.
- `💰┃economy-testing`: 3 seconds.
- `📈┃leveling-testing`: 5 seconds.
- `🛡️┃automod-testing`: 10 seconds.

## 💬 COMMUNITY

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `💬┃general` | Text | Normal community discussion. | View, Send, Read History, Reactions, Attach Files. | Manage Messages, Timeout Members. |
| `🖼️┃showcase` | Text | Server setups, rank cards, dashboards, embed designs. | View, Send, Read History, Attach Files, Embed Links. | Manage Messages. |
| `😂┃memes` | Text | The community meme repository. Light, funny, SFW only. | View, Send, Read History, Attach Files, Reactions. | Manage Messages. |
| `🛒┃shop-talk` | Text | Discuss global shop items, future item ideas, and economy balancing. | View, Send, Read History, Reactions. | Manage Messages. |
| `🏆┃leaderboards` | Text | Economy/leveling leaderboard screenshots and bragging rights. | View, Send, Read History, Attach Files. | Manage Messages. |

Recommended community permissions:

- `@everyone`: ✅ View Channel, ✅ Send Messages, ✅ Read Message History, ✅ Add Reactions, ✅ Attach Files.
- `Member`: same as `@everyone`.
- `Staff`: ✅ Manage Messages, ✅ Timeout Members.
- `AegisForge`: ✅ Send Messages, ✅ Embed Links, ✅ Attach Files.

Recommended slowmode:

- `💬┃general`: 2 seconds.
- `😂┃memes`: 5 seconds.
- `🛒┃shop-talk`: 5 seconds.

## 🧑‍💻 DEVELOPMENT

| Channel | Type | Purpose | Member Permissions | Contributor Permissions | Staff Permissions |
|---|---|---|---|---|---|
| `🧠┃dev-chat` | Text | Development discussion and implementation planning. | No View Channel unless public dev is desired. | View, Send, Read History, Attach Files. | Manage Messages. |
| `📦┃deploy-log` | Text | Fly.io deploys, Vercel deploys, and build notes. | No View Channel. | View, Read History. No Send unless trusted. | Send, Manage Messages. |
| `🧪┃staging-notes` | Text | Beta/staging bot notes and test plans. | No View Channel. | View, Send, Read History. | Manage Messages. |
| `📚┃docs-work` | Text | README, server.md, website copy, FAQ work. | No View Channel unless public docs work is desired. | View, Send, Read History. | Manage Messages. |

Recommended development permissions:

- `@everyone`: ❌ View Channel.
- `Contributor`: ✅ View Channel, ✅ Send Messages, ✅ Read Message History, ✅ Attach Files, ✅ Embed Links.
- `Beta Tester`: optional View for `🧪┃staging-notes`, no deploy-log access.
- `Core Developer`: ✅ Manage Messages, ✅ Manage Webhooks, ✅ Mention Roles if needed.
- `AegisForge`: optional Send/Embed permissions only in `📦┃deploy-log`.

## 🔒 STAFF

| Channel | Type | Purpose | Staff Permissions |
|---|---|---|
| `🛡️┃staff-chat` | Text | Private staff coordination. | View, Send, Read History, Attach Files, Embed Links. |
| `🚨┃mod-alerts` | Text | Reports, AutoMod alerts, urgent moderation notes. | View, Send, Read History, Manage Messages. |
| `🧾┃case-log` | Text | Manual moderation case notes and decisions. | View, Send, Read History, Manage Messages. |
| `📌┃triage-board` | Forum or Text | Staff triage for bugs, feature requests, and beta feedback. | View, Send/Create Posts, Manage Threads, Apply Tags. |
| `🔐┃admin-ops` | Text | Owner/core developer operations only. | Owner/Core Developer only. |

Recommended staff permissions:

- `@everyone`: ❌ View Channel.
- `Support Helper`: optional access to `📌┃triage-board`, no `🔐┃admin-ops`.
- `Staff`: ✅ View Channel, ✅ Send Messages, ✅ Read Message History, ✅ Manage Messages, ✅ Manage Threads, ✅ Timeout Members.
- `Core Developer`: ✅ Manage Webhooks, ✅ Manage Channels where needed.
- `Owner`: Administrator.
- `AegisForge`: ✅ Send Messages and Embed Links in `🚨┃mod-alerts` and `🧾┃case-log` only.

## 🔊 VOICE

| Channel | Type | Purpose | Member Permissions | Staff Permissions |
|---|---|---|---|---|
| `🔊┃general-vc` | Voice | Casual voice chat. | View, Connect, Speak, Use Voice Activity. | Move Members, Mute Members. |
| `🆘┃support-vc` | Voice | Live support calls when text is not enough. | View, Connect, Speak. | Move Members, Mute Members, Priority Speaker. |
| `🧪┃testing-vc` | Voice | Beta test sessions and live debugging. | Beta Tester/Contributor only. | Move Members, Mute Members. |
| `🔒┃staff-vc` | Voice | Staff-only calls. | Staff only. | Full voice moderation. |

Recommended voice permissions:

- Disable `Use Soundboard` for `@everyone` if it gets noisy.
- Allow `Stream` in support/testing channels if screen sharing is useful.
- Keep `Priority Speaker` limited to Staff.

## 🎭 Role Layout

Use colored roles, but keep the hierarchy clean. Higher roles should appear above lower roles.

| Order | Role | Purpose | Recommended Color | Key Permissions |
|---:|---|---|---|---|
| 1 | `👑 Owner` | Project/server owner. | Gold | Administrator. |
| 2 | `🧠 Core Developer` | Code, deploys, infrastructure, release authority. | Cyan | Manage Server, Manage Channels, Manage Webhooks, Manage Messages, Manage Roles below them. |
| 3 | `🛡️ Staff` | Moderation and support operations. | Blue | Manage Messages, Timeout Members, Manage Threads, Kick/Ban only if trusted. |
| 4 | `🆘 Support Helper` | Trusted helpers for setup questions. | Green | Manage Threads, Pin Messages, no moderation powers by default. |
| 5 | `🧪 Beta Tester` | Users testing beta builds. | Purple | Access beta/staging channels. |
| 6 | `🧑‍💻 Contributor` | Code/docs/design/test contributors. | Teal | Access development channels. |
| 7 | `🏅 Supporter` | Helpful community members, voters, boosters. | Pink | Cosmetic role, optional media perms. |
| 8 | `👤 Member` | Default community member. | Gray | Standard public access. |
| 9 | `🤖 AegisForge` | Production bot role. | Cyan | Bot permissions listed below. |
| 10 | `🧪 AegisForge Beta` | Staging/test bot role. | Purple | Testing-only bot permissions. |

## 🤖 Bot Permission Requirements

For easiest setup, the invite currently uses broad permissions. For a stricter production server, grant AegisForge these permissions:

### Required

- View Channels
- Send Messages
- Send Messages in Threads
- Embed Links
- Attach Files
- Read Message History
- Use Slash Commands / Use Application Commands
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
- View Audit Log if you want richer moderation diagnostics later.
- Manage Webhooks only if you use webhook-based status or log integrations.

### Role hierarchy warning

Place the `🤖 AegisForge` role:

- Above roles it needs to assign/remove.
- Below Owner/Core Developer/Staff roles.
- Above muted/test roles if the bot manages them.

If role commands fail, role hierarchy is usually the reason.

## 🧾 Support Templates

### 🆘 Support Request

```text
What are you trying to do?

What happened instead?

Command or feature:

Server permissions checked? yes/no

Screenshots or error logs:

```

### 🐞 Bug Report

```text
Bug summary:

Steps to reproduce:
1.
2.
3.

Expected behavior:

Actual behavior:

Command or page affected:

Approximate time it happened:

Error message or screenshot:

```

### 🧪 Beta Feedback

```text
Build or version tested:

Feature tested:

What felt good:

What felt confusing:

What broke or felt unreliable:

Suggested improvement:

```

### 🛒 Shop Item Request

Use this in `🛒┃shop-talk` or `💡┃feature-requests`.

```text
Item name:

Category: Profile / Cosmetic / Collectible / Boost / Utility / Community / Limited

Suggested price:

Rarity: Common / Rare / Epic / Legendary / Mythic

What should it do or represent?

Should it be permanent, seasonal, or limited?

```

## 🛒 Global Shop Notes

The bot now has `/economy shop`.

- The catalog is global across all servers.
- It is rendered from the current bot shop list.
- New items added to the catalog automatically appear in the command after deployment.
- Current categories: `Profile`, `Cosmetic`, `Collectible`, `Boost`, `Utility`, `Community`, `Limited`.
- The command is currently a catalog/browser command. Purchase and inventory hooks can be added later.

Recommended public channel: `🛒┃shop-talk`.

Recommended staff workflow:

1. Users suggest items in `🛒┃shop-talk` or `💡┃feature-requests`.
2. Staff tags accepted ideas.
3. Core Developer adds the item to the global shop catalog.
4. Announce item drops in `🧾┃changelog` or `📢┃announcements`.

## 🚦 Staff Triage Flow

1. Identify the area: bot command, website, API, deployment, database, docs, shop, or permissions.
2. Ask for exact command usage, page URL, screenshot, and timestamp.
3. Check common causes first: missing permissions, role hierarchy, slash command sync, cooldowns, or user input.
4. If reproducible, move it to `🐞┃bug-reports` or create a GitHub issue.
5. If it is beta-specific, move it to `🧪┃beta-feedback`.
6. If it is a known issue, add it to `🧷┃known-issues`.
7. If it affects uptime or deployment, post a short update in `🟢┃status`.

## 🔗 Useful Links

- **Invite Bot:** https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&scope=bot%20applications.commands
- **Website:** https://aegisforge-vert.vercel.app
- **GitHub:** https://github.com/watispro5212/AegisForge
- **Top.gg:** https://top.gg/bot/1500582485367722004
- **Support Server:** https://discord.gg/8p5Epc8Qd8

## ✅ Launch Checklist

- Create all categories and channels in the listed order.
- Apply category-level permissions first.
- Lock `📜┃rules`, `📢┃announcements`, `🟢┃status`, `🧾┃changelog`, `🚧┃maintenance`, `❓┃faq`, and `🧷┃known-issues`.
- Add pinned templates to support, bug reports, beta feedback, feature requests, and shop talk.
- Put `🤖 AegisForge` high enough in role hierarchy.
- Test `/help`, `/ping`, `/stats`, `/economy balance`, `/economy shop`, `/leveling rank`, and moderation commands.
- Configure `/logs`, `/welcome`, `/autorole`, `/prefix`, and `/settings`.
- Run one test incident post in `🟢┃status`.
- Invite beta testers only after the testing channels and rules are ready.

## 🧠 Final Standard

AegisForge should feel fast, practical, and trustworthy. The support server should match that: clear channels, sane permissions, useful templates, and a little personality in the places where it helps.
