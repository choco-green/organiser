// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 15]
        mobile -> Varchar,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 32]
        password_hash -> Varchar,
        created_at -> Timestamp,
        login_at -> Timestamp,
    }
}
