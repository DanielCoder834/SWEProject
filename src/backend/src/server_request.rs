// Third Party Libraries
use actix_web::{get, HttpResponse, Responder, HttpRequest, post, web};
use base64::prelude::*;
use uuid::Uuid;

// Our files/structs
use crate::database::{delete_sheet_by_sheet_name_and_user, get_publisher_of_username, insert_new_credentials, insert_sheet_relation_elem, get_sheets_by_a_publisher, get_all_publishers, update_sheet_elem, find_updates_by_id_and_ownership, get_sheet_id_by_sheet_name};
use crate::results::*;
// use crate::schema::sheet_elems::sheet_id;
use crate::sheet::{Sheets, NewSheetElem, NewSheets};
use crate::updates::{Ownership, Updates};

// Type Aliasing
type RustResult<T, E> = std::result::Result<T, E>;

/*
 *@author Daniel Kaplan
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

    // Made changes here
    let auth_vector = if username_and_password_unwrapped.split(":").collect::<Vec<&str>>().join("") != "" {
        username_and_password_unwrapped.split(":").collect::<Vec<&str>>()
    } else {
        return web::Json(Result::error("Username or password are not provided".to_string(), vec![]));
    };

    let result_cred_insert = insert_new_credentials(
        auth_vector[0],
        auth_vector[1],
    );
    if result_cred_insert.is_err() {
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

/* @author Daniel Kaplan
- Gets all the publishers from the database
- On success returns all publishers of newly created argument objects
*/
#[allow(non_snake_case)]
#[get("/api/v1/getPublishers")]
async fn getPublishers() -> impl Responder {
    let all_publishers_result = get_all_publishers();
    if all_publishers_result.is_err() {
        let err_msg = all_publishers_result.err().unwrap().to_string();
        return web::Json(Result::error(format!("Error retrieving all publishers: {err_msg}"),
                                       vec![]));
    }
    let all_publishers = all_publishers_result.unwrap()
        .into_iter().map(|publisher|
        Argument::new(publisher.username,
                      "".to_string(),
                      "".to_string(),
                      "".to_string())).collect::<Vec<Argument>>();

    web::Json(Result::new(true, "Successfully got all publishers".to_string(), all_publishers))
}

/* Pair-programmed by Daniel Kaplan and Brooklyn Schmidt
@author Daniel Kaplan
- Deserializes Argument Json Object
- Gets the publisher from the database
- Creates a new sheet and updates database
*/
#[allow(non_snake_case)]
#[post("/api/v1/createSheet")]
async fn createSheet(argument: web::Json<Argument>)
                     -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let result_publisher_of_sheet = get_publisher_of_username(publisher_name);
    let publisher_of_sheet = if result_publisher_of_sheet.is_err() {
        return web::Json(result_publisher_of_sheet.err().unwrap());
    } else {
        result_publisher_of_sheet.unwrap()
    };

    let sheet_title: &String = &optional_to_string(argument.clone().sheet);
    let sheet_id = Uuid::new_v4();
    let new_sheet: NewSheets = NewSheets {
        title: sheet_title.clone(),
        id: sheet_id,
    };

    let payload = optional_to_string(argument.clone().payload);

    // Initial Sheet Element
    let initial_sheet_element: Vec<NewSheetElem> = if payload.len() != 0 {
        let result_decoding_sheet = decoded_sheet(&payload, sheet_id);
        let new_sheet_element: Vec<NewSheetElem> = if result_decoding_sheet.is_ok() {
            result_decoding_sheet.unwrap()
        } else {
            let err_msg = result_decoding_sheet.err().unwrap();
            return web::Json(Result::error(
                format!("Sheet Encoding is not correct - Payload: {payload} - Error Msg: {err_msg}").to_string(),
                vec![argument.into_inner()]));
        };
        new_sheet_element
    } else {
        vec![NewSheetElem::default(sheet_id)]
    };

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


/* Pair-Programmed by Daniel Kaplan and Brooklyn Schmidt
@author Daniel Kaplan
- Deserializes Argument Json Object
- Gets the publisher from the database
- Gets list of sheets that they have
*/

