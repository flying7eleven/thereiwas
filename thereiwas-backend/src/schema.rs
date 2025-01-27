// @generated automatically by Diesel CLI.

diesel::table! {
    audit_log (id) {
        id -> Int4,
        request_time -> Timestamp,
        #[max_length = 32]
        action -> Varchar,
        #[max_length = 32]
        result -> Varchar,
        #[max_length = 46]
        source -> Varchar,
    }
}

diesel::table! {
    client_tokens (id) {
        id -> Int4,
        #[max_length = 36]
        client -> Varchar,
        #[max_length = 10]
        secret -> Varchar,
        #[max_length = 128]
        description -> Nullable<Varchar>,
    }
}

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
        created_at -> Nullable<Timestamp>,
        reporting_device -> Int4,
    }
}

diesel::table! {
    locations_to_wifi_access_points (id) {
        id -> Int4,
        location_id -> Int4,
        wifi_access_point_id -> Int4,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    roles_to_permissions (id) {
        id -> Int4,
        role_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 60]
        password_hash -> Varchar,
    }
}

diesel::table! {
    users_to_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::table! {
    wifi_access_points (id) {
        id -> Int4,
        #[max_length = 18]
        bssid -> Varchar,
        #[max_length = 32]
        ssid -> Varchar,
        last_seen -> Nullable<Timestamp>,
    }
}

diesel::joinable!(locations_to_wifi_access_points -> locations (location_id));
diesel::joinable!(locations_to_wifi_access_points -> wifi_access_points (wifi_access_point_id));
diesel::joinable!(roles_to_permissions -> permissions (permission_id));
diesel::joinable!(roles_to_permissions -> roles (role_id));
diesel::joinable!(users_to_roles -> roles (role_id));
diesel::joinable!(users_to_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_log,
    client_tokens,
    locations,
    locations_to_wifi_access_points,
    permissions,
    roles,
    roles_to_permissions,
    users,
    users_to_roles,
    wifi_access_points,
);
