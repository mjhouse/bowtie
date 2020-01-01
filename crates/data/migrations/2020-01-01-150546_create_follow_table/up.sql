CREATE TABLE follows (
    id SERIAL PRIMARY KEY,
    subscriber INTEGER NOT NULL REFERENCES views(id),
    publisher  INTEGER NOT NULL REFERENCES views(id),
    unique(subscriber,publisher)
)