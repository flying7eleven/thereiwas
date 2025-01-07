-- the table which will hold the WiFi networks the smartphone reported
CREATE TABLE wifi_access_points
(
    id    SERIAL PRIMARY KEY,
    bssid VARCHAR(18) NOT NULL, -- 48 octets / 13 hex characters + 5 colons
    ssid  VARCHAR(32) NOT NULL,

    constraint wifi_access_points_unique_key unique (bssid, ssid)
);