#[allow(non_snake_case)]
#[post("/api/v1/getSheets")]
async fn getSheets(argument: web::Json<Argument>) -> impl Responder {
    let publisher_username = &argument.publisher;

    let result_publisher_of_sheet = get_publisher_of_username(publisher_username);
    let publisher_of_sheet = if result_publisher_of_sheet.is_err() {
        return web::Json(result_publisher_of_sheet.err().unwrap());
    } else {
        result_publisher_of_sheet.unwrap()
    };
    let sheets: Vec<Sheets> = get_sheets_by_a_publisher(&publisher_of_sheet);

    let list_of_arguments: Vec<Argument> = sheets.into_iter().map(|sheet| Argument::new(
        publisher_username.clone(), sheet.title, "".to_string(), "".to_string(),
    )).collect();

    let result = Result::new(
        true,
        "Sheets retrieved successfully".to_string(),
        list_of_arguments);

    return web::Json(result);
}

/* Pair-Programmed by Daniel Kaplan and Brooklyn Schmidt
@author Daniel Kaplan
- Deserializes Json Object
- Retrieves list of sheets from given Publisher
- Deletes sheet of name "sheet" from vector
- Update database
*/

#[allow(non_snake_case)]
#[post("/api/v1/deleteSheet")]
async fn deleteSheet(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let sheet_name: &String = &optional_to_string(argument.clone().sheet);

    let delete_sheet_result = delete_sheet_by_sheet_name_and_user(publisher_name, sheet_name);

    if delete_sheet_result.is_err() {
        return web::Json(delete_sheet_result.err().unwrap());
    }

    let (sheet_deletion_count, relation_deletion_count, sheet_elem_deletion_count) = delete_sheet_result.unwrap();
    let successful_result = Result::new(
        true,
        format!("Deleted sheet: {sheet_deletion_count} -\
         Deleted Relatons: {relation_deletion_count} -\
          Sheet Elem: {sheet_elem_deletion_count}"),
        vec![]);

    web::Json(successful_result)
}


// Pair-Programmed by Daniel Kaplan and Brooklyn Schmidt
// @author Brooklyn Schmidt
// Ensures validity of provided sheet ID and ownership type.
// Retrieves all updated sheet elements from the provided payload
// Updates sheet given Sheet ID
// Returns a Message indicating how many elements were updated.

#[allow(non_snake_case)]
#[post("api/v1/updatePublished")]
async fn updatePublished(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let sheet_name: &String = &optional_to_string(argument.clone().sheet);

    let result_sheet_id = get_sheet_id_by_sheet_name(sheet_name);
    let sheet_id = if let Ok(id) = result_sheet_id {
        id
    } else {
        return web::Json(result_sheet_id.err().unwrap());
    };
    let new_sheet_elem = decoded_sheet(&optional_to_string(argument.clone().payload), sheet_id);
    if new_sheet_elem.is_err() {
        return web::Json(Result::error(
            "Failed to update sheet".to_string(),
            vec![],
        ));
    }
    let unwrapped_new_sheet_elem = if let Ok(new_sheet_elem) = new_sheet_elem {
        new_sheet_elem
    } else {
        return web::Json(Result::error(new_sheet_elem.err().unwrap(), vec![]));
    };
    let num_of_rows_updated = update_sheet_elem(
        &unwrapped_new_sheet_elem, publisher_name, sheet_name,
        argument.clone().payload, Ownership::Publisher);

    if num_of_rows_updated.is_err() {
        return web::Json(num_of_rows_updated.err().unwrap());
    }

    let string_num_of_rows_effect = num_of_rows_updated.unwrap();
    let successful_result: Result = Result::new(
        true,
        format!("{string_num_of_rows_effect} rows were affected"),
        vec![],
    );

    web::Json(successful_result)
}

// Pair-Programmed by Daniel Kaplan and Brooklyn Schmidt
// @author Brooklyn Schmidt
// Retrieves list of updates given the Sheet ID and Ownership Type
// Ensures validity of JSON Argument sent
// Encodes those updates and sends them back in the payload

