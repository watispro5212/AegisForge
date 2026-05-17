-- ============================================================
-- AegisForge v4.4 - Rank Card Cosmetic Equipment
-- ============================================================

ALTER TABLE users_leveling
ADD COLUMN IF NOT EXISTS rank_card_badge TEXT NOT NULL DEFAULT 'none',
ADD COLUMN IF NOT EXISTS rank_card_frame TEXT NOT NULL DEFAULT 'none';
