const { neon } = require('@neondatabase/serverless');

function toNumber(value) {
    return Number(value || 0);
}

module.exports = async function handler(req, res) {
    if (!process.env.DATABASE_URL) {
        return res.status(500).json({ error: 'Missing DATABASE_URL configuration' });
    }

    try {
        const sql = neon(process.env.DATABASE_URL);

        const [guilds] = await sql`SELECT COUNT(*) AS count FROM guild_configs`;
        const [wealth] = await sql`SELECT COALESCE(SUM(balance + bank), 0) AS total FROM users_economy`;
        const [xp] = await sql`SELECT COALESCE(SUM(xp), 0) AS total FROM users_leveling`;
        const [inventory] = await sql`SELECT COALESCE(SUM(quantity), 0) AS total FROM economy_inventory`;
        const [commands] = await sql`
            SELECT COALESCE((SELECT stat_value FROM global_stats WHERE stat_key = 'total_commands_executed'), 0) AS total
        `;
        const [transactions] = await sql`
            SELECT COALESCE((SELECT stat_value FROM global_stats WHERE stat_key = 'total_economy_transactions'), 0) AS total
        `;

        res.status(200).json({
            server_count: toNumber(guilds.count),
            user_count: 0,
            uptime_seconds: 0,
            economy_activity: toNumber(wealth.total),
            xp_gain_24h: toNumber(xp.total),
            total_commands_executed: toNumber(commands.total),
            total_economy_transactions: toNumber(transactions.total),
            inventory_items: toNumber(inventory.total),
            shards_total: 0,
            shards_online: 0,
            shards: [],
            version: '4.1.0',
            source: 'vercel-neon'
        });
    } catch (error) {
        console.error('Database query failed:', error);
        res.status(500).json({ error: 'Failed to fetch stats from database', details: error.message });
    }
};
