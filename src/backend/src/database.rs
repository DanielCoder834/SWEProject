// Third Party Libraries
use std::collections::HashMap;
// use std::error::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use dotenv::dotenv;

// Our Files
use crate::publisher::*;
use crate::{results, schema};
// use crate::schema::sheets;
use crate::schema::publisher_sheets;
use crate::sheet::{NewSheetElem, SheetElem};
use crate::publisher::Publisher;
use crate::schema::sheet_elems::dsl::sheet_elems;

// Type Aliasing
type Result = results::Result;
type RustResults<T, E> = std::result::Result<T, E>;

#[derive(serde::Deserialize)]
pub struct DataStructure {
    pub storage: HashMap<Publisher, Result>,
}

impl DataStructure {
    pub fn default() -> Self {
        DataStructure {
            storage: HashMap::new(),
            // credentialStorage: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: Publisher, value: &Result) -> Option<Result> {
        self.storage.insert(key, value.clone())
    }
    pub fn delete(&mut self, key: Publisher) -> Option<Result> {
        self.storage.remove(&key)
    }
    pub fn get(&mut self, key: Publisher) -> Option<&Result> {
        self.storage.get(&key)
    }
    pub fn update(&mut self, key: Publisher, new_result: Result) {
        if let Some(result) = self.storage.get_mut(&key) {
            *result = new_result;
        }
    }
}


fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_new_credentials(username: &str, password: &str) -> QueryResult<Publisher> {
    let new_credentials = NewPublisherCredentials {
        id: &0,
        username,
        password,
    };
    // self.incr_id_publisher += 1;
    diesel::insert_into(schema::publishers::table)
        .values(&new_credentials)
        .returning(Publisher::as_returning())
        .get_result(&mut establish_connection())
}

pub fn insert_sheet_elem(title: String,
                         sheet_column_identifier: String,
                         sheet_row: i64,
                         sheet_value: String,
                         id: i64,
                         sheet_id: i64,
) -> QueryResult<SheetElem> {
    use crate::schema::sheet_elems;
    let new_sheet_elem = NewSheetElem {
        title,
        sheet_column_identifier,
        sheet_row,
        sheet_value,
        id,
        sheet_id,
    };
    let conn = &mut establish_connection();
    diesel::insert_into(sheet_elems::table)
        .values(&new_sheet_elem)
        .returning(SheetElem::as_returning())
        .get_result(conn)
}

pub fn insert_publisher_sheet_elem(new_sheet: &NewSheetElem, publisher: &Publisher) -> RustResults<(), String> {
    use crate::schema::{sheet_elems, publisher_sheets};
    // let db_element_publisher = NewPublisherCredentials {
    //     id: &publisher.id,
    //     username: &*publisher.username,
    //     password: &*publisher.password,
    // };
    // // let insert_publisher_results =
    //     diesel::insert_into(publishers::table)
    //     .values(&db_element_publisher)
    //     .returning(Publisher::as_returning())
    //     .get_result(&mut establish_connection());
    //
    // if insert_publisher_results.is_err() {
    //     let err_msg = insert_publisher_results.err().unwrap().to_string();
    //     return Err(format!("Error for inserting publishers: {err_msg}"));
    // }

    let insert_sheet_results =
    diesel::insert_into(sheet_elems::table)
        .values(new_sheet)
        .returning(SheetElem::as_returning())
        .get_result(&mut establish_connection());

    if insert_sheet_results.is_err() {
        let err_msg = insert_sheet_results.err().unwrap().to_string();
        return Err(format!("Error for inserting sheet element: {err_msg}"));
    }

    // Need to make an insertable table for relations

    let new_sheet_publisher = NewPublisherSheet {
        publisher_id: publisher.id,
        sheet_elem_id: insert_sheet_results.unwrap().id,
    };
    let relationship_table_insert_result =
    diesel::insert_into(publisher_sheets::table)
        .values(&new_sheet_publisher)
        .returning(PublisherSheet::as_returning())
        .get_result(&mut establish_connection());

    if relationship_table_insert_result.is_err() {
        let err_msg = relationship_table_insert_result.err().unwrap().to_string();
        return Err(format!("Error for inserting sheet_publisher_relation: {err_msg}"));
    }

    Ok(())
}

// pub fn get_sheet_elem(&mut self, publisher: &str, sheet_title: &str) -> QueryResult<Vec<SheetElem>> {
//     diesel
// }

pub fn get_password_of_username(passed_username: &String) -> RustResults<Publisher, Result> {
    use crate::schema::publishers::dsl::{publishers, username};
    let res = publishers
        .filter(username.eq(passed_username))
        .limit(1)
        .select(Publisher::as_select())
        .get_result(&mut establish_connection());
    if res.is_err() {
        let err_msg = res.err().unwrap().to_string();
        return Err(Result::error(err_msg, vec![]));
    }
    Ok(res.unwrap())
}

pub fn password_and_username_in_db(auth_password: String, auth_username: String) -> bool {
    use crate::schema::publishers::dsl::{publishers, username, password};
    // If credentials_count == 1, there is a user, and if credentials_count == 0, there is none
    // Only can be 0 or 1 because of limit(1)
    let credentials_count: i64 = publishers
        .filter(username.eq(auth_username))
        .filter(password.eq(auth_password))
        .limit(1)
        .count()
        .get_result(&mut establish_connection())
        .expect("Error counting credentials filtered by username and password");
    return credentials_count == 1;
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(SheetElem))]
#[diesel(belongs_to(Publisher))]
#[diesel(table_name = publisher_sheets)]
#[diesel(primary_key(sheet_elem_id, publisher_id))]
struct PublisherSheet {
    pub publisher_id: i64,
    pub sheet_elem_id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = publisher_sheets)]
struct NewPublisherSheet {
    pub publisher_id: i64,
    pub sheet_elem_id: i64,
}