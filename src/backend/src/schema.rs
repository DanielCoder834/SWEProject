// @generated automatically by Diesel CLI.

diesel::table! {
    publisher_sheets (sheets_id, publisher_id) {
        sheets_id -> Int4,
        publisher_id -> Int4,
    }
}

diesel::table! {
    publishers (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
    }
}

diesel::table! {
    sheet_elems (id) {
        id -> Int4,
        #[max_length = 100]
        sheet_column_identifier -> Varchar,
        sheet_row -> Int4,
        #[max_length = 100]
        sheet_value -> Varchar,
        sheet_id -> Int4,
    }
}

diesel::table! {
    sheets (id, sheet_elem_id) {
        id -> Int4,
        sheet_elem_id -> Int4,
        #[max_length = 100]
        title -> Varchar,
    }
}

diesel::joinable!(publisher_sheets -> publishers (publisher_id));
diesel::joinable!(publisher_sheets -> sheet_elems (sheets_id));

diesel::allow_tables_to_appear_in_same_query!(
    publisher_sheets,
    publishers,
    sheet_elems,
    sheets,
);
