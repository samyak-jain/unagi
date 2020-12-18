-- This file should undo anything in `up.sql`
ALTER TABLE shows DROP COLUMN season;
ALTER TABLE shows DROP COLUMN parent_season;
ALTER TABLE episodes DROP COLUMN episode_number;
