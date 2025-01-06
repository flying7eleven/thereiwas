-- the table which will hold the location associations to the access points
CREATE TABLE locations_to_wifi_access_points
(
    id                   SERIAL PRIMARY KEY,
    location_id          INT NOT NULL
        constraint locations_to_wifi_access_points_locations_id_fk references locations,
    wifi_access_point_id INT NOT NULL
        constraint locations_to_wifi_access_points_wifi_access_points_id_fk references wifi_access_points
);