#[allow(non_snake_case)]
#[post("/api/v1/getUpdatesForSubscription")]
async fn getUpdatesForSubscription(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let sheet_name: &String = &optional_to_string(argument.clone().sheet);

    let sheet_id = if let Ok(id) = optional_to_string(argument.clone().id).parse::<i32>() {
        id
    } else {
        return web::Json(Result::error("Could not Parse Id".to_string(), vec![argument.into_inner()]));
    };

    if sheet_id < 0 {
        return web::Json(Result::error("Negative Id".to_string(), vec![argument.into_inner()]));
    }
    let list_of_updates = find_updates_by_id_and_ownership(sheet_id,
                                                           Ownership::Subscriber, publisher_name, sheet_name);

    if list_of_updates.is_err() {
        return web::Json(Result::error("Failed to send updates".to_string(), vec![]));
    }

    let sheet_updates_payload = encoding_updates(list_of_updates.unwrap());
    let successful_argument: Argument = Argument {
        publisher: publisher_name.to_string(),
        sheet: Some(sheet_name.to_string()),
        id: argument.clone().id,
        payload: Some(sheet_updates_payload),
    };


    let successful_result: Result =
        Result::new(true, "Successfully retrieved updates for subscription".to_string(), vec![successful_argument]);

    web::Json(successful_result)
}

// Pair-Programmed by Daniel Kaplan and Brooklyn Schmidt
// @author Brooklyn Schmidt
// Retrieves list of updates given the Sheet ID and Ownership Type
// Ensures validity of JSON Argument sent
// Encodes those updates and sends them back in the payload
#[allow(non_snake_case)]
#[post("/api/v1/getUpdatesForPublished")]
async fn getUpdatesForPublished(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let sheet_name: &String = &optional_to_string(argument.clone().sheet);

    let sheet_id = if let Ok(id) = optional_to_string(argument.clone().id).parse::<i32>() {
        id
    } else {
        return web::Json(Result::error("Could not Parse Id".to_string(), vec![argument.into_inner()]));
    };
    if sheet_id < 0 {
        return web::Json(Result::error("Negative Id".to_string(), vec![argument.into_inner()]));
    }

    let list_of_updates = find_updates_by_id_and_ownership(sheet_id,
                                                           Ownership::Publisher, publisher_name, sheet_name);

    if list_of_updates.is_err() {
        return web::Json(Result::error("Failed to send updates".to_string(), vec![]));
    }

    let sheet_updates_payload = encoding_updates(list_of_updates.unwrap());
    let successful_argument: Argument = Argument {
        publisher: publisher_name.to_string(),
        sheet: Some(sheet_name.to_string()),
        id: argument.clone().id,
        payload: Some(sheet_updates_payload),
    };

    let successfull_result: Result =
        Result::new(true, "Successfully retrieved updates for publishers".to_string(), vec![successful_argument]);

    web::Json(successfull_result)
}

// Pair-Programmed by Daniel Kaplan and Brooklyn Schmidt
// @author Brooklyn Schmidt
// Ensures validity of provided sheet ID and ownership type.
// Retrieves all updated sheet elements from the provided payload
// Updates sheet given Sheet ID
// Returns a Message indicating how many elements were updated.
#[allow(non_snake_case)]
#[post("/api/v1/updateSubscription")]
async fn updateSubscription(argument: web::Json<Argument>) -> impl Responder {
    let publisher_name: &String = &argument.publisher;
    let sheet_name: &String = &optional_to_string(argument.clone().sheet);

    let sheet_id_result = get_sheet_id_by_sheet_name(sheet_name);
    let sheet_id = if let Ok(sheet_id) = sheet_id_result {
        sheet_id
    } else {
        return web::Json(sheet_id_result.err().unwrap());
    };

    let new_sheet_elem = decoded_sheet(&optional_to_string(argument.clone().payload), sheet_id);
    if new_sheet_elem.is_err() {
        let err_msg = new_sheet_elem.err().unwrap();
        return web::Json(Result::new(
            false,
            format!("Failed to update sheet. Error: {err_msg}"),
            vec![],
        ));
    }

    let unwrapped_new_sheet_elem: Vec<NewSheetElem> = new_sheet_elem.unwrap();

    let num_of_rows_updated = update_sheet_elem(&unwrapped_new_sheet_elem, publisher_name, sheet_name, argument.clone().payload, Ownership::Subscriber);

    if num_of_rows_updated.is_err() {
        return web::Json(num_of_rows_updated.err().unwrap());
    }
    let unwrapped_num_of_rows = num_of_rows_updated.unwrap();
    let successful_result: Result = Result::new(
        true,
        format!("{unwrapped_num_of_rows} were affected"),
        vec![],
    );

    web::Json(successful_result)
}

