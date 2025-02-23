CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(30) NOT NULL,
    hashed_password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE (email)
);
