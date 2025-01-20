ALTER TABLE locations ADD topic VARCHAR(200) DEFAULT NULL;
UPDATE locations SET topic = 'unknown' WHERE topic IS NULL; -- bad migration, but okay for now since we're still in development
ALTER TABLE locations ALTER COLUMN topic SET NOT NULL;
ALTER TABLE locations DROP COLUMN reporting_device;