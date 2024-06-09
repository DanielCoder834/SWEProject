// Third Party Libraries
use std::collections::HashMap;
use std::env;
use diesel::dsl::exists;

// use std::error::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::select;
use dotenv::dotenv;
use uuid::Uuid;

use crate::{results, schema};
// Our Files
use crate::publisher::*;
use crate::publisher::Publisher;
// use crate::schema::sheets;
use crate::schema::{publisher_sheets, sheets, updates};
use crate::schema::publishers::dsl::publishers;
// use crate::schema::sheet_elems::dsl::sheet_elems;
use crate::schema::sheet_elems::{sheet_column_identifier, sheet_row, sheet_value};
use crate::schema::sheets::{title};
use crate::sheet::{New_Test_Sheet, NewSheetElem, SheetElem, Test_Sheet};
use crate::updates::{NewUpdates, Ownership, Updates};

// Type Aliasing
type Result = results::Result;
type RustResults<T, E> = std::result::Result<T, E>;


fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_new_credentials(username: &str, password: &str) -> QueryResult<Publisher> {
    let new_credentials = NewPublisherCredentials {
        id: &Uuid::new_v4(),
        username,
        password,
    };
    // self.incr_id_publisher += 1;
    diesel::insert_into(schema::publishers::table)
        .values(&new_credentials)
        .returning(Publisher::as_returning())
        .get_result(&mut establish_connection())
}

pub fn get_all_publishers() -> QueryResult<Vec<Publisher>> {
    use crate::schema::publishers::dsl::{publishers};
    publishers
        .select(Publisher::as_select())
        .get_results(&mut establish_connection())
}


///
///
/// # Arguments
///
/// * `new_sheet_elem`: The element to identify which sheet elements to update along with the value to update them with
/// * `publisher_name`: The value to identify the owner of the sheet
/// * `sheet_name`: The value to identify which sheet to do the updates on
/// * `payload`: The update value stored in the database
/// * `ownership`: An enum representing either a subscriber or a publisher
//
/// returns: Result<usize, Result>
/// The usize is the number of values updated, and the Result is for when there is an issue with the function
///
/// # Examples
///
/// ```
// let new_sheet_elem: NewSheetElem = decoded_sheet(&payload);
// let num_of_rows_updated = update_sheet_elem(
// &new_sheet_elem, &arguement.publisher_name, &arguement.sheet_name, arguement.payload,
// Ownership::publisher);
// println!(format!("{num_of_rows_updated.unwrap()} were affect"));
/// ```
pub fn update_sheet_elem(new_sheet_elem: &Vec<NewSheetElem>,
                         publisher_name: &String,
                         sheet_name: &String,
                         payload: String,
                         ownership: Ownership)
                         -> RustResults<usize, Result> {
    // use crate::schema::sheet_elems::dsl::{sheet_column_identifier, sheet_row, sheet_id};
    use crate::schema::{sheet_elems, updates};
    let publisher_of_sheet = get_password_of_username(publisher_name);
    let publisher = if publisher_of_sheet.is_err() {
        return Err(publisher_of_sheet.err().unwrap());
    } else {
        publisher_of_sheet.unwrap()
    };
    let matching_sheet_name_owned_by_publisher =
        matching_publisher_and_sheet_name(sheet_name, &publisher);

    let sheet_ids_of_matching_publishers_and_sheets =
        matching_sheet_name_owned_by_publisher.iter().map(|sheet| sheet.id).collect::<Vec<Uuid>>();

    // Map and Error handle
    if let Err(result_error) = update_multiple_sheet_elem(new_sheet_elem,
                                                          &sheet_ids_of_matching_publishers_and_sheets) {
        return Err(result_error);
    }

    let sheet_effected_count = new_sheet_elem.len();

    let new_update = NewUpdates {
        owner_id: publisher.id,
        ownership,
        update_value: payload,
    };

    let insert_update_rest =
        diesel::insert_into(updates::table)
            .values(&new_update)
            .returning(Updates::as_returning())
            .get_result(&mut establish_connection());

    if insert_update_rest.is_err() {
        let err_msg = insert_update_rest.err().unwrap().to_string();
        return Err(Result::error(format!("Error in asserting payload in update table. Error: {err_msg}"),
                                 vec![]));
    }

    Ok(sheet_effected_count)
}


