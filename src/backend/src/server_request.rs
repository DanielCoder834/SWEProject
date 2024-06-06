// Needeed Functions
// Result getPublishers()
// Result createSheet(Argument)
// Result getSheets(Argument)
// Result deleteSheet(Argument)
// Result getUpdatesForSubscription(Argument)
// Result getUpdatesForPublished(Argument)
// Result updatePublished(Argument)
// Result updateSubscription(Argument)

// use std::error::Error;
// use std::fmt::{Display};
// use std::sync::Mutex;


// Third Party Libraries
use actix_web::{HttpRequest, post, web};
use actix_web::{get, HttpResponse, put, Responder};
use base64::prelude::*;
use uuid::Uuid;
// use diesel::row::NamedRow;

// Our files/structs
use crate::database;
use crate::database::{delete_sheet_by_sheet_name_and_user, get_password_of_username, insert_new_credentials, insert_sheet_relation_elem, password_and_username_in_db, get_sheets_by_a_publisher, get_all_publishers, update_sheet_elem, find_updates_by_id_and_ownership};
use crate::publisher::{NewPublisherCredentials, Publisher};
// use crate::publisher;
use crate::results::*;
use crate::sheet::{New_Test_Sheet, NewSheetElem, Test_Sheet};
use crate::updates::{Ownership, Updates};

// Modules

// Type Aliasing
type RustResult<T, E> = std::result::Result<T, E>;


/*
 * Written by Daniel Kaplan
 * Simple: Registers a new user to the database
 * Pipeline from header element to username and password:
 * Header Elements { ..., Authentication: <base64 encoded string>, ... } ->
 * .get("Auth").split(" ") -> vec![Basic, <base64 encoded string>] ->
 * String::utf8(Decode(<base64 encoded string>)) -> username:password ->
 * username:password.split(" ") -> vec[username, password] :)
 */
#[get("/api/v1/register")]
pub async fn register(
    req: HttpRequest,
) -> impl Responder {
    // Decoding base64 string
    let encoded_base64_authentication_header = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .split(' ').collect::<Vec<&str>>();

    let username_and_password_encoded_base64 = if encoded_base64_authentication_header.len() != 2 {
        return web::Json(Result::error("Passed in more than one string for authentication. \n \
        Accept Format (Username and password both encoded 64): Basic username:password \n\
        Denied Format: Basic username1:password1 username2:password2".to_string(), vec![]));
    } else {
        encoded_base64_authentication_header[1]
    };

    let decoded_user_name_result =
        String::from_utf8(
            match BASE64_STANDARD.decode(username_and_password_encoded_base64) {
                Err(_) => return web::Json(
                    Result::error(
                        "Issue with decoding string to utf".to_string(),
                        vec![])),
                Ok(v) => v,
            });

    // username_and_password_unwrapped should look like username:password
    let username_and_password_unwrapped = match decoded_user_name_result {
        Err(_) => return web::Json(Result::error(
            "Issue with unwrapping result".to_string(),
            vec![])),
        Ok(v) => v,
    };

    let auth_vector = if username_and_password_unwrapped.split(":").collect::<Vec<&str>>().len() >= 2 {
        username_and_password_unwrapped.split(":").collect::<Vec<&str>>()
    } else {
        return web::Json(Result::error("Username or password are not provided".to_string(), vec![]));
    };

    // Additions to the database
    if password_and_username_in_db(auth_vector[0],
                                   auth_vector[1]) {
        return web::Json(Result::error("Username or Password already exists".to_string(), vec![]));
    }

    let result_cred_insert = insert_new_credentials(
        auth_vector[0],
        auth_vector[1],
    );
    if result_cred_insert.is_err() {
        // TODO: should credentials that error-ed
        let err_str = result_cred_insert.err().unwrap().to_string();
        return web::Json(Result::error(
            format!("Error on inserting new credentials. Error: {err_str}").to_string(), vec![]));
    }

    let successfull_result = Result::new(
        true,
        "Registered Successfully".to_string(),
        vec![],
    );

    web::Json(successfull_result)
}

/* Written by Daniel Kaplan
- Gets all the publishers from the database
- On success returns all publishers of newly created argument objects
*/
#[get("/api/v1/getPublishers")]
async fn getPublishers() -> impl Responder {
    let all_publishers_result = get_all_publishers();
    if all_publishers_result.is_err() {
        let err_msg = all_publishers_result.err().unwrap().to_string();
        return web::Json(Result::error(format!("Error retrieving all publishers: {err_msg}"),
                             vec![]));
    }
    let all_publishers = all_publishers_result.unwrap()
        .into_iter().map( |publisher|
                    Argument::new(publisher.username,
                                  "".to_string(),
                                  "".to_string(),
                                  "".to_string())).collect::<Vec<Argument>>();

    web::Json(Result::new(true, "Successfully got all publishers".to_string(), all_publishers))
}



