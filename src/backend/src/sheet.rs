use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;


// @author Daniel Kaplan
#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::sheets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
#[allow(non_camel_case_types)]
pub struct Test_Sheet {
    pub(crate) id: Uuid,
    pub title: String,
}

// @author Daniel Kaplan
#[derive(Insertable)]
#[diesel(table_name = crate::schema::sheets)]
#[diesel(primary_key(id))]
#[allow(non_camel_case_types)]
pub struct New_Test_Sheet {
    pub(crate) id: Uuid,
    pub(crate) title: String,
}

// @author Daniel Kaplan
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

impl SheetElem {
    // @author Daniel Kaplan
    pub fn new(
        id: Uuid,
        sheet_column_identifier: String,
        sheet_row: i32,
        sheet_value: String,
        sheet_id: Uuid) -> Self {
        Self {
            id,
            sheet_column_identifier,
            sheet_row,
            sheet_value,
            sheet_id
        }
    }

    // @author Daniel Kaplan
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            sheet_column_identifier: "A".to_string(),
            sheet_row: 0,
            sheet_value: "".to_string(),
            sheet_id: Uuid::new_v4(),
        }
    }
}

// @author Daniel Kaplan
#[derive(Insertable, Debug)]
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

