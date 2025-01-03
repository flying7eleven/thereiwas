use crate::schema::locations;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

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
