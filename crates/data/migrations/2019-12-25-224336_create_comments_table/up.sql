CREATE TABLE comments (
    id      SERIAL PRIMARY KEY,
    author  INTEGER NOT NULL REFERENCES views(id),
    post    INTEGER NOT NULL REFERENCES posts(id),
    parent  INTEGER REFERENCES comments(id),
    body    TEXT NOT NULL,
    created TIMESTAMP NOT NULL
)