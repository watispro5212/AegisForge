# 🛡️ AegisForge Official Support Server Setup Guide

This document contains the exact blueprint for creating the ultimate Discord Support server for AegisForge. Follow these instructions carefully to set up your categories, channels, permissions, and roles.

## 1. Server Profile

- **Server Name:** `AegisForge HQ`
- **Server Description:** "The official development, support, and community hub for AegisForge — the premium, high-performance Rust Discord bot."
- **Server Icon:** Use the `aegisforge_logo.png` we generated earlier.
- **System Messages Channel:** Set to a hidden `#server-logs` channel or disable them.

---

## 2. Roles & Permissions (Hierarchical Order)

*Ensure roles are listed exactly in this order in your server settings.*

1. **👑 Founder**
   - **Color:** `#00e5ff` (Neon Cyan)
   - **Permissions:** `Administrator`
   - **Display:** Display separately from online members.

2. **🛠️ Developer**
   - **Color:** `#ff5722` (Neon Orange)
   - **Permissions:** `Administrator`
   - **Display:** Display separately.

3. **🛡️ Aegis Vanguard** (Moderators/Support Team)
   - **Color:** `#4caf50` (Green)
   - **Permissions:** `Manage Messages`, `Kick Members`, `Ban Members`, `Manage Threads`, `Mute/Deafen Members`.
   - **Display:** Display separately.

4. **✨ Premium Supporter**
   - **Color:** `#ffea00` (Gold)
   - **Permissions:** Standard Member permissions + `Attach Files`, `Embed Links`, `Use External Emojis`.

5. **🤖 AegisForge** (The Bot)
   - **Color:** `#ffffff`
   - **Permissions:** `Administrator` (Ensure this role is physically above the Member and Vanguard roles).

6. **👥 Member** (Default auto-assigned role)
   - **Color:** Default
   - **Permissions:** `View Channels`, `Send Messages`, `Read Message History`, `Add Reactions`, `Connect`, `Speak`. (NO @everyone/here mentions).

7. **🔇 Muted**
   - **Color:** `#424242` (Dark Grey)
   - **Permissions:** Explicitly deny `Send Messages`, `Add Reactions`, `Connect` in all channel category settings.

---

## 3. Categories & Channels Setup

### 📌 INFORMATION (Read-only for Members)

- **#👋・welcome** (Read-only. Welcome messages go here)
- **#📢・announcements** (Read-only. Bot updates and downtime notices)
- **#📜・rules** (Read-only. See rules text below)
- **#🔄・updates** (Read-only. Patch notes and new features)
- **#📊・polls** (Read-only. Community voting and feedback)

### 💬 COMMUNITY

- **#💬・general** (Standard chat for all members)
- **#🤖・bot-commands** (Where users can test AegisForge commands)
- **#💡・suggestions** (Where users can suggest new features)
- **#✨・showcase** (For users to show off how they use the bot)
- **#🎲・off-topic** (Discussions unrelated to the bot)
- **#🐸・memes** (For sharing funny content)

### 🛠️ SUPPORT

- **#❓・faq** (Read-only. Common questions and answers)
- **#🎫・open-a-ticket** (Read-only. Ticket system panel)
- **#🐛・bug-reports** (Forum channel or standard channel for bug reporting)

### 🔐 STAFF (Hidden from standard Members)

- **#🛡️・staff-chat** (Private discussion for Vanguards and Developers)
- **#📂・mod-logs** (AegisForge's moderation logging channel)

### 🎙️ VOICE CHANNELS

- **🔊 General Lounge** (Standard voice chat)
- **🔊 Support Waiting Room** (For users needing live assistance)
- **🔊 Staff Meeting** (Hidden. For staff only)

---

## 4. Required Bots

1. **🤖 AegisForge**: Your own bot! It will handle all the moderation, logging (in `#mod-logs`), and utility commands.
2. **🎫 Ticket Tool** (or **Ticketmatic**): Use this in the `#🎫・open-a-ticket` channel. It allows users to click a button to open a private support channel with the `🛡️ Aegis Vanguard` team.

---

## 5. Texts & Prompts

### 📜 Rules Text (Post this in #📜・rules)

```text
**Welcome to AegisForge HQ! 🛡️**
To keep our community safe and organized, please follow these rules:

**1. Be Respectful:** No harassment, sexism, racism, or hate speech. Treat everyone with respect.
**2. No Spam or Self-Promotion:** Do not spam messages, ping staff unnecessarily, or advertise other Discord servers/bots without permission.
**3. Keep it SFW:** Absolutely no NSFW (Not Safe For Work) or suggestive content.
**4. Use the Right Channels:** Keep bot commands in <#BOT_COMMANDS_CHANNEL_ID> and support questions in <#TICKETS_CHANNEL_ID>.
**5. Do Not DM Staff:** If you need help with the bot, please open a ticket. Staff will not respond to unsolicited DMs.

*By participating in this server, you agree to these rules. The Aegis Vanguard reserves the right to moderate at their discretion.*
```

### 👋 Welcome Sign (Configure this in your bot or Welcome channel)

```text
**Welcome to AegisForge HQ, {user}! 🎉**

We're thrilled to have you here. AegisForge is the ultimate high-performance Rust Discord bot. 
- Grab your roles in <#ROLES_CHANNEL_ID>
- Read the <#RULES_CHANNEL_ID>
- Need help? Open a ticket in <#TICKETS_CHANNEL_ID>!

Enjoy your stay! 🛡️
```
