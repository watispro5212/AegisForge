const commandsData = [
    {
        category: "mod stuff",
        icon: "🛡️",
        commands: [
            { name: "/ban", desc: "ban someone from the server.", usage: "/ban user:@user reason:text days:int" },
            { name: "/unban", desc: "Unban a user by ID.", usage: "/unban id:string" },
            { name: "/kick", desc: "kick someone out.", usage: "/kick user:@user reason:text" },
            { name: "/timeout", desc: "Mute a member for a specified duration.", usage: "/timeout user:@user duration:string reason:text" },
            { name: "/warn", desc: "Issue a formal warning logged to the database.", usage: "/warn user:@user reason:text" },
            { name: "/warnings", desc: "View all warnings for a user.", usage: "/warnings user:@user" },
            { name: "/delwarn", desc: "Delete a specific warning by ID.", usage: "/delwarn id:string" },
            { name: "/clearwarns", desc: "Clear all warnings for a user.", usage: "/clearwarns user:@user" },
            { name: "/purge", desc: "Bulk delete up to 100 messages.", usage: "/purge amount:int user:@user" },
            { name: "/lock", desc: "Lock the current channel.", usage: "/lock reason:text" },
            { name: "/unlock", desc: "Unlock the current channel.", usage: "" },
            { name: "/slowmode", desc: "Set channel slowmode duration.", usage: "/slowmode seconds:int" },
            { name: "/nuke", desc: "Clone and delete a channel to clear all messages.", usage: "" },
            { name: "/vmute", desc: "Voice mute a member.", usage: "/vmute user:@user" },
            { name: "/vunmute", desc: "Voice unmute a member.", usage: "/vunmute user:@user" },
            { name: "/deafen", desc: "Server deafen a member.", usage: "/deafen user:@user" },
            { name: "/undeafen", desc: "Server undeafen a member.", usage: "/undeafen user:@user" },
            { name: "/modlog", desc: "View recent moderation actions.", usage: "" }
        ]
    },
    {
        category: "money",
        icon: "💰",
        commands: [
            { name: "/balance", desc: "check how much money you got.", usage: "/balance user:@user" },
            { name: "/work", desc: "Perform a job to earn credits.", usage: "" },
            { name: "/daily", desc: "Claim your daily credit reward.", usage: "" },
            { name: "/beg", desc: "Beg for some spare change.", usage: "" },
            { name: "/pay", desc: "Transfer credits to another user.", usage: "/pay user:@user amount:int" },
            { name: "/deposit", desc: "Move credits from wallet to bank.", usage: "/deposit amount:int" },
            { name: "/withdraw", desc: "Move credits from bank to wallet.", usage: "/withdraw amount:int" },
            { name: "/slots", desc: "High-stakes slot machine with high win rates.", usage: "/slots bet:int" },
            { name: "/coinflip", desc: "Flip a coin to double your bet.", usage: "/coinflip choice:enum bet:int" },
            { name: "/blackjack", desc: "Play a game of blackjack against the house.", usage: "/blackjack bet:int" },
            { name: "/roulette", desc: "Bet on numbers or colors in roulette.", usage: "/roulette bet:int space:string" },
            { name: "/shop", desc: "View the server item shop.", usage: "" },
            { name: "/buy", desc: "Purchase an item from the shop.", usage: "/buy item:string" },
            { name: "/inventory", desc: "View your collected items.", usage: "" },
            { name: "/leaderboard", desc: "View top earners in the server or globally.", usage: "/leaderboard type:enum" },
            { name: "/fish", desc: "Go fishing for rewards.", usage: "" },
            { name: "/hunt", desc: "Go hunting for rare trophies.", usage: "" },
            { name: "/crime", desc: "Attempt a risky crime for big rewards.", usage: "" },
            { name: "/rob", desc: "Try to steal from another user's wallet.", usage: "/rob user:@user" }
        ]
    },
    {
        category: "tools",
        icon: "🛠️",
        commands: [
            { name: "/ping", desc: "is the bot laggy?", usage: "" },
            { name: "/server", desc: "Display detailed server information.", usage: "" },
            { name: "/user", desc: "Look up information on any user.", usage: "/user user:@user" },
            { name: "/avatar", desc: "Retrieve a user's full-resolution avatar.", usage: "/avatar user:@user" },
            { name: "/uptime", desc: "View bot uptime and version details.", usage: "" },
            { name: "/timestamp", desc: "Generate Discord timestamp formats.", usage: "/timestamp time:string" },
            { name: "/calc", desc: "Perform mathematical calculations.", usage: "/calc expression:text" },
            { name: "/remind", desc: "Set a personal reminder.", usage: "/remind time:string note:text" },
            { name: "/weather", desc: "Get current weather for a city.", usage: "/weather city:text" },
            { name: "/translate", desc: "Translate text between languages.", usage: "/translate text:text to:string" },
            { name: "/wiki", desc: "Search Wikipedia for a topic.", usage: "/wiki query:text" },
            { name: "/poll", desc: "Create an interactive reaction poll.", usage: "/poll question:text options:text" },
            { name: "/shorten", desc: "Shorten a long URL.", usage: "/shorten url:string" },
            { name: "/qr", desc: "Generate a QR code for text or URLs.", usage: "/qr text:text" }
        ]
    },
    {
        category: "tunes",
        icon: "🎵",
        commands: [
            { name: "/play", desc: "Play audio from YouTube, Spotify, or Soundcloud.", usage: "/play query:text" },
            { name: "/skip", desc: "Skip the currently playing track.", usage: "" },
            { name: "/stop", desc: "Stop playback and clear the queue.", usage: "" },
            { name: "/pause", desc: "Pause the current track.", usage: "" },
            { name: "/resume", desc: "Resume a paused track.", usage: "" },
            { name: "/queue", desc: "View the current music queue.", usage: "" },
            { name: "/nowplaying", desc: "Show details of the current track.", usage: "" },
            { name: "/volume", desc: "Adjust playback volume (0-150).", usage: "/volume level:int" },
            { name: "/loop", desc: "Toggle loop mode for track or queue.", usage: "/loop mode:enum" },
            { name: "/shuffle", desc: "Shuffle the current queue.", usage: "" },
            { name: "/seek", desc: "Seek to a specific time in the track.", usage: "/seek time:string" },
            { name: "/lyrics", desc: "Fetch lyrics for the current or specified song.", usage: "/lyrics query:text" }
        ]
    },
    {
        category: "settings",
        icon: "⚙️",
        commands: [
            { name: "/config set", desc: "Change server-wide settings.", usage: "/config set option:enum value:any" },
            { name: "/config view", desc: "View current server configuration.", usage: "" },
            { name: "/config reset", desc: "Reset a configuration option to default.", usage: "/config reset option:enum" },
            { name: "/setup logs", desc: "Initialize moderation logging.", usage: "/setup logs channel:#channel" },
            { name: "/setup welcome", desc: "Configure welcome messages.", usage: "/setup welcome channel:#channel message:text" },
            { name: "/setup autorole", desc: "Set roles to give on join.", usage: "/setup autorole role:@role" },
            { name: "/prefix", desc: "Change the bot's custom prefix (Legacy).", usage: "/prefix new:string" },
            { name: "/dashboard", desc: "Get a link to the web configuration portal.", usage: "" }
        ]
    },
    {
        category: "roles",
        icon: "🎭",
        commands: [
            { name: "/role add", desc: "Assign a role to a member.", usage: "/role add user:@user role:@role" },
            { name: "/role remove", desc: "Remove a role from a member.", usage: "/role remove user:@user role:@role" },
            { name: "/role info", desc: "View detailed role metadata.", usage: "/role info role:@role" },
            { name: "/role list", desc: "List all roles in the server.", usage: "" },
            { name: "/role color", desc: "Change a role's color.", usage: "/role color role:@role hex:string" },
            { name: "/role rename", desc: "Rename an existing role.", usage: "/role rename role:@role name:text" },
            { name: "/reactionrole", desc: "Create a reaction-based role menu.", usage: "/reactionrole create channel:#channel message:text" }
        ]
    },
    {
        category: "fun stuff",
        icon: "🎮",
        commands: [
            { name: "/meme", desc: "Fetch a random meme from Reddit.", usage: "" },
            { name: "/joke", desc: "Tell a random programming or dad joke.", usage: "" },
            { name: "/cat", desc: "Get a random cat image.", usage: "" },
            { name: "/dog", desc: "Get a random dog image.", usage: "" },
            { name: "/8ball", desc: "Ask the magic 8-ball a question.", usage: "/8ball question:text" },
            { name: "/rps", desc: "Play Rock Paper Scissors with the bot.", usage: "/rps choice:enum" },
            { name: "/roll", desc: "Roll a dice (default 6-sided).", usage: "/roll sides:int" },
            { name: "/trivia", desc: "Start a trivia question.", usage: "" },
            { name: "/ship", desc: "Check compatibility between two users.", usage: "/ship user1:@user user2:@user" }
        ]
    },
    {
        category: "find things",
        icon: "🔍",
        commands: [
            { name: "/google", desc: "Search Google for information.", usage: "/google query:text" },
            { name: "/youtube", desc: "Search for YouTube videos.", usage: "/youtube query:text" },
            { name: "/twitch", desc: "Check if a Twitch streamer is live.", usage: "/twitch user:string" },
            { name: "/crypto", desc: "Get live cryptocurrency prices.", usage: "/crypto coin:string" },
            { name: "/stock", desc: "Get live stock market data.", usage: "/stock symbol:string" },
            { name: "/urban", desc: "Search Urban Dictionary.", usage: "/urban term:text" },
            { name: "/dictionary", desc: "Look up word definitions.", usage: "/dictionary word:string" }
        ]
    },
    {
        category: "levels",
        icon: "📈",
        commands: [
            { name: "/rank", desc: "View your current level and XP progress.", usage: "/rank user:@user" },
            { name: "/levels", desc: "Show the top ranked members in the server.", usage: "" },
            { name: "/xp add", desc: "Add XP to a member (Admin only).", usage: "/xp add user:@user amount:int" },
            { name: "/xp remove", desc: "Remove XP from a member.", usage: "/xp remove user:@user amount:int" },
            { name: "/levelrole add", desc: "Assign a role automatically at a specific level.", usage: "/levelrole add level:int role:@role" },
            { name: "/levelrole list", desc: "List all level-up rewards.", usage: "" },
            { name: "/level-config", desc: "Configure XP gain rates and announcement channel.", usage: "" }
        ]
    },
    {
        category: "gifts",
        icon: "🎁",
        commands: [
            { name: "/giveaway start", desc: "Start a new giveaway in the current channel.", usage: "/giveaway start duration:string winners:int prize:text" },
            { name: "/giveaway end", desc: "End an active giveaway prematurely.", usage: "/giveaway end message_id:string" },
            { name: "/giveaway reroll", desc: "Pick a new winner for an ended giveaway.", usage: "/giveaway reroll message_id:string" },
            { name: "/giveaway list", desc: "Show all active giveaways in the server.", usage: "" }
        ]
    },
    {
        category: "help tickets",
        icon: "🎫",
        commands: [
            { name: "/ticket setup", desc: "Create a ticket system with a reaction button.", usage: "/ticket setup channel:#channel category:id message:text" },
            { name: "/ticket add", desc: "Add a user to the current ticket.", usage: "/ticket add user:@user" },
            { name: "/ticket remove", desc: "Remove a user from the ticket.", usage: "/ticket remove user:@user" },
            { name: "/ticket close", desc: "Close and archive the current ticket.", usage: "/ticket close reason:text" },
            { name: "/ticket transcript", desc: "Generate a transcript of the ticket conversation.", usage: "" }
        ]
    },
    {
        category: "links",
        icon: "🔗",
        commands: [
            { name: "/github repo", desc: "Get information about a GitHub repository.", usage: "/github repo name:string" },
            { name: "/github user", desc: "Look up a GitHub user's profile.", usage: "/github user name:string" },
            { name: "/twitter user", desc: "Get information about a Twitter profile.", usage: "/twitter user name:string" },
            { name: "/steam profile", desc: "Look up a Steam account.", usage: "/steam profile id:string" },
            { name: "/minecraft server", desc: "Check status of a Minecraft server.", usage: "/minecraft server ip:string" },
            { name: "/roblox user", desc: "Look up a Roblox profile.", usage: "/roblox user name:string" }
        ]
    },
    {
        category: "bot ai",
        icon: "🤖",
        commands: [
            { name: "/ask", desc: "Interact with the Aegis AI engine.", usage: "/ask prompt:text" },
            { name: "/image generate", desc: "Generate an image from a text prompt.", usage: "/image prompt:text" },
            { name: "/summarize", desc: "Summarize a long piece of text or a URL.", usage: "/summarize text:text" },
            { name: "/code debug", desc: "Ask the AI to help debug a code snippet.", usage: "/code language:string code:text" },
            { name: "/translate ai", desc: "High-accuracy AI translation.", usage: "/translate target:string text:text" }
        ]
    },
    {
        category: "games",
        icon: "🎮",
        commands: [
            { name: "/valorant", desc: "Show Valorant player stats.", usage: "/valorant user:string tag:string" },
            { name: "/apex", desc: "Look up Apex Legends player stats.", usage: "/apex platform:string name:string" },
            { name: "/fortnite", desc: "Show Fortnite player stats.", usage: "/fortnite user:string" },
            { name: "/league", desc: "Look up League of Legends summoner info.", usage: "/league name:string region:string" },
            { name: "/csgo", desc: "Show CS2/CS:GO player stats.", usage: "/csgo id:string" }
        ]
    },
    {
        category: "safety",
        icon: "🛡️",
        commands: [
            { name: "/antiraid enable", desc: "Turn on advanced raid protection.", usage: "/antiraid sensitivity:low|med|high" },
            { name: "/verify setup", desc: "Set up a verification system to prevent bots.", usage: "/verify channel:#channel role:@role" },
            { name: "/lockdown", desc: "Immediately lock all public channels in the server.", usage: "" },
            { name: "/audit logs", desc: "View detailed internal Aegis security logs.", usage: "/audit limit:int" },
            { name: "/scan members", desc: "Scan recent joins for suspicious accounts.", usage: "" }
        ]
    },
    {
        category: "admin stuff",
        icon: "👑",
        commands: [
            { name: "/webhook create", desc: "Create a new webhook in a channel.", usage: "/webhook name:string channel:#channel" },
            { name: "/audit export", desc: "Export server audit logs to CSV.", usage: "" },
            { name: "/subscription", desc: "Manage your AegisForge premium subscription.", usage: "" },
            { name: "/backup create", desc: "Create a snapshot of server settings and roles.", usage: "" },
            { name: "/template apply", desc: "Apply a pre-configured server template.", usage: "/template name:string" }
        ]
    }
];
