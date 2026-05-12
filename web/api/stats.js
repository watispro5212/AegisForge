const { neon } = require('@neondatabase/serverless');

function toNumber(value) {
    return Number(value || 0);
}

const DEFAULT_BOT_STATS_URL = process.env.BOT_STATS_URL || 'https://aegisforge-bot.fly.dev/api/stats';

async function fetchBotStats() {
    const url = process.env.BOT_STATS_URL || process.env.NEXT_PUBLIC_STATS_API_URL || DEFAULT_BOT_STATS_URL;
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 8000);

    try {
        const response = await fetch(url, {
            headers: { accept: 'application/json' },
            signal: controller.signal
        });

        if (!response.ok) {
            throw new Error(`Bot stats returned ${response.status}`);
        }

        return await response.json();
    } finally {
        clearTimeout(timeout);
    }
}

function normalizeStats(stats, source) {
    const shards = Array.isArray(stats.shards) ? stats.shards : [];
    return {
        server_count: toNumber(stats.server_count),
        user_count: toNumber(stats.user_count),
        uptime_seconds: toNumber(stats.uptime_seconds),
        economy_activity: toNumber(stats.economy_activity),
        xp_processed: toNumber(stats.xp_processed || stats.xp_gain_24h),
        total_commands_executed: toNumber(stats.total_commands_executed),
        total_economy_transactions: toNumber(stats.total_economy_transactions),
        inventory_items: toNumber(stats.inventory_items),
        shards_total: toNumber(stats.shards_total || shards.length),
        shards_online: toNumber(stats.shards_online),
        shards,
        version: stats.version || '4.1.0',
        source
    };
}

module.exports = async function handler(req, res) {
    try {
        const liveStats = await fetchBotStats();
        return res.status(200).json(normalizeStats(liveStats, 'bot-api'));
    } catch (liveError) {
        console.warn('Live bot stats unavailable, falling back to Neon:', liveError.message);
    }

    if (!process.env.DATABASE_URL) {
        return res.status(502).json({ error: 'Live stats unavailable and DATABASE_URL is not configured' });
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

        res.status(200).json(normalizeStats({
            server_count: toNumber(guilds.count),
            user_count: 0,
            uptime_seconds: 0,
            economy_activity: toNumber(wealth.total),
            xp_processed: toNumber(xp.total),
            total_commands_executed: toNumber(commands.total),
            total_economy_transactions: toNumber(transactions.total),
            inventory_items: toNumber(inventory.total),
            shards_total: 0,
            shards_online: 0,
            shards: [],
            version: '4.2.5',
        }, 'vercel-neon'));
    } catch (error) {
        console.error('Database query failed:', error);
        res.status(500).json({ error: 'Failed to fetch stats', details: error.message });
    }
};
