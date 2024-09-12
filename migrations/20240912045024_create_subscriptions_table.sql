-- Add migration script here

-- Create Subscriptions Table
CREATE TABLE subscriptions(
    id uuid NOT NULL,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL,
    PRIMARY KEY (id)
);
