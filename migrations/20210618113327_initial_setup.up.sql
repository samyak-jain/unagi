-- Add up migration script here

-- Create Library
CREATE TABLE library (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    location VARCHAR NOT NULL
);

-- Create show for library
CREATE TABLE shows (
    id SERIAL PRIMARY KEY,
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
CREATE TABLE episodes (
    id SERIAL PRIMARY KEY,
    show_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    thumbnail VARCHAR,
    file_path VARCHAR NOT NULL,
    locator_id UUID NOT NULL,
    episode_number INTEGER,

    CONSTRAINT fk_show_id FOREIGN KEY (show_id) REFERENCES shows (id)
);
