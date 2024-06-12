use diesel::{AsExpression, FromSqlRow, Identifiable, Insertable, Queryable, QueryId, Selectable, SqlType};
use uuid::Uuid;
use crate::schema::updates;

use std::io::Write;
use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};

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

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = updates)]
#[diesel(primary_key(id))]
pub struct NewUpdates {
    pub owner_id: Uuid,
    pub ownership: Ownership,
    pub update_value: String,
}

#[derive(diesel_derive_enum::DbEnum, Clone, serde::Deserialize,
Debug, Eq, Hash, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::Ownership"]
pub enum Ownership {
    Publisher,
    Subscriber,
}