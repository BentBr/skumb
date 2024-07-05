CREATE TABLE chat_messages (
    id SERIAL PRIMARY KEY,
    chat_id int NOT NULL,
    creator_id INT NOT NULL,
    uuid UUID NOT NULL UNIQUE,
    text VARCHAR NOT NULL,
    creation_date TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO chat_messages (chat_id, creator_id, uuid, text) VALUES (1, 1, 'b860706b-3739-4f2d-9fe1-aeb2445d5013', 'test message');
INSERT INTO chat_messages (chat_id, creator_id, uuid, text) VALUES (1, 1, 'b860706b-3739-4f2d-9fe1-aeb2445d5014', 'test message 2');

ALTER TABLE chat_messages ADD CONSTRAINT chat_id_fkey FOREIGN KEY (chat_id) REFERENCES chats(id);
ALTER TABLE chat_messages ADD CONSTRAINT creator_id_fkey FOREIGN KEY (creator_id) REFERENCES users(id);
