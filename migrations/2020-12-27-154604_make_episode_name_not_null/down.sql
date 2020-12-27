-- This file should undo anything in `up.sql`
ALTER TABLE episodes ALTER COLUMN name DROP NOT NULL;
