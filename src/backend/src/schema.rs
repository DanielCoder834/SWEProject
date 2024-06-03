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
    sheets (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
    }
}

// diesel::table! {
//     updates (id) {
//         id -> Int4,
//         owner_id -> Int4,
//         #[max_length = 1000]
//         update_value -> Varchar,
//         ownership -> crate::updates::Ownership,
//     }
// }

// One to Many Relation from user to updates

diesel::joinable!(publisher_sheets -> publishers (publisher_id));
diesel::joinable!(publisher_sheets -> sheets (sheets_id));

diesel::allow_tables_to_appear_in_same_query!(
    publisher_sheets,
    publishers,
    sheet_elems,
    sheets,
);
