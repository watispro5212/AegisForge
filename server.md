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
   - **Permissions:** `Administrator` (Bypasses all channel permissions).
   - **Display:** Display separately from online members.

2. **🛠️ Developer**
   - **Color:** `#ff5722` (Neon Orange)
   - **Permissions:** `Administrator`
   - **Display:** Display separately.

3. **📢 Community Manager**
   - **Color:** `#e91e63` (Neon Pink)
   - **Permissions:** `Manage Server`, `Manage Roles`, `Manage Channels`, `Manage Messages`, `Kick Members`, `Ban Members`, `Mention @everyone`.
   - **Display:** Display separately.

4. **🛡️ Aegis Vanguard** (Moderators/Support Team)
   - **Color:** `#4caf50` (Green)
   - **Permissions:** `Manage Messages`, `Kick Members`, `Ban Members`, `Manage Threads`, `Mute/Deafen Members`, `Move Members`.
   - **Display:** Display separately.

5. **🤖 AegisForge** (The Bot)
   - **Color:** `#ffffff`
   - **Permissions:** `Administrator` (Ensure this role is physically above the Member and Vanguard roles).

6. **✨ Premium Supporter**
   - **Color:** `#ffea00` (Gold)
   - **Permissions:** Standard Member permissions + `Attach Files`, `Embed Links`, `Use External Emojis`, `Change Nickname`.

7. **🌟 Contributor** (Open source contributors)
   - **Color:** `#9c27b0` (Purple)
   - **Permissions:** Standard Member permissions + `Attach Files`.

8. **🧪 Beta Tester**
   - **Color:** `#00bcd4` (Cyan)
   - **Permissions:** Base member perms.

9. **🎨 Content Creator**
   - **Color:** `#e91e63` (Pink)
   - **Permissions:** Base member perms + ability to bypass slowmode.

10. **🚀 Server Booster**
    - **Color:** `#f47fff` (Light Pink)
    - **Permissions:** Granted by Discord automatically.

11. **🎉 Event Winner**
    - **Color:** `#ffc107` (Amber)

12. **🤝 Partner**
    - **Color:** `#1abc9c` (Turquoise)
    - **Permissions:** Base member perms + ability to post links in designated channels.

13. **🐛 Bug Hunter**
    - **Color:** `#e67e22` (Orange)
    - **Permissions:** Access to private #bug-testing channels.

14. **🌍 Translator**
    - **Color:** `#3498db` (Blue)
    - **Permissions:** Recognition for helping localize AegisForge commands.

15. **✅ Verified**
    - **Color:** `#bbbbbb` (Grey)
    - **Permissions:** View Channels, Send Messages, Read Message History.

16. **👥 Member** (Default auto-assigned role)
    - **Color:** Default
    - **Permissions:** `View Channels`, `Send Messages`, `Read Message History`, `Add Reactions`, `Connect`, `Speak`. (NO @everyone/here mentions).

17. **🔔 Notification Squad** (Self-assigned ping role)
    - **Color:** Default
    - **Permissions:** No extra permissions, used only to mention for minor announcements.

18. **🔇 Muted**
    - **Color:** `#000000` (Black)
    - **Permissions:** Explicitly denies `Send Messages`, `Add Reactions`, `Speak`.

---

## 3. Categories & Channels Setup (With Detailed Permissions)

### 📌 INFORMATION

*Category Permissions:* `@everyone`: `View Channel` ✅, `Send Messages` ❌, `Add Reactions` ❌ | `Aegis Vanguard`: `Send Messages` ✅

- **#👋・welcome**
  - *Permissions:* Syncs with category.
- **#📢・announcements**
  - *Permissions:* Syncs with category.
- **#📜・rules**
  - *Permissions:* Syncs with category.
- **#🔄・updates**
  - *Permissions:* Syncs with category.
- **#📊・polls**
  - *Permissions:* Syncs with category. `@everyone`: `Add Reactions` ✅
- **#🎭・get-roles**
  - *Permissions:* Syncs with category. `@everyone`: `Add Reactions` ✅
- **#📡・status**
  - *Permissions:* Syncs with category. Receives bot heartbeat and server count updates via webhook.

### 💬 COMMUNITY

*Category Permissions:* `@everyone`: `View Channel` ✅, `Send Messages` ✅, `Read Message History` ✅, `Attach Files` ❌

- **#💬・general**
  - *Permissions:* Syncs with category.
- **#🤖・bot-commands**
  - *Permissions:* Syncs with category. `@everyone`: `Use Application Commands` ✅
- **#💡・suggestions**
  - *Permissions:* Syncs with category. `@everyone`: `Create Public Threads` ✅
