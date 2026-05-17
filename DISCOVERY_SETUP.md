# AegisForge Discord Discovery Setup

Use this checklist to prepare AegisForge for Discord App Directory / Discovery.

## 1. Discovery-Safe Deploy

Discord's App Directory content requirements disallow gambling-adjacent or addictive behavior. Before opting into Discovery, deploy the bot with:

```env
DISCOVERY_SAFE_COMMANDS=true
```

Then restart the bot so global slash commands are re-registered without the risky economy subcommands:

- `slots`
- `blackjack`
- `coinflip`
- `dice`
- `gamble_info`
- `rob`
- `crime`
- `hunt`

The safe `/economy` group still keeps balance, daily, work, pay, leaderboards, banking, shop, inventory, profile, beg, search, and work list.

## 2. Developer Portal Checklist

Open the Developer Portal:

https://discord.com/developers/applications/1500582485367722004

Complete:

- App Verification first.
- General Information: add 1-5 tags.
- OAuth2: generate and save the default install URL.
- Discovery -> Discovery Settings: fill support server, description, links, languages, and media.
- Discovery -> Discovery Status: wait for all green checks, then click Enable Discovery.

Discord says it can take up to 24 hours to appear after enabling.

## 3. Required URLs

Website:

https://aegisforge-vert.vercel.app/

Privacy Policy:

https://aegisforge-vert.vercel.app/privacy.html

Terms of Service:

https://aegisforge-vert.vercel.app/terms.html

Support Server:

https://discord.gg/HbmafcgjNa

GitHub:

https://github.com/watispro5212/aegisforge

Default Install URL:

https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&scope=bot+applications.commands

## 4. Suggested Tags

Use up to 5:

- Moderation
- Utilities
- Economy
- Logging
- Leveling

If Discord provides a curated tag list instead of free text, choose the closest available matches.

## 5. Short Description

AegisForge is a fast Rust-powered Discord app for server moderation, raid protection, AutoMod, logging, leveling, utility commands, and server operations.

## 6. Full App Directory Description

AegisForge helps Discord communities run cleaner, safer, and faster servers.

Built in Rust, it combines real-time Sentinel anti-raid protection, modular AutoMod, moderation tools, configurable logs, welcome and goodbye automation, role utilities, XP leveling, rank-card customization, economy profiles, and practical utility commands in one app.

Key features:

- Sentinel anti-raid detection for abnormal join spikes.
- AutoMod modules for spam, invite links, caps, mentions, and blacklist phrases.
- Moderation commands for bans, kicks, timeouts, warnings, purges, locks, slowmode, cases, and incident reports.
- Message, member, and moderation logging.
- Leveling with leaderboards and customizable rank cards.
- Server economy profiles, shop inventory, banking, and daily/work rewards.
- Utility commands for ping, stats, user/server info, polls, reminders, weather, QR codes, timestamps, and more.

AegisForge is designed for server owners and staff who want powerful defaults, clear configuration, and reliable performance without juggling a pile of separate bots.

## 7. External Links

Add these in Discovery Settings:

- Website: https://aegisforge-vert.vercel.app/
- Commands: https://aegisforge-vert.vercel.app/commands.html
- Security: https://aegisforge-vert.vercel.app/security.html
- GitHub: https://github.com/watispro5212/aegisforge
- Top.gg: https://top.gg/bot/1500582485367722004

## 8. Media Carousel Ideas

Use up to 5 assets. Recommended order:

1. Sentinel raid detection / mod-log screenshot.
2. AutoMod status and blacklist screenshot.
3. Leveling rank-card customization screenshot.
4. Economy shop and inventory screenshot.
5. Live dashboard/status page screenshot.

Use clean screenshots with no private usernames, messages, IDs, tokens, or staff-only data.

## 9. Support Server Prep

Discord requires the support server to be a Community server.

Recommended channels:

- `#start-here`
- `#announcements`
- `#support`
- `#bug-reports`
- `#feature-requests`
- `#status`
- `#commands-help`

Pin a support template:

```text
Command:
Server ID:
What happened:
What you expected:
Permissions checked:
Screenshot or error:
```

## 10. Final Preflight

- `DISCOVERY_SAFE_COMMANDS=true` is deployed.
- Bot restarted after the env change.
- Global slash commands have refreshed.
- Privacy Policy and Terms pages are public.
- Support server invite works and points to a Community server.
- Description avoids unsafe, NSFW, gambling, or IP-infringing language.
- Media screenshots show only safe public/demo content.
- Discovery Status shows all green checks.
