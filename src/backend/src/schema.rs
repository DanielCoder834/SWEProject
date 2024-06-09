// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ownership"))]
    pub struct Ownership;
}

diesel::table! {
    publisher_sheets (sheets_id, publisher_id) {
        sheets_id -> Uuid,
        publisher_id -> Uuid,
    }
}

diesel::table! {
    publishers (id) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
    }
}

diesel::table! {
    sheet_elems (id) {
        id -> Uuid,
        #[max_length = 100]
        sheet_column_identifier -> Varchar,
        sheet_row -> Int4,
        #[max_length = 100]
        sheet_value -> Varchar,
        sheet_id -> Uuid,
    }
}

diesel::table! {
    sheets (id) {
        id -> Uuid,
        #[max_length = 100]
        title -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::{Int4, Uuid, Varchar};
    use super::sql_types::Ownership;

    updates (id) {
        id -> Int4,
        owner_id -> Uuid,
        #[max_length = 1000]
        update_value -> Varchar,
        ownership -> Ownership,
    }
}

diesel::joinable!(publisher_sheets -> publishers (publisher_id));
diesel::joinable!(publisher_sheets -> sheets (sheets_id));
diesel::joinable!(sheet_elems -> sheets (sheet_id));

diesel::allow_tables_to_appear_in_same_query!(
    publisher_sheets,
    publishers,
    sheet_elems,
    sheets,
    updates,
);