fn update_multiple_sheet_elem(new_sheet_elem_vec: &Vec<NewSheetElem>,
                       sheet_ids_of_matching_publishers_and_sheets: &Vec<Uuid>
) -> RustResults<(), Result> {
    use crate::schema::sheet_elems::dsl::{sheet_column_identifier, sheet_row, sheet_id, sheet_value};
    new_sheet_elem_vec.iter().map(|new_sheet_elem: &NewSheetElem| {
        let new_sheet_col = &new_sheet_elem.sheet_column_identifier;
        let new_sheet_row = &new_sheet_elem.sheet_row;
        let new_sheet_value = &new_sheet_elem.sheet_value;
        let sheet_element_to_update: QueryResult<SheetElem> = diesel::update(
            crate::schema::sheet_elems::table
                .filter(sheet_column_identifier.eq(new_sheet_col))
                .filter(sheet_row.eq(new_sheet_row))
                .filter(sheet_id.eq_any(sheet_ids_of_matching_publishers_and_sheets)))
            .set(sheet_value.eq(new_sheet_value))
            .returning(SheetElem::as_returning())
            .get_result(&mut establish_connection());
        if sheet_element_to_update.is_err() {
            let err_msg = sheet_element_to_update.err().unwrap().to_string();
            return Err(Result::error(format!("Error on updating new sheet elements. Error: {err_msg}"),
                                     vec![]));
        }
        Ok(())
    }).collect::<RustResults<(), Result>>()
}

///
///
/// # Arguments
///
/// * `update_id`: The id of the update being sent by the argument object
/// * `ownership_passed_in`: ownership type of the file
/// * `publisher_name`: Name of the publisher passed in by the argument object
/// * `sheet_name`: Name of the sheet passed in by the argument object
///
/// returns: Result<Vec<Updates, Global>, Result>
/// On success it returns all the updates fitting the parameters
///
/// # Examples
///
/// ```
// let list_of_updates = find_updates_by_id_and_ownership(arguement.id,
// Ownership::Publisher, &arguement.publisher, &arguement.sheet);
// Error handle the response from find_updates_by_id_and_ownership
/// ```
pub fn find_updates_by_id_and_ownership(
    update_id: i32,
    ownership_passed_in: Ownership,
    publisher_name: &String,
    sheet_name: &String) -> RustResults<Vec<Updates>, Result> {
    use crate::schema::updates::dsl::{updates, owner_id, id, ownership};
    let publisher_of_sheet = get_password_of_username(publisher_name);
    let publisher = if publisher_of_sheet.is_err() {
        return Err(publisher_of_sheet.err().unwrap());
    } else {
        publisher_of_sheet.unwrap()
    };
    let matching_sheet_name_owned_by_publisher =
        matching_publisher_and_sheet_name(sheet_name, &publisher);

    let sheet_ids_of_matching_publishers_and_sheets =
        matching_sheet_name_owned_by_publisher.iter().map(|sheet| sheet.id).collect::<Vec<Uuid>>();

    let get_updates_based_on_ids_and_ownership = updates
        .filter(owner_id.eq_any(sheet_ids_of_matching_publishers_and_sheets))
        .filter(id.ge(update_id))
        .filter(ownership.eq(ownership_passed_in))
        .select(Updates::as_returning())
        .get_results::<Updates>(&mut establish_connection());

    if get_updates_based_on_ids_and_ownership.is_err() {
        let err_msg = get_updates_based_on_ids_and_ownership.err().unwrap().to_string();
        return Err(Result::error(format!("Issue with getting updates. Error: {err_msg}"),
        vec![]))
    }
    Ok(get_updates_based_on_ids_and_ownership.unwrap())
}

pub fn insert_sheet_relation_elem(new_sheet: &New_Test_Sheet,
                                  new_sheet_elemt: &Vec<NewSheetElem>,
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


pub fn get_sheets_by_a_publisher(publisher: &Publisher) -> Vec<Test_Sheet> {
    use crate::schema::{sheets};
    PublisherSheet::belonging_to(publisher)
        .inner_join(sheets::table)
        .select(Test_Sheet::as_select())
        .load(&mut establish_connection())
        .expect("Oops")
}

pub fn matching_publisher_and_sheet_name(sheet_title: &String, publisher: &Publisher)
                                         -> Vec<Test_Sheet> {
    PublisherSheet::belonging_to(publisher)
        .inner_join(sheets::table)
        .filter(title.eq(sheet_title))
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

    let sheets_to_delete: Vec<Test_Sheet> = matching_publisher_and_sheet_name(sheet_title, &publisher_no_err);

    let sheet_ids_to_delete: &Vec<Uuid> =
        &sheets_to_delete.iter().map(|sheet| sheet.id).collect::<Vec<Uuid>>();

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

pub fn get_sheet_id_by_sheet_name(passed_sheet_name: &String) -> RustResults<Uuid, Result> {
    use crate::schema::sheets::dsl::{sheets, title, id};
    let res = sheets
        .filter(title.eq(passed_sheet_name))
        .select(id)
        .get_result(&mut establish_connection());

    if res.is_err() {
        let err_msg = res.err().unwrap().to_string();
        return Err(Result::error(format!("Error Getting Sheet: {err_msg}"), vec![]));
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
    pub publisher_id: Uuid,
    pub sheets_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = publisher_sheets)]
struct NewPublisherSheet {
    pub publisher_id: Uuid,
    pub sheets_id: Uuid,
}