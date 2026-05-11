const commandsData = [
    {
        category: "Moderation",
        icon: "Shield",
        commands: [
            { name: "/ban", desc: "Ban a member with an optional message purge window.", usage: "/ban user:@user reason:text days:int" },
            { name: "/softban", desc: "Ban and immediately unban a member to remove recent messages.", usage: "/softban user:@user days:int reason:text" },
            { name: "/unban", desc: "Unban a user by Discord ID.", usage: "/unban id:string" },
            { name: "/kick", desc: "Kick a member with a logged reason.", usage: "/kick user:@user reason:text" },
            { name: "/timeout", desc: "Temporarily timeout a member.", usage: "/timeout user:@user duration:int reason:text" },
            { name: "/mute", desc: "Quick mute alias powered by Discord timeouts.", usage: "/mute user:@user reason:text" },
            { name: "/unmute", desc: "Remove a member's active timeout.", usage: "/unmute user:@user" },
            { name: "/warn", desc: "Create a persistent moderation warning.", usage: "/warn user:@user reason:text" },
            { name: "/purge", desc: "Bulk-delete recent messages in the current channel.", usage: "/purge amount:int" },
            { name: "/nuke", desc: "Recreate the current channel to clear all history.", usage: "/nuke" },
            { name: "/slowmode", desc: "Set the channel rate limit in seconds.", usage: "/slowmode seconds:int" },
            { name: "/lock", desc: "Prevent @everyone from sending messages.", usage: "/lock" },
            { name: "/unlock", desc: "Remove the current channel lock.", usage: "/unlock" },
            { name: "/shadowban", desc: "Silently restrict a member using the mute role without notifying them. Logs internally only.", usage: "/shadowban user:@user reason:text" },
            { name: "/unshadowban", desc: "Lift a shadow ban and restore a member's ability to interact.", usage: "/unshadowban user:@user" },
            { name: "/tactical report", desc: "Generate a full moderation history report for a user — all cases, action counts, and timestamps.", usage: "/tactical report user:@user" },
            { name: "/tactical intercept", desc: "Lock every text channel across the entire server simultaneously.", usage: "/tactical intercept reason:text" },
            { name: "/tactical restore", desc: "Lift a server-wide intercept — unlocks all text channels at once.", usage: "/tactical restore" },
            { name: "/tactical breach", desc: "Kick a user and bulk-delete their recent messages from the current channel.", usage: "/tactical breach user:@user purge_count:int reason:text" }
        ]
    },
    {
        category: "Economy",
        icon: "Coins",
        commands: [
            { name: "/economy balance", desc: "Show wallet, bank, net worth, earned, and spent totals.", usage: "/economy balance user:@user" },
            { name: "/economy daily", desc: "Claim a $500 reward every 24 hours.", usage: "/economy daily" },
            { name: "/economy work", desc: "Work a job for $50-$200 every 30 minutes.", usage: "/economy work" },
            { name: "/economy pay", desc: "Transfer wallet money to another member.", usage: "/economy pay user:@user amount:int" },
            { name: "/economy deposit", desc: "Move wallet funds into the protected bank.", usage: "/economy deposit amount:int" },
            { name: "/economy withdraw", desc: "Move bank funds back into your wallet.", usage: "/economy withdraw amount:int" },
            { name: "/economy beg", desc: "Ask for small spare change.", usage: "/economy beg" },
            { name: "/economy search", desc: "Search random places for hidden cash.", usage: "/economy search" },
            { name: "/economy slots", desc: "Spin the Hyperforge slot machine.", usage: "/economy slots bet:int" },
            { name: "/economy gamble_info", desc: "View slot payout rules and multipliers.", usage: "/economy gamble_info" },
            { name: "/economy shop", desc: "Browse the global auto-rendered shop catalog with profile, cosmetic, boost, utility, community, and limited items.", usage: "/economy shop category:text" },
            { name: "/economy buy", desc: "Buy one or more items from the global shop using wallet funds.", usage: "/economy buy item:text quantity:int" },
            { name: "/economy inventory", desc: "View your purchased shop items in the current server economy.", usage: "/economy inventory user:@user" },
            { name: "/economy profile", desc: "View wallet, bank, net worth, local/global rank, and shop inventory count in one v4.1 profile.", usage: "/economy profile user:@user" },
            { name: "/economy rob", desc: "Attempt to steal from another user's wallet.", usage: "/economy rob user:@user" },
            { name: "/economy crime", desc: "Take a high-risk job with a 60-minute cooldown.", usage: "/economy crime" },
            { name: "/economy fish", desc: "Catch and sell fish with a 5-minute cooldown.", usage: "/economy fish" },
            { name: "/economy hunt", desc: "Hunt valuable animals with a 10-minute cooldown.", usage: "/economy hunt" },
            { name: "/economy leaderboard", desc: "Show the richest users in this server.", usage: "/economy leaderboard" },
            { name: "/economy global_leaderboard", desc: "Show the richest users across all servers.", usage: "/economy global_leaderboard" }
        ]
    },
    {
        category: "Leveling",
        icon: "Chart",
        commands: [
            { name: "/leveling rank", desc: "View level, XP, progress, and rank card preferences.", usage: "/leveling rank user:@user" },
            { name: "/leveling leaderboard", desc: "Show the top activity leaders locally or globally.", usage: "/leveling leaderboard global:true" },
            { name: "/leveling customize", desc: "Set rank card background and colors.", usage: "/leveling customize background:text color:#00E5FF text_color:#FFFFFF" }
        ]
    },
    {
        category: "Utility",
        icon: "Tools",
        commands: [
            { name: "/help", desc: "Show command categories and quick links.", usage: "/help command:text" },
            { name: "/ping", desc: "Measure command response latency.", usage: "/ping" },
            { name: "/botinfo", desc: "Show version, network telemetry, and project links.", usage: "/botinfo" },
            { name: "/stats", desc: "Show live bot reach and uptime stats.", usage: "/stats" },
            { name: "/server", desc: "Show information about the current server.", usage: "/server" },
            { name: "/user", desc: "Show account metadata for a user.", usage: "/user user:@user" },
            { name: "/avatar", desc: "Fetch a user's avatar.", usage: "/avatar user:@user" },
            { name: "/embed", desc: "Send a custom embed.", usage: "/embed title:text description:text color:text" },
            { name: "/timestamp", desc: "Preview Discord timestamp formats.", usage: "/timestamp unix:int" },
            { name: "/math", desc: "Evaluate a math expression.", usage: "/math expression:text" },
            { name: "/qr", desc: "Generate a QR code image.", usage: "/qr data:text" },
            { name: "/crypto", desc: "Show the configured crypto lookup placeholder.", usage: "/crypto symbol:text" },
            { name: "/translate", desc: "Show translation setup status.", usage: "/translate text:text target:text" },
            { name: "/dictionary", desc: "Look up English word definitions.", usage: "/dictionary word:text" },
            { name: "/timer", desc: "Set a DM reminder timer.", usage: "/timer minutes:int" },
            { name: "/worldclock", desc: "Show current times in major cities.", usage: "/worldclock" },
            { name: "/poll", desc: "Create a yes/no reaction poll.", usage: "/poll question:text" },
            { name: "/vote", desc: "Get the Top.gg voting link and reward details.", usage: "/vote" }
        ]
    },
    {
        category: "Fun",
        icon: "Spark",
        commands: [
            { name: "/fun coinflip", desc: "Flip a coin.", usage: "/fun coinflip" },
            { name: "/fun dice", desc: "Roll a die with custom sides.", usage: "/fun dice sides:int" },
            { name: "/fun eightball", desc: "Ask the magic 8-ball a question.", usage: "/fun eightball question:text" },
            { name: "/fun joke", desc: "Get a programming joke.", usage: "/fun joke" },
            { name: "/fun fact", desc: "Get a random fact.", usage: "/fun fact" },
            { name: "/fun cat", desc: "Get a cat image and fact.", usage: "/fun cat" },
            { name: "/fun dog", desc: "Get a dog image.", usage: "/fun dog" },
            { name: "/fun fox", desc: "Get a fox image.", usage: "/fun fox" },
            { name: "/fun panda", desc: "Get a panda image and fact.", usage: "/fun panda" },
            { name: "/fun bird", desc: "Get a bird image and fact.", usage: "/fun bird" },
            { name: "/fun cookie", desc: "Give someone a cookie.", usage: "/fun cookie user:@user" },
            { name: "/fun hug", desc: "Give someone a hug.", usage: "/fun hug user:@user" },
            { name: "/fun pat", desc: "Give someone a pat.", usage: "/fun pat user:@user" },
            { name: "/fun kiss", desc: "Give someone a kiss.", usage: "/fun kiss user:@user" },
            { name: "/fun slap", desc: "Send a playful slap.", usage: "/fun slap user:@user" },
            { name: "/fun meme", desc: "Show a curated meme image.", usage: "/fun meme" },
            { name: "/fun ship", desc: "Rate compatibility between two users.", usage: "/fun ship user1:@user user2:@user" },
            { name: "/fun rate", desc: "Rate anything out of 10.", usage: "/fun rate thing:text" },
            { name: "/fun mock", desc: "Convert text to alternating case.", usage: "/fun mock text:text" },
            { name: "/fun reverse", desc: "Reverse text.", usage: "/fun reverse text:text" },
            { name: "/fun ascii", desc: "Make short spaced-out text.", usage: "/fun ascii text:text" },
            { name: "/fun choose", desc: "Choose from comma-separated options.", usage: "/fun choose options:text" },
            { name: "/fun trivia", desc: "Ask a multiple-choice trivia question.", usage: "/fun trivia" },
            { name: "/fun roast", desc: "Send a light SFW roast.", usage: "/fun roast user:@user" },
            { name: "/fun compliment", desc: "Send a genuine compliment.", usage: "/fun compliment user:@user" }
        ]
    },
    {
        category: "Roles and Config",
        icon: "Config",
        commands: [
            { name: "/add", desc: "Add a role to a member.", usage: "/add user:@user role:@role" },
            { name: "/remove", desc: "Remove a role from a member.", usage: "/remove user:@user role:@role" },
            { name: "/list", desc: "List server roles.", usage: "/list" },
            { name: "/logs", desc: "Configure the moderation log channel.", usage: "/logs channel:#channel" },
            { name: "/welcome", desc: "Configure welcome messages.", usage: "/welcome channel:#channel message:text" },
            { name: "/autorole", desc: "Configure the join auto-role.", usage: "/autorole role:@role" },
            { name: "/muterole", desc: "Set the role applied on /mute and /shadowban.", usage: "/muterole role:@role" },
            { name: "/prefix", desc: "Set the prefix command prefix.", usage: "/prefix prefix:text" },
            { name: "/settings", desc: "Review this server's saved configuration.", usage: "/settings" },
            { name: "/sentinel enable", desc: "Activate Sentinel anti-raid detection for this server.", usage: "/sentinel enable" },
            { name: "/sentinel disable", desc: "Deactivate Sentinel anti-raid detection.", usage: "/sentinel disable" },
            { name: "/sentinel threshold", desc: "Set the join rate that triggers a raid response.", usage: "/sentinel threshold joins:int window:int" },
            { name: "/sentinel status", desc: "Show current Sentinel configuration and state.", usage: "/sentinel status" },
            { name: "/create", desc: "Create a reminder.", usage: "/create duration:text message:text" }
        ]
    }
];
