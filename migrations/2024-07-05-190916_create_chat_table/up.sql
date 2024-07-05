CREATE TABLE chats (
    id SERIAL PRIMARY KEY,
    creator_id INT NOT NULL,
    uuid UUID NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    share_uri VARCHAR UNIQUE,
    creation_date TIMESTAMP NOT NULL DEFAULT NOW(),
    modification_date TIMESTAMP,
    deletion_date TIMESTAMP
);

INSERT INTO chats (creator_id, uuid, name) VALUES (1, 'b860706a-3739-4f2d-9fe1-aeb2445d50d1', 'test chat');

ALTER TABLE chats ADD CONSTRAINT creator_id_fkey FOREIGN KEY (creator_id) REFERENCES users(id);