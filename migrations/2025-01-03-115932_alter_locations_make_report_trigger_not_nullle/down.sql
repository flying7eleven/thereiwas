ALTER TABLE locations ALTER COLUMN report_trigger DROP NOT NULL;
ALTER TABLE locations ALTER COLUMN report_trigger SET DEFAULT NULL;
UPDATE locations SET report_trigger = NULL WHERE report_trigger = '?';