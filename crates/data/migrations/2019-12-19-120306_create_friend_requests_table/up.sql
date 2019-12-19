CREATE TABLE friend_requests (
    id SERIAL PRIMARY KEY,
    view1 INTEGER NOT NULL REFERENCES views(id),
    view2 INTEGER REFERENCES views(id)
)