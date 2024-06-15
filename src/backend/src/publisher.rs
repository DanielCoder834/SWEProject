use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;
use crate::schema::publishers;


// @author Daniel Kaplan
// Represents the individual elements of the publisher to be able to query from the database
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


// @author Daniel Kaplan
// Represents the individual elements of the publisher to be able to insert into the database
#[derive(Insertable)]
#[diesel(table_name = publishers)]
#[diesel(primary_key(id))]
pub struct NewPublisherCredentials<'a> {
    pub id: &'a Uuid,
    pub username: &'a str,
    pub password: &'a str,
}


