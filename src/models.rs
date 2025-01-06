use crate::schema::{locations, locations_to_wifi_access_points, wifi_access_points};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable)]
#[diesel(table_name = locations)]
pub struct Location {
    pub id: i32,
    pub horizontal_accuracy: Option<i32>,
    pub altitude: Option<i32>,
    pub latitude: f64,
    pub longitude: f64,
    pub report_trigger: String,
    pub measurement_time: NaiveDateTime,
    pub vertical_accuracy: Option<i32>,
    pub barometric_pressure: Option<f64>,
    pub topic: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = locations)]
pub struct NewLocation {
    pub horizontal_accuracy: Option<i32>,
    pub altitude: Option<i32>,
    pub latitude: f64,
    pub longitude: f64,
    pub report_trigger: String,
    pub measurement_time: NaiveDateTime,
    pub vertical_accuracy: Option<i32>,
    pub barometric_pressure: Option<f64>,
    pub topic: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = wifi_access_points)]
pub struct WifiAccessPoint {
    pub id: i32,
    pub bssid: String,
    pub ssid: String,
}

#[derive(Insertable)]
#[diesel(table_name = wifi_access_points)]
pub struct NewWifiAccessPoint {
    pub bssid: String,
    pub ssid: String,
}

#[derive(Insertable)]
#[diesel(table_name = locations_to_wifi_access_points)]
pub struct NewLocationToWifiAccessPoint {
    pub location_id: i32,
    pub wifi_access_point_id: i32,
}
