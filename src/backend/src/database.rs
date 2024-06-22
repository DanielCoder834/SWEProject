// Third Party Libraries
use std::env;

use diesel::dsl::exists;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::select;
use dotenv::dotenv;
use uuid::Uuid;

use crate::{results, schema};
// Our Files
use crate::publisher::*;
use crate::publisher::Publisher;
use crate::results::optional_to_string;
use crate::schema::{publisher_sheets, sheets};
use crate::schema::sheets::title;
use crate::sheet::{NewSheets, NewSheetElem, SheetElem, Sheets};
use crate::updates::{NewUpdates, Ownership, Updates};
// use crate::updates::Ownership::Publisher;

// Type Aliasing
type Result = results::Result;
type RustResults<T, E> = std::result::Result<T, E>;

// @author Daniel Kaplan
// Meant to allow rust code to interact with the postgres database
// The database is identified through the DATABASE_URL
// One of the few functions that panics.
// If it does, you need to set the DATABASE_URL in your .env correctly
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connect = PgConnection::establish(&database_url);
    match connect {
        Ok(v) => {
            v
        }
        Err(_) => {
            panic!("Error connecting to {}", database_url)
        }
    }
}

// @author Daniel Kaplan
// Inserts new publishers into the database
// Used in register
pub fn insert_new_credentials(username: &str, password: &str) -> QueryResult<Publisher> {
    let new_credentials = NewPublisherCredentials {
        id: &Uuid::new_v4(),
        username,
        password,
    };
    diesel::insert_into(schema::publishers::table)
        .values(&new_credentials)
        .returning(Publisher::as_returning())
        .get_result(&mut establish_connection())
}

// @author Daniel Kaplan
// Gets all the publishers through a basic select call, the error handling is done in the function that calls this
pub fn get_all_publishers() -> QueryResult<Vec<Publisher>> {
    use crate::schema::publishers::dsl::publishers;
    publishers
        .select(Publisher::as_select())
        .get_results(&mut establish_connection())
}


