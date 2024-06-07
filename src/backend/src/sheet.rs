use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(PartialEq, Eq, Debug, serde::Deserialize, Clone, Hash)]
pub struct Sheet {
    owner: String,
    name: String,
    private: bool,
    // Privacy of Sheet
    // values: HashMap<u64, String>, // Change this to an Enum of CellFilled, CellEmpty, CellInt, etc.
}

impl Sheet {
    pub fn default() -> Self {
        Sheet {
            owner: "".to_string(),
            name: "".to_string(),
            private: true,
            // values: HashMap::new(),
        }
    }

    pub fn new(owner: String, name: String) -> Self {
        Self {
            owner,
            name,
            private: true,
            // values: HashMap::new(),
        }
    }

    pub fn owner(&self) -> &String {
        &self.owner
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

// CREATE TABLE publishers
// (
// id       INT PRIMARY KEY,
// username VARCHAR(100) NOT NULL,
// password VARCHAR(100) NOT NULL
// );
// CREATE TABLE sheets
// (
// id     INT PRIMARY KEY,
// title   VARCHAR(100) NOT NULL,
// sheet_column INT          NOT NULL,
// sheet_row    INT          NOT NULL,
// sheet_value  VARCHAR(100) NOT NULL
// );
//
// CREATE TABLE sheet_publisher_relations (
// id   INT PRIMARY KEY,
// sheet_id INT,
// publishers_id INT,
// );
// @generated automatically by Diesel CLI.
// diesel::table! {
//     publishers (id) {
//         id -> Integer,
//         username -> VarChar,
//         password -> VarChar,
//     }
// }
// diesel::table! {
//     sheets (id) {
//         id -> Integer,
//         title -> Varchar,
//         sheet_column -> Integer,
//         sheet_row -> Integer,
//         sheet_value -> VarChar,
//     }
// }

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::sheets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Test_Sheet {
    pub(crate) id: Uuid,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::sheets)]
#[diesel(primary_key(id))]
pub struct New_Test_Sheet {
    pub(crate) id: Uuid,
    pub(crate) title: String,
}

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

#[derive(Insertable)]
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

