-- the table which will hold all collected locations
CREATE TABLE locations
(
    id                  SERIAL PRIMARY KEY,
    horizontal_accuracy INT        DEFAULT NULL,
    altitude            INT        DEFAULT NULL,
    latitude            FLOAT        NOT NULL,
    longitude           FLOAT        NOT NULL,
    report_trigger      VARCHAR(1) DEFAULT NULL,
    measurement_time    TIMESTAMP    NOT NULL,
    vertical_accuracy   INT        DEFAULT NULL,
    barometric_pressure FLOAT      DEFAULT NULL,
    topic               VARCHAR(200) NOT NULL,
    created_at          TIMESTAMP  DEFAULT NULL
);