const commandsData = [
    {
        category: "Moderation",
        icon: "🛡️",
        commands: [
            { name: "/ban", desc: "Ban a user from the server with an optional message purge window (0–7 days).", usage: "/ban user:@user reason:text days:int" },
            { name: "/softban", desc: "Ban and immediately unban a user to clear their messages without a permanent ban.", usage: "/softban user:@user days:int reason:text" },
            { name: "/unban", desc: "Unban a user by their Discord ID.", usage: "/unban id:string" },
            { name: "/kick", desc: "Kick a member from the server with a reason.", usage: "/kick user:@user reason:text" },
            { name: "/timeout", desc: "Temporarily timeout a member for 1–40,320 minutes.", usage: "/timeout user:@user duration:int reason:text" },
            { name: "/mute", desc: "Alias for timeout — mutes a member for 60 minutes by default.", usage: "/mute user:@user reason:text" },
            { name: "/unmute", desc: "Remove an active timeout from a member.", usage: "/unmute user:@user" },
            { name: "/warn", desc: "Issue a formal warning that is logged to the database with a case number.", usage: "/warn user:@user reason:text" },
            { name: "/purge", desc: "Bulk delete up to 100 messages from the current channel.", usage: "/purge amount:int" },
            { name: "/nuke", desc: "Vaporize the current channel and re-create it to clear all history.", usage: "/nuke" },
            { name: "/slowmode", desc: "Set the channel's slowmode rate limit in seconds.", usage: "/slowmode seconds:int" },
            { name: "/lock", desc: "Lock the current channel by denying @everyone the ability to send messages.", usage: "/lock" },
            { name: "/unlock", desc: "Remove the lock on the current channel.", usage: "/unlock" }
        ]
    },
    {
        category: "Economy",
        icon: "💰",
        commands: [
            { name: "/economy balance", desc: "Check your wallet, bank, and total net worth with advanced v4 tracking.", usage: "/economy balance user:@user" },
            { name: "/economy daily", desc: "Claim your daily reward of $500. Resets every 24 hours.", usage: "/economy daily" },
            { name: "/economy work", desc: "Work a job to earn $50–$200. Has a 30-minute cooldown.", usage: "/economy work" },
            { name: "/economy crime", desc: "Commit a high-risk crime for big payouts. Watch out for fines!", usage: "/economy crime" },
            { name: "/economy fish", desc: "Relax by the forge lake and catch some fish to sell.", usage: "/economy fish" },
            { name: "/economy hunt", desc: "Go hunting for rare animals to sell at the market.", usage: "/economy hunt" },
            { name: "/economy slots", desc: "Bet your money on the high-stakes forge slots.", usage: "/economy slots bet:int" },
            { name: "/economy shop", desc: "Browse the server marketplace for items and roles.", usage: "/economy shop" }
        ]
    },
    {
        category: "Leveling",
        icon: "📈",
        commands: [
            { name: "/leveling rank", desc: "View your current level, XP, and global rank on a beautiful v4 card.", usage: "/leveling rank user:@user" },
            { name: "/leveling leaderboard", desc: "See the top 10 most active members in the server.", usage: "/leveling leaderboard" },
            { name: "/leveling customize", desc: "Change your rank card background, text color, and progress bar color.", usage: "/leveling customize" }
        ]
    },
    {
        category: "Utility",
        icon: "⚙️",
        commands: [
            { name: "/botinfo", desc: "Detailed telemetry and global network statistics for AegisForge v4.", usage: "/botinfo" },
            { name: "/serverinfo", desc: "Get detailed information about the current server.", usage: "/serverinfo" },
            { name: "/userinfo", desc: "Lookup information about a specific user.", usage: "/userinfo user:@user" },
            { name: "/avatar", desc: "Fetch a high-resolution version of a user's avatar.", usage: "/avatar user:@user" },
            { name: "/ping", desc: "Check the bot's heartbeat and database latency.", usage: "/ping" },
            { name: "/qrcode", desc: "Generate a QR code from any text or URL instantly.", usage: "/qrcode text:string" },
            { name: "/poll", desc: "Create an interactive reaction-based poll for your community.", usage: "/poll question:string" }
        ]
    },
    {
        category: "Fun",
        icon: "🎉",
        commands: [
            { name: "/meme", desc: "Fetch a random high-quality meme from curated subreddits.", usage: "/meme" },
            { name: "/joke", desc: "Get a random joke to lighten the mood.", usage: "/joke" },
            { name: "/trivia", desc: "Test your knowledge with multiple-choice trivia questions.", usage: "/trivia category:text" },
            { name: "/coinflip", desc: "Flip a coin to settle a debate.", usage: "/coinflip" },
            { name: "/8ball", desc: "Ask the magic 8-ball a question about the future.", usage: "/8ball question:text" },
            { name: "/ship", desc: "See the compatibility between two users.", usage: "/ship user1:@user user2:@user" }
        ]
    },
    {
        category: "Music",
        icon: "🎵",
        commands: [
            { name: "/play", desc: "Stream high-quality audio from various platforms into your voice channel.", usage: "/play query:text" },
            { name: "/skip", desc: "Vote to skip the currently playing track.", usage: "/skip" },
            { name: "/queue", desc: "View the upcoming tracks in the music forge.", usage: "/queue" },
            { name: "/lyrics", desc: "Fetch lyrics for the current song or a specific query.", usage: "/lyrics query:text" },
            { name: "/volume", desc: "Adjust the volume of the music forge (0-100).", usage: "/volume level:int" }
        ]
    }
];
