-- Your SQL goes here
CREATE TABLE IF NOT EXISTS images(
    id SERIAL PRIMARY KEY,
    image bytea
);
CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    images INTEGER NOT NULL,
    FOREIGN KEY (images) REFERENCES images(id)
);
CREATE TABLE IF NOT EXISTS scores(
     id SERIAL PRIMARY KEY,
     users INTEGER NOT NULL,
     epoch VARCHAR NOT NULL,
     FOREIGN KEY (users) REFERENCES users(id)
);