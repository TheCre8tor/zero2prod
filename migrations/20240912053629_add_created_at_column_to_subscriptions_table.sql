-- Add migration script here

-- Add created_at column to subscription table
ALTER TABLE subscriptions
ADD COLUMN created_at timestamptz DEFAULT CURRENT_TIMESTAMP;