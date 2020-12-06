-- Your SQL goes here
CREATE TABLE episodes (
    id SERIAL PRIMARY KEY,
    show_id INTEGER NOT NULL,
    name VARCHAR,
    thumbnail VARCHAR,
    file_path VARCHAR NOT NULL,
    locator_id UUID NOT NULL,

    CONSTRAINT fk_show_id FOREIGN KEY (show_id) REFERENCES shows (id)
);

ALTER TABLE shows 
ADD COLUMN description VARCHAR,
ADD COLUMN cover_image VARCHAR,
ADD COLUMN banner_image VARCHAR;
