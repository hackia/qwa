CREATE TABLE breathes
(
    id          INTEGER PRIMARY KEY,
    title       TEXT UNIQUE NOT NULL,
    description TEXT        NOT NULL,
    breath      TEXT        NOT NULL,
    author      TEXT        NOT NULL,
    url         TEXT UNIQUE NOT NULL,
    categories  TEXT[]      NOT NULL,
    tags        TEXT[]      NOT NULL,
    history     INTEGER[]      NOT NULL,
    properties  TEXT[]      DEFAULT ARRAY[]::TEXT[],
    stars       INTEGER              DEFAULT 0,
    depth       INTEGER              DEFAULT 0,
    status      TEXT        NOT NULL DEFAULT 'normal',
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);
