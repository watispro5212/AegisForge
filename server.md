# AegisForge Support Server Guide

This document is the operating guide for the official AegisForge Discord server. It is written for staff, testers, contributors, and community members so the server feels clear, useful, and easy to navigate.

## Server Purpose

The AegisForge server exists to support people who use, test, or contribute to the bot.

Primary goals:

- Help server owners install, configure, and troubleshoot AegisForge.
- Give beta testers a clean place to report bugs and usability issues.
- Announce new releases, incidents, and planned maintenance.
- Let users test commands without disrupting their own communities.
- Keep development feedback organized enough to act on.

## Recommended Server Identity

**Server name:** AegisForge Support  
**Short description:** Support, updates, testing, and community feedback for the AegisForge Discord bot.  
**Tone:** Sharp, helpful, calm, lightly funny, and SFW. Avoid low-effort chaos in support spaces.

Suggested welcome blurb:

> Welcome to AegisForge Support. Use this server for setup help, bug reports, beta feedback, command testing, and release updates. Check the rules first, then head to the channel that matches what you need.

## Channel Layout

Replace placeholder channel IDs with real Discord channel mentions after creating the channels.

### Information

- `#rules`  
  Short community rules, support expectations, and safety guidelines.

- `#announcements`  
  Release notes, important updates, new features, and policy changes. Staff-only posting.

- `#status`  
  Bot uptime notices, Fly.io deployment notes, website/API incidents, and known outages.

- `#changelog`  
  Detailed release summaries with fixed bugs, new commands, and migration notes.

- `#faq`  
  Answers for invite permissions, slash command sync delays, database setup, and common command questions.

### Support

- `#support`  
  General help for setup, permissions, commands, and configuration.

- `#bug-reports`  
  Reproducible bot or website bugs. Use the bug report template below.

- `#beta-feedback`  
  Detailed reports and usability feedback on current beta builds of AegisForge.

- `#feature-requests`  
  New command ideas, dashboard ideas, moderation workflow requests, and economy balancing feedback.

### Testing

- `#command-testing`  
  Slash command testing, embed previewing, and quick sanity checks.

- `#economy-testing`  
  Economy testing for `/economy daily`, `/economy work`, `/economy slots`, `/economy fish`, `/economy hunt`, and `/economy crime`.

- `#leveling-testing`  
  XP, rank, leaderboard, and rank card customization testing.

- `#automod-testing`  
  Controlled AutoMod checks. Keep test phrases harmless and SFW.

### Community

- `#general`  
  Normal community chat.

- `#showcase`  
  Screenshots of AegisForge setups, server configs, embed designs, and rank card styles.

- `#memes`  
  The community meme repository. Keep it light, funny, and within SFW guidelines.

## Role Layout

Suggested roles:

- `Owner` - Full server and project ownership.
- `Core Developer` - Maintainers with repository and deployment responsibility.
- `Staff` - Moderates the support server and triages user questions.
- `Support Helper` - Trusted community helpers who answer setup questions.
- `Beta Tester` - Users testing unreleased bot or website builds.
- `Contributor` - People who have contributed code, docs, design, reports, or testing.
- `Member` - Default community role.

Optional bot roles:

- `AegisForge` - The production bot.
- `AegisForge Beta` - A staging or test bot, if used.

## Rules

Recommended `#rules` content:

1. Keep everything SFW.
2. Be respectful. Critique bugs and ideas, not people.
3. Use the right channel for support, bugs, feedback, and command testing.
4. Do not spam commands outside testing channels.
5. Do not share tokens, database URLs, private keys, webhook URLs, or personal data.
6. Do not abuse bot commands, economy mechanics, or AutoMod testing.
7. Staff may remove content that disrupts support or makes the server harder to use.

## Support Request Template

Use this in `#support` when someone needs help:

```text
What are you trying to do?

What happened instead?

Command or feature:

Server permissions checked? yes/no

Screenshots or error logs:

```

## Bug Report Template

Use this in `#bug-reports`:

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

## Beta Feedback Template

Use this in `#beta-feedback`:

```text
Build or version tested:

Feature tested:

What felt good:

What felt confusing:

What broke or felt unreliable:

Suggested improvement:

```

## Staff Triage Flow

1. Confirm the affected area: bot command, website, API, deployment, database, or documentation.
2. Ask for exact command usage or page URL.
3. Check whether the issue is permissions, slash command sync, cooldowns, or a real bug.
4. If reproducible, move it to `#bug-reports` or create a GitHub issue.
5. If it is user error, answer clearly and add the question to `#faq` if it comes up often.
6. If it is an outage, post a short note in `#status`.

## Useful Links

- **Invite Bot:** https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&scope=bot%20applications.commands
- **Website:** https://aegisforge-vert.vercel.app
- **GitHub:** https://github.com/watispro5212/AegisForge
- **Top.gg:** https://top.gg/bot/1500582485367722004

## Launch Checklist

- Create all information, support, testing, and community channels.
- Replace placeholder text with real channel mentions where needed.
- Lock posting in `#announcements`, `#status`, and `#changelog` to staff.
- Enable slowmode in `#support` if traffic gets noisy.
- Add the support templates as pinned messages.
- Configure AegisForge logging, welcome channel, autorole, and prefix settings.
- Test `/help`, `/ping`, `/stats`, `/economy balance`, `/leveling rank`, and moderation commands.
- Invite beta testers only after rules and support templates are pinned.

## Closing Line

AegisForge should feel fast, practical, and easy to trust. The support server should match that: organized, friendly, and useful before it tries to be funny.
