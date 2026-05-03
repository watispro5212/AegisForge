# context.md

## Project Name
**AegisForge**

## Bot Type
Discord utility, moderation, automation, and developer assistant bot written in Rust.

## One-Line Description
AegisForge is a fast, secure, and customizable Rust-powered Discord bot that helps servers stay organized, automated, and easy to manage.

## Short Public Description
AegisForge is a Rust Discord bot built for speed, reliability, and clean server automation. It provides moderation tools, utility commands, role management, logging, reminders, and developer-friendly features in a sleek modern package.

## Extended Description
AegisForge is designed as a modern all-purpose Discord bot for communities that want strong automation without unnecessary clutter. The bot should feel reliable, precise, and efficient, with a polished command structure and clear responses. It should be built in Rust for performance and stability, using a framework such as Poise or Serenity for slash commands, event handling, and extensibility.

The bot’s identity should be centered around the idea of a forged tool: strong, dependable, and built to last. It should support server administration, moderation actions, logging, reminders, role tools, onboarding flows, and optional developer utilities. The tone should be professional but not cold, with concise responses and a slight futuristic edge.

## Core Goals
- Provide reliable moderation and automation tools.
- Keep commands fast, organized, and easy to discover.
- Support slash commands as the primary interaction model.
- Use Rust to emphasize performance, safety, and maintainability.
- Present a polished brand that looks trustworthy and premium.

## Primary Use Cases
- Moderating spam, raids, and rule violations.
- Managing roles, welcome messages, and onboarding.
- Logging message edits, deletions, joins, leaves, and moderation actions.
- Scheduling reminders and recurring tasks.
- Providing utility commands like server info, user info, timestamps, and embeds.
- Offering developer/admin commands for configured servers.

## Target Audience
- Community servers.
- Gaming servers.
- Developer and tech communities.
- Small-to-medium Discord communities that want a clean, efficient admin bot.

## Personality
AegisForge should feel:
- Calm.
- Professional.
- Helpful.
- Precise.
- Slightly futuristic.
- Reliable under pressure.

It should never feel overly playful or chaotic. Its brand should suggest a tool that is engineered, not improvised.

## Naming Rationale
“Aegis” suggests protection and defense. “Forge” suggests craftsmanship, strength, and building durable systems. Together, the name implies a bot that protects communities and is carefully engineered in Rust.

## Tag Suggestions
Use tags that describe the app clearly and narrowly. Recommended tags:
- Moderation
- Automation
- Utility
- Developer Tools
- Community Management

If only a few tags are allowed, prioritize:
- Moderation
- Utility
- Automation

## Discord App Description
A good Discord app description should be short and clear. Use something like:

**AegisForge is a fast Rust Discord bot for moderation, automation, role management, and server utilities. Built for speed, stability, and clean control.**

## Feature Set
### Moderation
- Timeout, kick, ban, warn, and note system.
- Configurable moderation logs.
- Auto-moderation for spam, caps, mentions, invite links, and repeated messages.
- Appeal-friendly audit trails for moderation actions.

### Utility
- User info, server info, avatar lookup, role lookup.
- Timestamp formatting and time conversion.
- Ping, uptime, and latency commands.
- Embed builder or message formatting helpers.

### Automation
- Welcome and goodbye messages.
- Auto-role assignment.
- Reaction role support.
- Scheduled reminders.
- Temporary roles and temporary mute systems.

### Logging
- Message edits and deletes.
- Channel changes.
- Member joins, leaves, and nickname changes.
- Moderation event logs with timestamps and actor details.

### Developer Features
- Configurable command prefix fallback, though slash commands should be the default.
- Per-guild settings stored in a database.
- Feature toggles for admins.
- Structured error messages for easier debugging.

## Command Style
Commands should be:
- Short.
- Consistent.
- Easy to search in slash menus.
- Organized by category.

Example command categories:
- `/mod ban`
- `/mod timeout`
- `/util server`
- `/util user`
- `/config welcome`
- `/config logs`
- `/role add`
- `/role remove`
- `/remind create`

## Response Style
Bot replies should:
- Be concise by default.
- Use embeds for important actions.
- Confirm success clearly.
- Explain failures in plain language.
- Avoid unnecessary text walls.
- Use consistent formatting across commands.

## Technical Stack
### Language
- Rust

### Recommended Libraries
- Poise for slash-command-first bot architecture.
- Serenity for Discord API access and gateway handling.
- Tokio for async runtime.
- SQLx or SeaORM for persistent data.
- Redis optional for caching and rate limiting.
- Tracing for logs and observability.

### Architecture Notes
- Separate command logic, event handlers, database access, and configuration.
- Keep command modules small and focused.
- Use typed data models for guild settings.
- Store all server-specific settings in persistent storage.
- Prefer slash commands over message-prefix commands for discoverability and clarity, since modern Discord bot frameworks are optimized around them.

## Permissions Philosophy
Request only the permissions the bot actually needs.
- Read messages.
- Send messages.
- Embed links.
- Manage messages.
- Moderate members.
- Manage roles.
- View audit log if needed.
- Use application commands.

Avoid requesting overly broad permissions unless a feature genuinely needs them.

## Safety And Trust
- Never expose the bot token.
- Validate all admin-only commands.
- Log moderation actions securely.
- Avoid destructive commands without confirmation.
- Rate-limit spammy endpoints and repeated interactions.
- Handle missing permissions gracefully.

## Database Needs
Store:
- Guild configuration.
- Moderation cases.
- Reminder jobs.
- Auto-role mappings.
- Logging channel IDs.
- Feature toggles.
- Temporary punishments.
- User preferences if needed.

## Visual Identity
### Profile Picture
The profile picture should be a bold, simple emblem that reads well at very small sizes. Use a dark background with a metallic or electric accent. The icon should combine a stylized shield and an anvil or forge mark, with one central glowing line or rune to imply protection and power.

Design direction:
- Circular composition.
- Dark charcoal or near-black background.
- Silver, steel, or gunmetal main shape.
- Accent glow in cyan, blue, or ember orange.
- Minimal detail, high contrast, no text.
- Sharp edges, symmetrical layout.

The image should feel premium, technical, and trustworthy. It should not look cartoonish.

### Banner
The banner should expand the same brand into a wider scene. Picture a dark industrial-futuristic backdrop with subtle forge sparks, glowing metal lines, and a stylized network/grid pattern. The center can feature the AegisForge emblem enlarged and partially transparent, with light trails suggesting speed and automation.

Design direction:
- Wide horizontal layout.
- Deep navy, charcoal, and black base tones.
- Cyan or orange energy accents.
- Subtle smoke, sparks, or heat haze.
- A refined sci-fi forge aesthetic.
- Clean negative space so text can fit if needed.

The banner should communicate strength, precision, and engineering quality rather than chaos or gaming energy.

## Brand Colors
- Primary: Charcoal black.
- Secondary: Gunmetal gray.
- Accent 1: Electric cyan.
- Accent 2: Ember orange.
- Neutral: White or pale silver for text.

## Font Direction
Use a modern sans-serif or tech-style typeface for branding assets. The text should feel sharp and readable, not decorative.

## Example Elevator Pitch
AegisForge is the Rust bot that keeps your Discord server organized, protected, and efficient with clean moderation, smart automation, and reliable utilities.
