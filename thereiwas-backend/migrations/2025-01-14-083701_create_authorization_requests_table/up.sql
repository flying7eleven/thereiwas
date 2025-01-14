CREATE TABLE authorization_requests
(
    id                       SERIAL PRIMARY KEY,
    request_time             TIMESTAMP    NOT NULL,
    auth_type                VARCHAR(16)  NOT NULL,     -- User, ClientToken
    auth_result              VARCHAR(16)  NOT NULL,     -- Successful, Failed
    identification_principle VARCHAR(128) DEFAULT NULL, -- e.g. username, client_id, etc.
    source                   VARCHAR(128) NOT NULL
);