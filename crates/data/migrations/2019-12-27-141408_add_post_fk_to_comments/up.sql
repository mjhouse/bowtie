ALTER TABLE comments
ADD post INTEGER NOT NULL REFERENCES posts(id);