// @generated automatically by Diesel CLI.

diesel::table! {
    locations (id) {
        id -> Int4,
        horizontal_accuracy -> Nullable<Int4>,
        altitude -> Nullable<Int4>,
        latitude -> Float8,
        longitude -> Float8,
        #[max_length = 1]
        report_trigger -> Varchar,
        measurement_time -> Timestamp,
        vertical_accuracy -> Nullable<Int4>,
        barometric_pressure -> Nullable<Float8>,
        #[max_length = 200]
        topic -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
