CREATE TABLE IF NOT EXISTS cryptocurrency (
    id BLOB PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    price REAL NOT NULL,
    spent REAL NOT NULL
);
