use diesel::{Identifiable, Insertable, Queryable, Selectable, SqlType};
use uuid::Uuid;
use crate::schema::updates;

// use diesel::sql_types::{Integer};
// use diesel::not_none;

// Written by Daniel Kaplan
#[derive(SqlType, PartialEq, Hash, Eq, Debug, serde::Deserialize,
Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::updates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Updates {
    id: i32,
    owner_id: Uuid,
    ownership: Ownership,
    pub update_value: String,
}

// Written by Daniel Kaplan
#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = updates)]
#[diesel(primary_key(id))]
pub struct NewUpdates {
    pub owner_id: Uuid,
    pub ownership: Ownership,
    pub update_value: String,
}

// Written by Daniel Kaplan
#[derive(diesel_derive_enum::DbEnum, Clone, serde::Deserialize,
Debug, Eq, Hash, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::Ownership"]
pub enum Ownership {
    Publisher,
    Subscriber,
}
