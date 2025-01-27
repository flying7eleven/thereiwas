CREATE TABLE audit_log
(
    id           SERIAL PRIMARY KEY,
    request_time TIMESTAMP   NOT NULL,
    action       VARCHAR(32) NOT NULL, -- client_auth, user_auth, etc.
    result       VARCHAR(32) NOT NULL, -- successful, failed, etc.
    source       VARCHAR(46) NOT NULL  -- length of INET6_ADDRSTRLEN field
);

DROP TABLE authorization_requests; -- delete the old table