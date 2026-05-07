# 🛡️ AegisForge Official Support Server Setup Guide

This document contains the exact blueprint for creating the ultimate Discord Support server for AegisForge. Follow these instructions carefully to set up your categories, channels, permissions, and roles.

## 1. Server Profile

- **Server Name:** `AegisForge HQ`
- **Server Description:** "The official development, support, and community hub for AegisForge — the ultimate high-performance Rust Discord bot."
- **Official Avatar:** `web/assets/logo.jpg` (Official bot pfp)
- **Official Banner:** `web/assets/banner.png` (Official bot banner)
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

### 📌 info

*permissions:* everyone can see. `staff` can talk.

- **#👋・welcome**
  - says hi to new people.
- **#📢・announcements**
  - updates and stuff.
- **#📜・rules**
  - read these or get banned lol.
- **#🔄・updates**
  - bot code updates.
- **#📊・polls**
  - vote on things.
- **#🎭・get-roles**
  - pick your roles.
- **#📡・status**
  - is the bot alive?

### 💬 COMMUNITY

*Category Permissions:* `@everyone`: `View Channel` ✅, `Send Messages` ✅, `Read Message History` ✅, `Attach Files` ❌

- **#💬・general**
  - *Description:* The main hub for community conversation. Keep it civil and respect the rules.
- **#🤖・bot-commands**
  - *Description:* The dedicated playground for testing AegisForge and other bots without cluttering general chat.
- **#💡・suggestions**
  - *Description:* Got an idea for AegisForge? Post it here for community discussion and developer review.
- **#✨・showcase**
  - *Description:* Share your server setups, custom embeds, or interesting ways you're using AegisForge.
- **#🎨・media**
  - *Description:* Images, videos, and creative content related to the community and the bot.
- **#🎲・off-topic**
  - *Description:* For conversations that don't fit in general—random chat, hobbies, and more.
- **#🐸・memes**
  - *Description:* The community meme repository. Keep it light, funny, and within SFW guidelines.

### 🛠️ SUPPORT

*Category Permissions:* `@everyone`: `View Channel` ✅, `Send Messages` ❌

- **#❓・faq**
  - *Description:* Quick answers to common questions about AegisForge setup and functionality.
- **#🎫・open-a-ticket**
  - *Description:* The private support portal. Click to open a ticket for direct assistance from the staff.
- **#💬・peer-support**
  - *Description:* Community-driven help where experienced users assist newcomers with setup.
- **#🐛・bug-reports**
  - *Description:* Forum channel for reporting issues. Use the standard template for faster resolution.

### 🧪 BETA TESTING (Hidden)

*Category Permissions:* `@everyone`: `View Channel` ❌ | `Beta Tester`: `View Channel` ✅, `Send Messages` ✅

- **#🧪・beta-chat**
  - *Description:* Private discussion for Beta Testers regarding upcoming experimental features.
- **#📝・beta-feedback**
  - *Description:* Detailed reports and usability feedback on current beta builds of AegisForge.

### 🔐 STAFF (Hidden)

*Category Permissions:* `@everyone`: `View Channel` ❌ | `Aegis Vanguard`: `View Channel` ✅, `Send Messages` ✅

- **#🛡️・staff-chat**
  - *Description:* Internal coordination and moderation discussion for the Aegis Vanguard team.
- **#👑・admin-chat**
  - *Description:* High-level management discussion for founders and community managers.
- **#📂・mod-logs**
  - *Description:* Automated audit feed of all moderation actions taken by AegisForge across the server.
- **#🚨・bot-alerts**
  - *Description:* Critical system alerts, API errors, and high-priority bot notifications.

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
