use diesel::{Identifiable, Insertable, Queryable, Selectable};
use crate::schema::publishers;

#[derive(PartialEq, Hash, Eq, Debug, serde::Deserialize,
Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::publishers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Publisher {
    pub id: i32,
    pub username: String,
    pub password: String,
    // sheet_list: Vec<Sheet>,
}

impl Publisher {
    pub fn default() -> Self {
        Publisher {
            id: 0,
            username: "".to_string(),
            password: "".to_string(),
            // sheet_list: vec![],
        }
    }
    pub fn new(username: String, password: String, id: i32) -> Self {
        Publisher {
            id,
            username: username.clone(),
            password,
            // sheet_list: vec![],
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    // pub fn get_sheet_list(&self) -> &Vec<Sheet> {
    //     // &self.sheet_list
    // }
}

#[derive(Insertable)]
#[diesel(table_name = publishers)]
#[diesel(primary_key(id))]
pub struct NewPublisherCredentials<'a> {
    pub id: &'a i32,
    pub username: &'a str,
    pub password: &'a str,
}


