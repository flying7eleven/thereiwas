UPDATE locations SET report_trigger = '?' WHERE report_trigger IS NULL;
ALTER TABLE locations ALTER COLUMN report_trigger SET DEFAULT '?';
ALTER TABLE locations ALTER COLUMN report_trigger SET NOT NULL;