CREATE TABLE comments_meta (
    id      SERIAL PRIMARY KEY,
    parent  INTEGER NOT NULL REFERENCES comments(id),
    child   INTEGER NOT NULL REFERENCES comments(id)
)