/// @author Daniel Kaplan
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
                         payload: Option<String>,
                         ownership: Ownership)
                         -> RustResults<usize, Result> {
    // use crate::schema::sheet_elems::dsl::{sheet_column_identifier, sheet_row, sheet_id};
    use crate::schema::updates;
    let publisher_of_sheet = get_publisher_of_username(publisher_name);
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
        sheet_id: *sheet_ids_of_matching_publishers_and_sheets.first().unwrap(),
        owner_id: publisher.id,
        ownership,
        update_value: optional_to_string(payload),
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


/// @author Daniel Kaplan
/// # Arguments
///
/// * `new_sheet_elem_vec`: The elements being updates
/// * `sheet_ids_of_matching_publishers_and_sheets`: Ids of which sheets to update
///
/// returns: Result<(), Result>
/// If it errs, it will that in the form of a result object.
/// Nothing happens otherwise
///
fn update_multiple_sheet_elem(new_sheet_elem_vec: &Vec<NewSheetElem>,
                       sheet_ids_of_matching_publishers_and_sheets: &Vec<Uuid>
) -> RustResults<(), Result> {
    use crate::schema::sheet_elems::dsl::{sheet_column_identifier, sheet_id, sheet_row, sheet_value, sheet_elems};
    new_sheet_elem_vec.iter().map(|new_sheet_elem: &NewSheetElem| {
        let new_sheet_value = &new_sheet_elem.sheet_value;
        let sheet_element_to_update: QueryResult<usize> =
            diesel::insert_into(sheet_elems)
                .values(new_sheet_elem)
                .on_conflict((sheet_row, sheet_column_identifier, sheet_id))
                .do_update()
                .set(sheet_value.eq(new_sheet_value))
                .execute(&mut establish_connection());
        if sheet_element_to_update.is_err() {
            let err_msg = sheet_element_to_update.err().unwrap().to_string();
            return Err(Result::error(format!("Error on updating new sheet elements. Error: {err_msg}"),
                                     vec![]));
        }
        Ok(())
    }).collect::<RustResults<(), Result>>()
}

/// @author Daniel Kaplan
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
    use crate::schema::updates::dsl::{id, owner_id, ownership, updates};
    use crate::schema::publishers::dsl::id as publisher_id;
    let publisher_of_sheet = get_publisher_of_username(publisher_name);
    let publisher = if publisher_of_sheet.is_err() {
        return Err(publisher_of_sheet.err().unwrap());
    } else {
        publisher_of_sheet.unwrap()
    };
    let matching_sheet_name_owned_by_publisher =
        matching_publisher_and_sheet_name(sheet_name, &publisher);

    let publisher_ids_of_sheet_updates =
        matching_sheet_name_owned_by_publisher.iter().map(|sheet| {
            PublisherSheet::belonging_to(sheet)
                .inner_join(schema::publishers::table)
                .select(publisher_id)
                .load(&mut establish_connection())
                .unwrap()
        }).flatten().collect::<Vec<Uuid>>();

    let get_updates_based_on_ids_and_ownership = updates
        .filter(owner_id.eq_any(publisher_ids_of_sheet_updates))
        .filter(id.gt(update_id))
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

/// @author Daniel Kaplan
///
/// # Arguments
///
/// * `new_sheet`: The new sheet to be inserted
/// * `new_sheet_elemt`: Initial elements to insert into the database
/// * `publisher`: The publisher object being inserted
///
/// returns: Result<(), String>
/// Returns the string of the error message and some more information if it errors
/// Nothing gets return if it succeeds
///
/// This function is used to insert a publisher, sheet and connecting them to a many-to-many relationship
/// Through the publisher_sheet table
pub fn insert_sheet_relation_elem(new_sheet: &NewSheets,
                                  new_sheet_elemt: &Vec<NewSheetElem>,
                                  publisher: &Publisher) -> RustResults<(), String> {
    use crate::schema::{publisher_sheets, sheet_elems, sheets};

    // Inserting new sheet
    let insert_sheet_result =
        diesel::insert_into(sheets::table)
            .values(new_sheet)
            .returning(Sheets::as_returning())
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

/// @author Daniel Kaplan
///
/// # Arguments
///
/// * `publisher`: The publisher object to identify which sheets belongs to that user
///
/// returns: Vec<Test_Sheet>
/// Returns the sheets belonging to a publisher
pub fn get_sheets_by_a_publisher(publisher: &Publisher) -> Vec<Sheets> {
    use crate::schema::sheets;
    PublisherSheet::belonging_to(publisher)
        .inner_join(sheets::table)
        .select(Sheets::as_select())
        .load(&mut establish_connection())
        .expect("Oops")
}

// @author Daniel Kaplan
// Used to fetch all the sheets owned by a user with a specific name of the sheet
pub fn matching_publisher_and_sheet_name(sheet_title: &String, publisher: &Publisher)
                                         -> Vec<Sheets> {
    PublisherSheet::belonging_to(publisher)
        .inner_join(sheets::table)
        .filter(title.eq(sheet_title))
        .select(Sheets::as_select())
        .load(&mut establish_connection())
        .expect("Oops")
}

/// @author Daniel Kaplan
///
/// # Arguments
///
/// * `publisher_name`: The name of the publisher who owns the sheet
/// * `sheet_title`: The title of the sheet being deleted
///
/// returns: Result<(usize, usize, usize), Result>
/// On success returns a tuple representing the number of the sheets deleted,
/// the number of sheet to publisher relations deleted and the number of sheet elements deleted, respectively
///
/// Returns the error Result object if the function does not succeed
pub fn delete_sheet_by_sheet_name_and_user(publisher_name: &String, sheet_title: &String) -> RustResults<(usize, usize, usize), Result> {
    // use crate::schema::{sheet_elems};
    use crate::schema::publisher_sheets::dsl::{publisher_sheets, sheets_id};
    use crate::schema::sheets::dsl::{id, sheets};
    use crate::schema::sheet_elems::dsl::{sheet_elems, sheet_id};
    use crate::schema::updates::dsl::{updates, sheet_id as update_sheet_id};

    let publisher = get_publisher_of_username(publisher_name);
    let publisher_no_err = if publisher.is_err() {
        return Err(publisher.err().unwrap());
    } else {
        publisher.unwrap()
    };

    let sheets_to_delete: Vec<Sheets> = matching_publisher_and_sheet_name(sheet_title, &publisher_no_err);

    let sheet_ids_to_delete: &Vec<Uuid> =
        &sheets_to_delete.iter().map(|sheet| sheet.id).collect::<Vec<Uuid>>();

    let delete_sheet_relation =
        diesel::delete(publisher_sheets.filter(
            sheets_id.eq_any(sheet_ids_to_delete)
        )).execute(&mut establish_connection());
    let delete_sheet_relation_result = if delete_sheet_relation.is_err() {
        let err_msg = delete_sheet_relation.err().unwrap().to_string();
        return Err(Result::error(err_msg, vec![]));
    } else {
        delete_sheet_relation.unwrap()
    };

    let delete_sheet_elem_result =
        diesel::delete(sheet_elems.filter(
            sheet_id.eq_any(sheet_ids_to_delete)
        )).execute(&mut establish_connection());

    if delete_sheet_elem_result.is_err() {
        let err_msg = delete_sheet_elem_result.err().unwrap().to_string();
        return Err(Result::error(err_msg, vec![]));
    }

    let delete_sheet_result =
        diesel::delete(sheets.filter(id.eq_any(sheet_ids_to_delete)))
            .execute(&mut establish_connection());
    if delete_sheet_result.is_err() {
        let err_msg = delete_sheet_result.err().unwrap().to_string();
        return Err(Result::error(err_msg, vec![]))
    }

    let delete_update_result =
        diesel::delete(updates.filter(update_sheet_id.eq_any(sheet_ids_to_delete)))
            .execute(&mut establish_connection());

    if delete_update_result.is_err() {
        let err_msg = delete_update_result.err().unwrap().to_string();
        return Err(Result::error(err_msg, vec![]));
    }

    let final_tuple = (delete_sheet_result.unwrap(), delete_sheet_relation_result, delete_sheet_elem_result.unwrap());
    if final_tuple.eq(&(0, 0, 0)) {
        return Err(Result::error("Deleted Nothing".to_string(), vec![]))
    }
    Ok(final_tuple)
}

// @author Daniel Kaplan
// Gets the Publisher with the matching username passed in.
pub fn get_publisher_of_username(passed_username: &String) -> RustResults<Publisher, Result> {
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

// @author Daniel Kaplan
// gets the uuid of a sheet based on the name of the sheet passed in
// Used primarily in the get update route functions
pub fn get_sheet_id_by_sheet_name(passed_sheet_name: &String) -> RustResults<Uuid, Result> {
    use crate::schema::sheets::dsl::{id, sheets, title};
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

// @author Daniel Kaplan
// Checks if the username and password exists in the database
pub fn password_and_username_in_db(auth_username: &str, auth_password: &str) -> bool {
    use crate::schema::publishers::dsl::{password, publishers, username};
    let exists_credentials = select(exists(publishers
        .filter(username.eq(auth_username))
        .filter(password.eq(auth_password))))
        .get_result(&mut establish_connection());
    return exists_credentials.unwrap();
}

// @author Daniel Kaplan
#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Publisher))]
#[diesel(belongs_to(Sheets))]
#[diesel(table_name = publisher_sheets)]
#[diesel(primary_key(sheets_id, publisher_id))]
struct PublisherSheet {
    pub publisher_id: Uuid,
    pub sheets_id: Uuid,
}

// @author Daniel Kaplan
#[derive(Insertable)]
#[diesel(table_name = publisher_sheets)]
struct NewPublisherSheet {
    pub publisher_id: Uuid,
    pub sheets_id: Uuid,
}