CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    sender INTEGER NOT NULL REFERENCES views(id),
    receiver INTEGER NOT NULL REFERENCES views(id),
    body TEXT NOT NULL,
    created TIMESTAMP NOT NULL
)