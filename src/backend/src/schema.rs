diesel::table! {
    publishers (id) {
        id -> Int8,
        username -> VarChar,
        password -> VarChar,
    }
}
diesel::table! {
    sheet_elems (id) {
        id -> Int8,
        title -> Varchar,
        sheet_column_identifier -> VarChar,
        sheet_row -> Int8,
        sheet_value -> VarChar,
        sheet_id -> Int8,
    }
}


diesel::table!{
    publisher_sheets(publisher_sheets_id) {
        sheet_elem_id -> Int8,
        publisher_id -> Int8,
        publisher_sheets_id -> Int8
    }
}