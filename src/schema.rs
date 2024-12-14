// @generated automatically by Diesel CLI.

diesel::table! {
    endpoints (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        url -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    hits (id) {
        id -> Int4,
        endpoint_id -> Int4,
        hit_time -> Nullable<Timestamptz>,
        ip -> Inet,
        #[max_length = 100]
        user_agent -> Nullable<Varchar>,
    }
}

diesel::joinable!(hits -> endpoints (endpoint_id));

diesel::allow_tables_to_appear_in_same_query!(
    endpoints,
    hits,
);