/* Written by Brooklyn Schmidt and Daniel Kaplan
- Deserializes Argument Json Object
- Gets the publisher from the database
- Creates a new sheet and updates database
*/

#[post("/api/v1/createSheet")]
async fn createSheet(argument: web::Json<Argument>)
                     -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let result_publisher_of_sheet = get_password_of_username(publisher_name);
    let publisher_of_sheet = if result_publisher_of_sheet.is_err() {
        return web::Json(result_publisher_of_sheet.err().unwrap());
    } else {
        result_publisher_of_sheet.unwrap()
    };

    let sheet_title: &String = &argument.sheet;
    let new_sheet: New_Test_Sheet = New_Test_Sheet {
        title: sheet_title.clone(),
        id: Uuid::new_v4(),
    };

    let payload = &argument.payload;

    // Initial Sheet Element
    let initial_sheet_element: NewSheetElem = if payload.len() != 0 {
        let result_decoding_sheet = decoded_sheet(payload);
        let new_sheet_element: NewSheetElem = if result_decoding_sheet.is_ok() {
            result_decoding_sheet.unwrap()
        } else {
            let err_msg = result_decoding_sheet.err().unwrap();
            return web::Json(Result::error(
                format!("Sheet Encoding is not correct - Payload: {payload} - Error Msg: {err_msg}").to_string(),
                vec![argument.into_inner()]));
        };
        new_sheet_element
    } else {
        // Add error handling for duplicate ids
        NewSheetElem::default()
    };

    // let sheet_id =
    // let payload: &String = &argument.payload;

    let insert_result = insert_sheet_relation_elem(
        &new_sheet,
        &initial_sheet_element,
        &publisher_of_sheet);

    if insert_result.is_err() {
        return web::Json(Result::error(insert_result.err().unwrap(), vec![argument.into_inner()]));
    }

    let successful_result = Result::new(
        true,
        "Created a new sheet!".to_string(),
        vec![]);

    web::Json(successful_result)
}
//
// /* Written by Brooklyn Schmidt and Daniel Kaplan
// - Deserializes Argument Json Object
// - Gets the publisher from the database
// - Gets list of sheets that they have
// */
//
#[post("/api/v1/getSheets")]
async fn getSheets(argument: web::Json<Argument>) -> impl Responder {
    let publisher_username = &argument.publisher;

    let result_publisher_of_sheet = get_password_of_username(publisher_username);
    let publisher_of_sheet = if result_publisher_of_sheet.is_err() {
        return web::Json(result_publisher_of_sheet.err().unwrap());
    } else {
        result_publisher_of_sheet.unwrap()
    };
    let sheets: Vec<Test_Sheet> = get_sheets_by_a_publisher(&publisher_of_sheet);

    let list_of_arguments: Vec<Argument> = sheets.into_iter().map(|sheet| Argument::new(
        publisher_username.clone(), sheet.title, "".to_string(), "".to_string()
    )).collect();

    let result = Result::new(
        true,
        "Sheets retrieved successfully".to_string(),
        list_of_arguments);

    return web::Json(result);
}

//
// /* Written by Brooklyn Schmidt and Daniel Kaplan
// - Deserializes Json Object
// - Retrieves list of sheets from given Publisher
// - Deletes sheet of name "sheet" from vector
// - Update database
// */
//
#[post("/api/v1/deleteSheet")]
async fn deleteSheet(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let sheet_name: &String = &argument.sheet;

    let delete_sheet_result = delete_sheet_by_sheet_name_and_user(publisher_name, sheet_name);

    if delete_sheet_result.is_err() {
        return web::Json(delete_sheet_result.err().unwrap());
    }

    let (sheet_deletion_count, relation_deletion_count) = delete_sheet_result.unwrap();
    let successful_result = Result::new(
        true,
        format!("Deleted sheet: {sheet_deletion_count} - Deleted Relatons: {relation_deletion_count}"),
         vec![]);

    web::Json(successful_result)
}

// Written by Brooklyn Schmidt
// Gets the provided argument's sheet and publisher
// Decodes the payload into a new sheet element
// Updates the sheet with the decoded payload
#[post("api/v1/updatePublished")]
async fn updatePublished(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let sheet_name: &String = &argument.sheet;

    let new_sheet_elem = decoded_sheet(&argument.payload);
    if new_sheet_elem.is_err() {
        return web::Json(Result::new(
            true,
            "Failed to update sheet".to_string(),
            vec![]
        ));
    }
    let unwrapped_new_sheet_elem = new_sheet_elem.unwrap();
    let num_of_rows_updated = update_sheet_elem(&unwrapped_new_sheet_elem, publisher_name, sheet_name, argument.clone().payload, Ownership::Publisher);

    let string_num_of_rows_effect = num_of_rows_updated.unwrap();
    let successful_result : Result = Result::new(
        true,
        (format!("{string_num_of_rows_effect} were affected")),
        vec![]
    );

    web::Json(successful_result)
}

