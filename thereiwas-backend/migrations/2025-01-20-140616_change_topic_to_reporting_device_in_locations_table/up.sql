ALTER TABLE locations ADD reporting_device INT DEFAULT NULL;
UPDATE locations SET reporting_device = (SELECT id FROM client_tokens LIMIT 1) WHERE reporting_device IS NULL; -- bad migration, but okay for now since we're still in development
ALTER TABLE locations ALTER COLUMN reporting_device SET NOT NULL;
ALTER TABLE locations DROP COLUMN topic;