// @author Daniel Kaplan
// The most basic route.
// Used for manual testing
#[get("/api/v1/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

/// @author Daniel Kaplan
/// Decodes a payload like the one below to transform them into the new sheet element objects
/// Valid Format for encoded_sheet: "$A0\nValue0\n$A1\nValue1\n"
/// Invalid Format for encoded_sheet: "${10^999}\n$AValue1\n"
/// # Arguments
///
/// * `encoded_sheet`: The encoded payload sent to the server
/// * `sheet_id`: The id of the sheet the new elements belong to
///
/// returns: Result<Vec<NewSheetElem>, String>
/// On Success: Returns the vector of decoded sheet elements
/// On Error, returns a string describing the issue
fn decoded_sheet(encoded_sheet: &String, sheet_id: Uuid) -> RustResult<Vec<NewSheetElem>, String> {
    if !(*encoded_sheet).contains("$") {
        return Err("No $".to_string());
    }

    // Split sheet by $
    let lists_of_seperate_sheet_elems = encoded_sheet.split("$").collect::<Vec<&str>>();
    // Remove any empty string sheets
    let no_empty_string_sheet_elems = lists_of_seperate_sheet_elems.into_iter().filter(|payload| !payload.is_empty()).collect::<Vec<&str>>();

    let sheet_elem_vec = no_empty_string_sheet_elems.iter().map(|payload| {
        decode_sheet_elem(&payload.to_string(), sheet_id)
    }).collect::<RustResult<Vec<NewSheetElem>, String>>();

    sheet_elem_vec
}

/// @author Daniel Kaplan
/// Used to decode a single sheet element from a payload
/// # Arguments
///
/// * `encoded_sheet_elem`: The payload of the single sheet element
/// * `sheet_id`: The Uuid of the sheet that the update should belong to
///
/// returns: Result<NewSheetElem, String>
/// On Success: Returns the decoded sheet element
/// On Error, returns a string describing the issue
fn decode_sheet_elem(encoded_sheet_elem: &String, sheet_id: Uuid) -> RustResult<NewSheetElem, String> {
    let values = encoded_sheet_elem.trim().split(" ").collect::<Vec<&str>>();
    let values_length = values.len();
    if values_length != 2 {
        return Err(format!("Must have Position information and value information divided by a newline.\
         Eg. $A0\nValue. String-length: {values_length}, Sheet_elem: {encoded_sheet_elem}"));
    }

    let meta_sheet_data = values[0];
    let char_column = meta_sheet_data.chars().nth(0);
    let str_row: String = meta_sheet_data.chars().skip(1).collect();

    // Parsing Mainly and Error Handling
    // For Sheet Row
    let sheet_row = if let Ok(value) = str_row.parse::<i32>() {
        value
    } else {
        return Err("Could not parse to integer".to_string());
    };

    // For Sheet Column Identifier
    let sheet_column_identifier = if let Some(value) = char_column {
        value.to_string()
    } else {
        return Err("Value is not found at the 1st position".to_string());
    };

    let sheet_value = values[1].to_string();
    Ok(NewSheetElem {
        id: Uuid::new_v4(),
        sheet_row,
        sheet_value,
        sheet_column_identifier,
        sheet_id,
    })
}

// @author Daniel Kaplan
// Takes a vector of updates and concatenate the update_values of those updates to form a payload
// Used primarily in the get update functions
fn encoding_updates(updates: Vec<Updates>) -> String {
    updates.into_iter().map(|mut update| {
        update.update_value
    }).collect::<String>()
}