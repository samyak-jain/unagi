-- Add up migration script here

-- Create Library
CREATE TABLE IF NOT EXISTS library (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY ,
    name VARCHAR NOT NULL,
    location VARCHAR NOT NULL
);

-- Create show for library
CREATE TABLE IF NOT EXISTS shows (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    library_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    file_path VARCHAR NOT NULL,
    description VARCHAR,
    cover_image VARCHAR,
    banner_image VARCHAR,
    season BIGINT NOT NULL,
    parent_season BIGINT NOT NULL,

    CONSTRAINT fk_library_id FOREIGN KEY (library_id) REFERENCES library (id)
);

-- Create episode for show
CREATE TABLE IF NOT EXISTS episodes (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    show_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    thumbnail VARCHAR,
    file_path VARCHAR NOT NULL,
    locator_id UUID NOT NULL,
    episode_number INTEGER,

    CONSTRAINT fk_show_id FOREIGN KEY (show_id) REFERENCES shows (id)
);

-- Create Anime List for caching the list
CREATE TABLE IF NOT EXISTS anime (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    anidb INTEGER NOT NULL,
    tvdb INTEGER,
    season INTEGER,
    episode_offset INTEGER
);

