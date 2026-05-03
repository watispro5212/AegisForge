const { neon } = require('@neondatabase/serverless');

export default async function handler(req, res) {
    // Ensure the database URL is present
    if (!process.env.DATABASE_URL) {
        return res.status(500).json({ error: "Missing DATABASE_URL configuration" });
    }

    try {
        const sql = neon(process.env.DATABASE_URL);
        
        // Fetch Live Stats
        const [warnings] = await sql`SELECT COUNT(*) as count FROM warnings`;
        const [cases] = await sql`SELECT COUNT(*) as count FROM mod_cases`;
        const [reminders] = await sql`SELECT COUNT(*) as count FROM reminders`;
        const [guilds] = await sql`SELECT COUNT(*) as count FROM guild_configs`;

        // Fetch Recent Activity (last 5 warnings)
        const recentActivity = await sql`
            SELECT id, reason, created_at 
            FROM warnings 
            ORDER BY created_at DESC 
            LIMIT 5
        `;

        // Fetch Leaderboard (top 5 users by warnings received)
        const leaderboard = await sql`
            SELECT user_id, COUNT(*) as infraction_count 
            FROM warnings 
            GROUP BY user_id 
            ORDER BY infraction_count DESC 
            LIMIT 5
        `;

        // Send the response
        res.status(200).json({
            stats: {
                warnings: parseInt(warnings.count || 0, 10),
                cases: parseInt(cases.count || 0, 10),
                reminders: parseInt(reminders.count || 0, 10),
                guilds: parseInt(guilds.count || 0, 10)
            },
            recentActivity,
            leaderboard
        });
    } catch (error) {
        console.error("Database query failed:", error);
        res.status(500).json({ error: "Failed to fetch stats from database", details: error.message });
    }
}
