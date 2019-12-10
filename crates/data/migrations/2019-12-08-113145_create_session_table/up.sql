CREATE TABLE session (
    id       SERIAL PRIMARY KEY,
    user_key VARCHAR(128) NOT NULL UNIQUE,
    user_id  INTEGER NOT NULL REFERENCES users(id)
)