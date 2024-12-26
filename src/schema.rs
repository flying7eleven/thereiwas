// @generated automatically by Diesel CLI.

diesel::table! {
    locations (id) {
        id -> Int4,
        horizontal_accuracy -> Nullable<Int4>,
        altitude -> Nullable<Int4>,
        latitude -> Float8,
        longitude -> Float8,
        report_trigger -> Nullable<Varchar>,
        measurement_time -> Timestamp,
        vertical_accuracy -> Nullable<Int4>,
        barometric_pressure -> Nullable<Float8>,
        topic -> Varchar,
        created_at -> Timestamp,
    }
}