- **#✨・showcase**
  - *Permissions:* Syncs with category. `@everyone`: `Attach Files` ✅, `Embed Links` ✅
- **#🎨・media**
  - *Permissions:* Syncs with category. `@everyone`: `Attach Files` ✅, `Embed Links` ✅
- **#🎲・off-topic**
  - *Permissions:* Syncs with category.
- **#🐸・memes**
  - *Permissions:* Syncs with category. `@everyone`: `Attach Files` ✅, `Embed Links` ✅

### 🛠️ SUPPORT

*Category Permissions:* `@everyone`: `View Channel` ✅, `Send Messages` ❌

- **#❓・faq**
  - *Permissions:* Syncs with category.
- **#🎫・open-a-ticket**
  - *Permissions:* Syncs with category.
- **#💬・peer-support**
  - *Permissions:* Syncs with category. `@everyone`: `Send Messages` ✅
- **#🐛・bug-reports**
  - *Permissions:* Forum Channel. `@everyone`: `Send Messages` ✅, `Create Posts` ✅

### 🧪 BETA TESTING (Hidden)

*Category Permissions:* `@everyone`: `View Channel` ❌ | `Beta Tester`: `View Channel` ✅, `Send Messages` ✅

- **#🧪・beta-chat**
  - *Permissions:* Syncs with category.
- **#📝・beta-feedback**
  - *Permissions:* Syncs with category.

### 🔐 STAFF (Hidden)

*Category Permissions:* `@everyone`: `View Channel` ❌ | `Aegis Vanguard`: `View Channel` ✅, `Send Messages` ✅

- **#🛡️・staff-chat**
  - *Permissions:* Syncs with category.
- **#👑・admin-chat**
  - *Permissions:* `Aegis Vanguard`: `View Channel` ❌ | `Community Manager`: `View Channel` ✅
- **#📂・mod-logs**
  - *Permissions:* Syncs with category. `Aegis Vanguard`: `Send Messages` ❌ (Bot only)
- **#🚨・bot-alerts**
  - *Permissions:* Syncs with category. `Aegis Vanguard`: `Send Messages` ❌ (Bot only)

### 🎙️ VOICE CHANNELS

*Category Permissions:* `@everyone`: `View Channel` ✅, `Connect` ✅, `Speak` ✅, `Video` ❌

- **🔊 General Lounge**
  - *Permissions:* Syncs with category.
- **🔊 Gaming Lounge 1**
  - *Permissions:* Syncs with category.
- **🔊 Gaming Lounge 2**
  - *Permissions:* Syncs with category.
- **🎥 Stream Room**
  - *Permissions:* Syncs with category. `@everyone`: `Video` ✅
- **🎧 Support Waiting Room**
  - *Permissions:* Syncs with category. `@everyone`: `Speak` ❌ (Staff pulls users out)
- **🔒 Staff Meeting**
  - *Permissions:* `@everyone`: `View Channel` ❌ | `Aegis Vanguard`: `View Channel` ✅, `Connect` ✅

---

## 4. Required Bots

1. **🤖 AegisForge**: Your own custom bot! Handles advanced moderation, custom webhook logging (in `#mod-logs` and `#status`), utility commands, and dynamic embeds. Recently updated with a new `/help` command!
2. **👋 Welcomer**: Dedicated to generating high-quality image welcome cards when members join, logging member leaves, and automatically sending a direct message to new users with server rules.
3. **🎫 Ticket Tool**: Creates a highly professional support ticket system. Users click a button in `#support` to open a private thread that only they and the `Aegis Vanguard`/`Aegis Sentinel` roles can see.
4. **⚔️ Arcane**: A powerful leveling and economy bot that rewards members with XP for being active in text and voice channels. Automatically assigns the `⭐ VIP Member` role when they reach level 10.
5. **🐢 Carl-bot**: Used for high-end reaction roles, custom logging (separate from Aegis), and advanced automated moderation triggers that require complex regex patterns.
6. **🦕 Dyno**: A secondary moderation layer and automated message cleaning utility. Perfect for massive spam events where multiple bots are needed to sweep channels.
7. **📊 Member Count**: Creates a dynamic voice channel at the top of the server list that displays the current member count in real-time.

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

### 🛰️ Status Channel

- **Name:** #status
- **Type:** Text Channel
- **Category:** Information / System
- **Permissions:**
  - @everyone: View Channel, Read Message History
  - @everyone: Send Messages (DENY)
  - Bot: View Channel, Send Messages, Embed Links
- **Purpose:** Receives automated webhook pings whenever the AegisForge bot goes online or joins a new server. Allows users to monitor the exact uptime and scale of the ecosystem.
