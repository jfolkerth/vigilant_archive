CREATE TABLE characters
(
    id              INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name            VARCHAR NOT NULL,
    description     TEXT,
    secrets         TEXT
)