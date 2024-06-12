use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;
use crate::schema::publishers;

// Written by Daniel Kaplan
#[derive(PartialEq, Hash, Eq, Debug, serde::Deserialize,
Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::publishers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Publisher {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

// Written by Daniel Kaplan
impl Publisher {
    pub fn default() -> Self {
        Publisher {
            id: Uuid::new_v4(),
            username: "".to_string(),
            password: "".to_string(),
        }
    }
    pub fn new(username: String, password: String, id: Uuid) -> Self {
        Publisher {
            id,
            username: username.clone(),
            password,
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

}

// Written by Daniel Kaplan
#[derive(Insertable)]
#[diesel(table_name = publishers)]
#[diesel(primary_key(id))]
pub struct NewPublisherCredentials<'a> {
    pub id: &'a Uuid,
    pub username: &'a str,
    pub password: &'a str,
}


