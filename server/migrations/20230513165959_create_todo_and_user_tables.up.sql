-- Add up migration script here

CREATE TABLE todos
(
    id              BIGSERIAL PRIMARY KEY UNIQUE NOT NULL,
    title           TEXT NOT NULL,
    description     TEXT NOT NULL,
    created_at      TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP NOT NULL
);

CREATE TABLE users
(
    id              BIGSERIAL PRIMARY KEY UNIQUE NOT NULL,
    email           TEXT NOT NULL UNIQUE,
    first_name      TEXT NOT NULL,
    created_at      TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP NOT NULL
);

