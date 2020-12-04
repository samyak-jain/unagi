-- Your SQL goes here
CREATE TABLE shows (
    id SERIAL PRIMARY KEY,
    library_id INTEGER,
    title VARCHAR NOT NULL,
    image VARCHAR,
    file_path VARCHAR NOT NULL,

    CONSTRAINT fk_library_id FOREIGN KEY (library_id) REFERENCES library (id)
)
