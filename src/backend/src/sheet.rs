use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;


// @author Daniel Kaplan
// Represents the type for the individual sheet to be able to query from the database
#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::sheets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
#[allow(non_camel_case_types)]
pub struct Test_Sheet {
    pub(crate) id: Uuid, // Identifier
    pub title: String, // Name of the sheet
}

// @author Daniel Kaplan
// Represents the type for the individual sheet to be able to insert into the database
#[derive(Insertable)]
#[diesel(table_name = crate::schema::sheets)]
#[diesel(primary_key(id))]
#[allow(non_camel_case_types)]
pub struct New_Test_Sheet {
    pub(crate) id: Uuid, // Identifier
    pub(crate) title: String, // Name of the sheet
}

// @author Daniel Kaplan
// Represents the individual elements of the sheet to be able to query from the database
#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::sheet_elems)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct SheetElem {
    pub id: Uuid,
    sheet_value: String,
    sheet_column_identifier: String,
    sheet_row: i32,
    sheet_id: Uuid,
}

// @author Daniel Kaplan
// Represents the individual elements of the sheet to be able to insert into the database
#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::sheet_elems)]
#[diesel(primary_key(id))]
pub struct NewSheetElem {
    pub id: Uuid,
    pub sheet_column_identifier: String,
    pub sheet_row: i32,
    pub sheet_value: String,
    pub sheet_id: Uuid,
}

impl NewSheetElem {
    // @author Daniel Kaplan
    // Creates a standard element for the database
    // id is the identifier so it is the only element passed in
    pub fn default(sheet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            sheet_column_identifier: "A".to_string(),
            sheet_row: 0,
            sheet_value: "".to_string(),
            sheet_id,
        }
    }
}

