use diesel::{AsExpression, FromSqlRow, Identifiable, Insertable, Queryable, QueryId, Selectable, SqlType};
use uuid::Uuid;
use crate::schema::updates;

use std::io::Write;
use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
// use diesel::sql_types::{Integer};
// use diesel::not_none;

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

#[derive(Insertable)]
#[diesel(table_name = updates)]
#[diesel(primary_key(id))]
pub struct NewUpdates {
    pub owner_id: Uuid,
    pub ownership: Ownership,
    pub update_value: String,
}

//, schema = "Updates"
#[derive(SqlType, QueryId)]
#[diesel(postgres_type(name = "ownership"))]
pub struct OwnershipType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Clone, Hash, serde::Deserialize)]
#[diesel(sql_type = OwnershipType)]
pub enum Ownership {
    Publisher,
    Subscriber,
}

impl ToSql<OwnershipType, Pg> for Ownership {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Ownership::Publisher => out.write_all(b"publisher")?,
            Ownership::Subscriber => out.write_all(b"subscriber")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<OwnershipType, Pg> for Ownership {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"publisher" => Ok(Ownership::Publisher),
            b"subscriber" => Ok(Ownership::Subscriber),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}