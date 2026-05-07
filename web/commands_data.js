const commandsData = [
    {
        category: "Moderation",
        icon: "🛡️",
        commands: [
            { name: "/ban", desc: "Ban a user from the server with an optional message purge window (0–7 days).", usage: "/ban user:@user reason:text days:int" },
            { name: "/unban", desc: "Unban a user by their Discord ID.", usage: "/unban id:string" },
            { name: "/kick", desc: "Kick a member from the server with a reason.", usage: "/kick user:@user reason:text" },
            { name: "/timeout", desc: "Temporarily timeout a member for 1–40,320 minutes.", usage: "/timeout user:@user duration:int reason:text" },
            { name: "/mute", desc: "Alias for timeout — mutes a member for 60 minutes by default.", usage: "/mute user:@user reason:text" },
            { name: "/unmute", desc: "Remove an active timeout from a member.", usage: "/unmute user:@user" },
            { name: "/warn", desc: "Issue a formal warning that is logged to the database with a case number.", usage: "/warn user:@user reason:text" },
            { name: "/purge", desc: "Bulk delete up to 100 messages from the current channel.", usage: "/purge amount:int" },
            { name: "/slowmode", desc: "Set the channel's slowmode rate limit in seconds.", usage: "/slowmode seconds:int" },
            { name: "/lock", desc: "Lock the current channel by denying @everyone the ability to send messages.", usage: "/lock" },
            { name: "/unlock", desc: "Remove the lock on the current channel.", usage: "/unlock" }
        ]
    },
    {
        category: "Economy",
        icon: "💰",
        commands: [
            { name: "/economy balance", desc: "Check your wallet, bank, and total net worth. Can view another user's balance.", usage: "/economy balance user:@user" },
            { name: "/economy daily", desc: "Claim your daily reward of $500. Resets every 24 hours.", usage: "/economy daily" },
            { name: "/economy work", desc: "Work a job to earn $50–$200. Has a 30-minute cooldown between shifts.", usage: "/economy work" },
            { name: "/economy beg", desc: "Beg for spare change — earn $0–$50 with no cooldown.", usage: "/economy beg" },
            { name: "/economy search", desc: "Search random locations for loose cash ($10–$150).", usage: "/economy search" },
            { name: "/economy pay", desc: "Send money from your wallet to another user.", usage: "/economy pay user:@user amount:int" },
            { name: "/economy deposit", desc: "Move funds from your wallet into the bank for safekeeping.", usage: "/economy deposit amount:int" },
            { name: "/economy withdraw", desc: "Pull funds from your bank back into your wallet.", usage: "/economy withdraw amount:int" },
            { name: "/economy slots", desc: "Spin the Hyperforge Slot Machine. ~81.5% win rate. Minimum bet: $10.", usage: "/economy slots bet:int" },
            { name: "/economy rob", desc: "Attempt to steal 10–50% of a user's wallet. 40% success rate — fail and pay a $200 fine.", usage: "/economy rob user:@user" },
            { name: "/economy leaderboard", desc: "View the top 10 wealthiest members in the server or globally.", usage: "/economy leaderboard global:bool" },
            { name: "/economy global_leaderboard", desc: "View the top 10 wealthiest members across all servers.", usage: "/economy global_leaderboard" },
            { name: "/economy gamble_info", desc: "View the full slot machine payout table and FairForge™ odds.", usage: "/economy gamble_info" }
        ]
    },
    {
        category: "Leveling",
        icon: "📈",
        commands: [
            { name: "/leveling rank", desc: "View your current level, XP, and progress to the next level.", usage: "/leveling rank user:@user" },
            { name: "/leveling leaderboard", desc: "View the top 10 most active members ranked by XP.", usage: "/leveling leaderboard" }
        ]
    },
    {
        category: "Utility",
        icon: "🛠️",
        commands: [
            { name: "/ping", desc: "Check the bot's gateway latency in milliseconds.", usage: "/ping" },
            { name: "/server", desc: "Display detailed server statistics — member count, channels, boost level, and more.", usage: "/server" },
            { name: "/user", desc: "Look up a user's Discord profile, account age, and bot status.", usage: "/user user:@user" },
            { name: "/avatar", desc: "Retrieve a user's full-resolution avatar.", usage: "/avatar user:@user" },
            { name: "/uptime", desc: "View how long the bot has been online in the current session.", usage: "/uptime" },
            { name: "/stats", desc: "View global bot statistics — server count, user count, and uptime.", usage: "/stats" },
            { name: "/embed", desc: "Create and send a custom embed message to the current channel.", usage: "/embed title:text description:text color:hex" },
            { name: "/math", desc: "Evaluate a mathematical expression (supports +, -, *, /, ^, and more).", usage: "/math expression:text" },
            { name: "/qr", desc: "Generate a QR code for any text or URL.", usage: "/qr data:text" },
            { name: "/timestamp", desc: "Generate all Discord timestamp formats for a given UNIX timestamp.", usage: "/timestamp unix:int" },
            { name: "/timer", desc: "Acknowledge a timer for N minutes (reminder system).", usage: "/timer minutes:int" },
            { name: "/translate", desc: "Translate text to a target language (requires API key configuration).", usage: "/translate text:text target:string" },
            { name: "/dictionary", desc: "Look up a word definition via Wiktionary.", usage: "/dictionary word:string" },
            { name: "/worldclock", desc: "Show the current time in London, New York, and Tokyo.", usage: "/worldclock" },
            { name: "/poll", desc: "Create a simple yes/no reaction poll in the current channel.", usage: "/poll question:text" },
            { name: "/help", desc: "Display a full list of command categories and commands.", usage: "/help command:string" },
            { name: "/crypto", desc: "Look up a cryptocurrency symbol (market data display).", usage: "/crypto symbol:string" }
        ]
    },
    {
        category: "Fun",
        icon: "🎉",
        commands: [
            { name: "/fun coinflip", desc: "Flip a coin — heads or tails.", usage: "/fun coinflip" },
            { name: "/fun dice", desc: "Roll a die with a custom number of sides (defaults to d6).", usage: "/fun dice sides:int" },
            { name: "/fun eightball", desc: "Ask the Magic 8-Ball a yes/no question.", usage: "/fun eightball question:text" },
            { name: "/fun joke", desc: "Get a random programming joke.", usage: "/fun joke" },
            { name: "/fun fact", desc: "Get a random interesting fact.", usage: "/fun fact" },
            { name: "/fun choose", desc: "Randomly choose between options separated by commas.", usage: "/fun choose options:text" },
            { name: "/fun trivia", desc: "Get a trivia question with multiple choice answers (spoiler-tagged answer).", usage: "/fun trivia" },
            { name: "/fun ship", desc: "Check the compatibility percentage between two users.", usage: "/fun ship user1:@user user2:@user" },
            { name: "/fun rate", desc: "Rate anything out of 10 with a verdict.", usage: "/fun rate thing:text" },
            { name: "/fun roast", desc: "Deliver a light-hearted roast to a server member.", usage: "/fun roast user:@user" },
            { name: "/fun compliment", desc: "Give a genuine compliment to a server member.", usage: "/fun compliment user:@user" },
            { name: "/fun mock", desc: "Transform text into SpOnGeBoB mOcKiNg CaSe.", usage: "/fun mock text:text" },
            { name: "/fun reverse", desc: "Reverse any text string.", usage: "/fun reverse text:text" },
            { name: "/fun ascii", desc: "Display text as large spaced ASCII characters (max 10 chars).", usage: "/fun ascii text:text" },
            { name: "/fun meme", desc: "Get a random curated meme image.", usage: "/fun meme" },
            { name: "/fun cat", desc: "Get a random cat image.", usage: "/fun cat" },
            { name: "/fun dog", desc: "Get a random dog image.", usage: "/fun dog" },
            { name: "/fun fox", desc: "Get a random fox image.", usage: "/fun fox" },
            { name: "/fun panda", desc: "Get a random panda image.", usage: "/fun panda" },
            { name: "/fun bird", desc: "Get a random bird image.", usage: "/fun bird" },
            { name: "/fun cookie", desc: "Give a virtual cookie to someone.", usage: "/fun cookie user:@user" },
            { name: "/fun hug", desc: "Give someone a warm hug.", usage: "/fun hug user:@user" },
            { name: "/fun pat", desc: "Pat someone on the head.", usage: "/fun pat user:@user" },
            { name: "/fun kiss", desc: "Give someone a kiss.", usage: "/fun kiss user:@user" },
            { name: "/fun slap", desc: "Slap someone (all in good fun).", usage: "/fun slap user:@user" }
        ]
    },
    {
        category: "Config",
        icon: "⚙️",
        commands: [
            { name: "/logs", desc: "Set the moderation log channel for audit events.", usage: "/logs channel:#channel" },
            { name: "/welcome", desc: "Set the welcome channel and message template ({user}, {server} supported).", usage: "/welcome channel:#channel message:text" },
            { name: "/autorole", desc: "Set a role to automatically assign to new members on join.", usage: "/autorole role:@role" },
            { name: "/prefix", desc: "Change the bot's legacy prefix for text commands (default: !).", usage: "/prefix new:string" },
            { name: "/settings", desc: "View all current server configuration settings.", usage: "/settings" }
        ]
    },
    {
        category: "Roles",
        icon: "🎭",
        commands: [
            { name: "/role add", desc: "Assign a role to a server member.", usage: "/role add user:@user role:@role" },
            { name: "/role remove", desc: "Remove a role from a server member.", usage: "/role remove user:@user role:@role" },
            { name: "/role list", desc: "List all roles in the server with their IDs.", usage: "/role list" }
        ]
    },
    {
        category: "Reminders",
        icon: "⏰",
        commands: [
            { name: "/remind create", desc: "Set a reminder for yourself that fires after N minutes.", usage: "/remind create minutes:int message:text" }
        ]
    }
];
