CREATE TABLE client_tokens
(
    id          SERIAL PRIMARY KEY,
    client      VARCHAR(36) NOT NULL, -- GUID which identifies the client
    secret      VARCHAR(10) NOT NULL, -- secret matching to the GUID
    description VARCHAR(128) DEFAULT NULL
);