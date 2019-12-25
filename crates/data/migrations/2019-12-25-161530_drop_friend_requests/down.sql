CREATE TABLE friend_requests (
    id SERIAL PRIMARY KEY,
    sender INTEGER NOT NULL REFERENCES views(id),
    receiver INTEGER NOT NULL REFERENCES views(id),
    accepted BOOLEAN NOT NULL,
    unique ( sender, receiver )
)