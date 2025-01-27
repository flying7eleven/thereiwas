CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    username      VARCHAR(64) NOT NULL,
    password_hash VARCHAR(60) NOT NULL -- a bcrypt hash should be max. 60 characters
);

CREATE TABLE roles
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(32) NOT NULL UNIQUE, -- the name of the role
    description TEXT        NOT NULL         -- a small description what this role is for
);

INSERT INTO roles
VALUES (DEFAULT, 'admin', 'A user with most privileges on this instance'),
       (DEFAULT, 'user', 'A generic user who can only see some of the information');

CREATE TABLE permissions
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(32) NOT NULL UNIQUE, -- the permission in the form of 'category:permission:read'
    description TEXT        NOT NULL         -- a small description what this permission will allow
);

INSERT INTO permissions
VALUES (DEFAULT, 'view:users', 'List all users registered to this instance'),
       (DEFAULT, 'delete:users', 'Delete a user from this instance');

CREATE TABLE users_to_roles
(
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL CONSTRAINT users_to_roles_user_id_fk references users,
    role_id INT NOT NULL CONSTRAINT users_to_roles_role_id_fk references roles
);

CREATE TABLE roles_to_permissions
(
    id            SERIAL PRIMARY KEY,
    role_id       INT NOT NULL CONSTRAINT roles_to_permissions_role_id_fk references roles,
    permission_id INT NOT NULL CONSTRAINT roles_to_permissions_permission_id_fk references permissions
);