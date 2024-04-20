CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    uuid UUID NOT NULL UNIQUE,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    salutation VARCHAR NOT NULL DEFAULT '',
    first_name VARCHAR NOT NULL DEFAULT '',
    last_name VARCHAR NOT NULL DEFAULT '',
    creation_date TIMESTAMP NOT NULL DEFAULT NOW(),
    modification_date TIMESTAMP,
    deletion_date TIMESTAMP
);

INSERT INTO users (uuid, username, email, password) VALUES ('b860706a-3739-4f2d-9fe1-aeb2445d50d0', 'placeholder', 'placeholder@email.com', 'placeholder password');

ALTER TABLE users ADD CONSTRAINT email_format CHECK (email ~* '^[^@]+@[A-Za-z0-9üäöß-]+(\.[A-Za-z0-9üäöß-]+)*\.[A-Za-z]{2,4}$');