-- Add migration script here
-- Wrap the whole migration in a transaction
BEGIN;
UPDATE subscriptions
SET status = 'confirmed'
WHERE status IS NULL;
ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
