-- This file should undo anything in `up.sql`
DROP TABLE episodes;

ALTER TABLE shows
DROP COLUMN description,
DROP COLUMN cover_image,
DROP COLUMN banner_image;
