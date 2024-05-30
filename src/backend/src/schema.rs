// @generated automatically by Diesel CLI.

diesel::table! {
    credentials (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
    }
}
