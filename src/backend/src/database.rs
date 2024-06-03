// Third Party Libraries
use std::collections::HashMap;
use std::env;
use diesel::dsl::exists;

// use std::error::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::select;
use dotenv::dotenv;

use crate::{results, schema};
// Our Files
use crate::publisher::*;
use crate::publisher::Publisher;
// use crate::schema::sheets;
use crate::schema::{publisher_sheets, sheets};
use crate::schema::sheets::{title};
use crate::sheet::{New_Test_Sheet, NewSheetElem, SheetElem, Test_Sheet};

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

pub fn insert_sheet_elem(sheet_column_identifier: String,
                         sheet_row: i32,
                         sheet_value: String,
                         id: i32,
                         sheet_id: i32,
) -> QueryResult<SheetElem> {
    use crate::schema::sheet_elems;
    let new_sheet_elem = NewSheetElem {
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

pub fn insert_sheet_relation_elem(new_sheet: &New_Test_Sheet,
                                  new_sheet_elemt: &NewSheetElem,
                                  publisher: &Publisher) -> RustResults<(), String> {
    use crate::schema::{publisher_sheets, sheet_elems, sheets};

    // Inserting new sheet
    let insert_sheet_result =
        diesel::insert_into(sheets::table)
            .values(new_sheet)
            .returning(Test_Sheet::as_returning())
            .get_result(&mut establish_connection());

    if insert_sheet_result.is_err() {
        let err_msg = insert_sheet_result.err().unwrap().to_string();
        return Err(format!("Error for inserting sheet: {err_msg}"));
    }

    // Inserting new sheet element
    let insert_sheet_elem_results =
        diesel::insert_into(sheet_elems::table)
            .values(new_sheet_elemt)
            .returning(SheetElem::as_returning())
            .get_result(&mut establish_connection());

    if insert_sheet_elem_results.is_err() {
        let err_msg = insert_sheet_elem_results.err().unwrap().to_string();
        return Err(format!("Error for inserting sheet element: {err_msg}"));
    }

    // Inserting into the junction table
    let new_sheet_publisher = NewPublisherSheet {
        publisher_id: publisher.id,
        sheets_id: insert_sheet_result.unwrap().id,
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

// pub fn delete_sheet()

pub fn get_sheets_by_a_publisher(publisher: &Publisher) -> Vec<Test_Sheet> {
    use crate::schema::{sheets};
    PublisherSheet::belonging_to(publisher)
        .inner_join(sheets::table)
        .select(Test_Sheet::as_select())
        .load(&mut establish_connection())
        .expect("Oops")
}

pub fn delete_sheet_by_sheet_name_and_user(publisher_name: &String, sheet_title: &String) -> RustResults<(usize, usize), Result> {
    use crate::schema::{sheets, publisher_sheets};

    let publisher = get_password_of_username(publisher_name);
    let publisher_no_err = if publisher.is_err() {
        return Err(publisher.err().unwrap());
    } else {
        publisher.unwrap()
    };

    let sheets_to_delete: Vec<Test_Sheet> = PublisherSheet::belonging_to(&publisher_no_err)
        .inner_join(sheets::table)
        .filter(title.eq(sheet_title))
        .select(Test_Sheet::as_select())
        .load(&mut establish_connection())
        .expect("Oops");

    let sheet_ids_to_delete: &Vec<i32> =
        &sheets_to_delete.iter().map(|sheet| sheet.id).collect::<Vec<i32>>();

    let delete_sheet_relation =
        diesel::delete(publisher_sheets::dsl::publisher_sheets.filter(
            publisher_sheets::dsl::sheets_id.eq_any(sheet_ids_to_delete)
        )).execute(&mut establish_connection());
    let delete_sheet_relation_result = if delete_sheet_relation.is_err() {
        let err_msg = delete_sheet_relation.err().unwrap().to_string();
        return Err(Result::error(err_msg, vec![]));
    } else {
        delete_sheet_relation.unwrap()
    };

    let delete_sheet_result =
        diesel::delete(sheets::dsl::sheets.filter(sheets::dsl::id.eq_any(sheet_ids_to_delete))).execute(&mut establish_connection());
    if delete_sheet_result.is_err() {
        let err_msg = delete_sheet_result.err().unwrap().to_string();
        Err(Result::error(err_msg, vec![]))
    } else {
        Ok((delete_sheet_result.unwrap(), delete_sheet_relation_result))
    }
}

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

pub fn password_and_username_in_db(auth_username: &str, auth_password: &str) -> bool {
    use crate::schema::publishers::dsl::{password, publishers, username};
    let exists_credentials = select(exists(publishers
        .filter(username.eq(auth_username))
        .filter(password.eq(auth_password))))
        .get_result(&mut establish_connection());
    return exists_credentials.unwrap();
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Publisher))]
#[diesel(table_name = publisher_sheets)]
#[diesel(primary_key(sheets_id, publisher_id))]
struct PublisherSheet {
    pub publisher_id: i32,
    pub sheets_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = publisher_sheets)]
struct NewPublisherSheet {
    pub publisher_id: i32,
    pub sheets_id: i32,
}