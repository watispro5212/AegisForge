use std::collections::VecDeque;
use std::time::Instant;

/// Per-guild sentinel anti-raid configuration (held in memory, not persisted).
#[derive(Debug, Clone)]
pub struct SentinelConfig {
    /// Whether Sentinel is active for this guild.
    pub enabled: bool,
    /// Number of joins within `window_secs` that triggers a raid response.
    pub threshold: usize,
    /// Rolling detection window in seconds.
    pub window_secs: u64,
}

impl Default for SentinelConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            threshold: 5,
            window_secs: 10,
        }
    }
}

/// Tracks recent join timestamps and user IDs per guild.
/// Each entry is (join_time, user_id).
pub type RaidTracker = dashmap::DashMap<u64, VecDeque<(Instant, u64)>>;
