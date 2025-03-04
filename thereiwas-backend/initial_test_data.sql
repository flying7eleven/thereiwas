INSERT INTO users(username, password_hash)
VALUES ('demo', '$2b$12$M/ELjth7dTOG9zB/mfYPKOUl0LD4YzLqp2ugCKPaz.9sz5OKXyKHa'); -- password demo
INSERT INTO client_tokens(client, secret)
VALUES ('0d2e3a43-3951-4a9f-9e3b-7b7e5a2760cd', 'somesecret');