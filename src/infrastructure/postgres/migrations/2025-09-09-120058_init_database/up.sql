CREATE TABLE users (
    id                   INTEGER PRIMARY KEY,
    citizen_id           VARCHAR(32)  NOT NULL UNIQUE,
    first_name           VARCHAR(100) NOT NULL,
    last_name            VARCHAR(100) NOT NULL,
    phone_number         VARCHAR(32)  NOT NULL,
    password        VARCHAR(255) NOT NULL,

    role                 VARCHAR[] NOT NULL,

    created_at           TIMESTAMP NOT NULL DEFAULT now(),
    updated_at           TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at           TIMESTAMP
);