// Written by Brooklyn Schmidt
// Retrieves list of updates for subscribers from database
// Error handles
// Returns argument object
#[get("/api/vi/getUpdatesForSubscription")]
async fn getUpdatesForSubscription(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name : &String = &argument.publisher;
    let sheet_name: &String = &argument.sheet;

    let list_of_updates = find_updates_by_id_and_ownership(argument.id.parse().unwrap(),
                                                           Ownership::Subscriber, publisher_name, sheet_name);

    if list_of_updates.is_err() {
        return web::Json(Result::error("Failed to send updates".to_string(), vec![]));
    }

    let sheet_updates_payload = encoding_updates(list_of_updates.unwrap());
    let successful_argument: Argument = Argument::new(
        publisher_name.to_string(),
        sheet_name.to_string(), 
        argument.clone().id, // needs to be last taken ID
        sheet_updates_payload // map everything to Argument
    );

    let successfull_result: Result =
        Result::new(true, "Successfully retrieved updates for subscription".to_string(), vec![successful_argument]);

    web::Json(successfull_result)

}

// Written by Brooklyn Schmidt
// Retrieves list of updates for publisher from database
// Error handles
// Returns argument object
#[get("/api/vi/getUpdatesForPublished")]
async fn getUpdatesForPublished(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name : &String = &argument.publisher;
    let sheet_name: &String = &argument.sheet;

    let list_of_updates = find_updates_by_id_and_ownership((argument.id).parse().unwrap(),
                                                           Ownership::Publisher, publisher_name, sheet_name);

    if list_of_updates.is_err() {
        return web::Json(Result::error("Failed to send updates".to_string(), vec![]));
    }

    let sheet_updates_payload = encoding_updates(list_of_updates.unwrap());
    let successful_argument: Argument = Argument::new(
        publisher_name.to_string(),
        sheet_name.to_string(),
        argument.clone().id, // needs to be last taken ID
        sheet_updates_payload // map everything to Argument
    );

    let successfull_result: Result =
        Result::new(true, "Successfully retrieved updates for publishers".to_string(), vec![successful_argument]);

    web::Json(successfull_result)
}

// Written by Brooklyn Schmidt
// Gets the provided argument's sheet and publisher
// Decodes the payload into a new sheet element
// Updates the sheet with the decoded payload
#[post("/api/vi/updateSubscription")]
async fn updateSubscription(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let sheet_name: &String = &argument.sheet;

    let new_sheet_elem = decoded_sheet(&argument.payload);
    if new_sheet_elem.is_err() {
        return web::Json(Result::new(
            true,
            "Failed to update sheet".to_string(),
            vec![]
        ));
    }

    let unwrapped_new_sheet_elem: NewSheetElem = new_sheet_elem.unwrap();

    let num_of_rows_updated = update_sheet_elem(&unwrapped_new_sheet_elem, publisher_name, sheet_name, argument.clone().payload, Ownership::Subscriber);

    let unwrapped_num_of_rows = num_of_rows_updated.unwrap();
    let successful_result : Result = Result::new(
        true,
        (format!("{unwrapped_num_of_rows} were affected")),
        vec![]
    );

    web::Json(successful_result)
}

#[get("/api/v1/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

// TODO: Make the decoding possible for multiple entries in a payload
// Recommend using .split("$"), possibly making a small function for a single payload, and then map -> collect<NewSheetElem>
fn decoded_sheet(encoded_sheet: &String) -> RustResult<NewSheetElem, String> {
    if (*encoded_sheet).chars().nth(0).expect("parsing issue").to_string() != "$" {
        return Err("Incorrect Sheet Meta String Length or no $".to_string());
    }
    let values = encoded_sheet.split("\n").collect::<Vec<&str>>();
    if values.len() != 2 {
        return Err("Incorrect Number of strings, must be 2".to_string());
    }
    let meta_sheet_data = values[0];
    // meta_sheet_data.chars().nth(2)
    let value = values[1];
    Ok(NewSheetElem {
        id: Uuid::new_v4(),
        // title: sheet_title.clone(),
        sheet_row: 1,
        sheet_value: value.to_string(),
        sheet_column_identifier: meta_sheet_data.chars().nth(1).expect("Parsing issue").to_string(),
        sheet_id: Uuid::new_v4(),
    })
}

fn encoding_updates(updates: Vec<Updates>) -> String {
    updates.into_iter().map(|update| update.update_value).collect::<String>()
}