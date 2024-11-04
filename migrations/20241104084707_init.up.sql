CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS post (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        title VARCHAR(255) NOT NULL UNIQUE,
        content TEXT NOT NULL
    );
