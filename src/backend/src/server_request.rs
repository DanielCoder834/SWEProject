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
// use diesel::row::NamedRow;

// Our files/structs
use crate::database;
use crate::database::{get_password_of_username, insert_new_credentials, insert_sheet_relation_elem, password_and_username_in_db};
use crate::publisher::{NewPublisherCredentials, Publisher};
// use crate::publisher;
use crate::results::*;
use crate::sheet::NewSheetElem;

// Modules

// Type Aliasing
type DataStructure = database::DataStructure;
type RustResult<T, E> = std::result::Result<T, E>;


/*
 * Written by Daniel. K
 * Simple: Registers a new user to the database
 * Pipeline from header element to username and password:
 * Header Elements { ..., Authentication: <base64 encoded string>, ... } ->
 * .get("Auth").split(" ") -> vec![Basic, <base64 encoded string>] ->
 * String::utf8(Decode(<base64 encoded string>)) -> username:password ->
 * username:password.split(" ") -> vec[username, password] :)
 */
#[put("/api/v1/register")]
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
    if password_and_username_in_db(auth_vector[0].to_string(),
                                                    auth_vector[1].to_string()) {
        return web::Json(Result::error("Username already exists".to_string(), vec![]));
    }

    let result_cred_insert = insert_new_credentials(
        auth_vector[0],
        auth_vector[1]
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
        vec![]
    );
    // db.lock().unwrap().add(
    //     publisher::Publisher::new(
    //         auth_vector[0].to_string(),
    //         auth_vector[1].to_string(),
    //     ),
    //     &successfull_result,
    // );
    web::Json(successfull_result)
}

// #[get("/api/vi/getPublishers")]
// async fn getPublishers() {}

/* Written by Brooklyn Schmidt
- Deserializes Argument Json Object
- Gets the publisher from the database
- Creates a new sheet and updates database
*/

#[post("/api/v1/createSheet")]
async fn createSheet(argument: web::Json<Argument>)
    -> impl Responder  {
    let publisher_name: &String = &argument.publisher;
    let sheet_name: &String = &argument.sheet;
    let payload: &String = &argument.payload;

    let result_decoding_sheet: RustResult<NewSheetElem, String> = decoded_sheet(payload, sheet_name);
    let new_sheet_element: NewSheetElem = if result_decoding_sheet.is_ok() {
        result_decoding_sheet.unwrap()
    } else {
        let err_msg = result_decoding_sheet.err().unwrap();
        return web::Json(Result::error(
            format!("Sheet Encoding is not correct - Payload: {payload} - Error Msg: {err_msg}").to_string(),
            vec![argument.into_inner()]));
    };

    let result_publisher_of_sheet = get_password_of_username(publisher_name);
    let publisher_of_sheet = if result_publisher_of_sheet.is_err() {
        return web::Json(result_publisher_of_sheet.err().unwrap());
    } else {
        result_publisher_of_sheet.unwrap()
    };

    let insert_result = insert_sheet_relation_elem(&new_sheet_element,
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
// /* Written by Brooklyn Schmidt
// - Deserializes Argument Json Object
// - Gets the publisher from the database
// - Gets list of sheets that they have
// */
//
// #[get("/api/vi/getSheets")]
// async fn getSheets(req_body: web::Json<Argument>, db: web::Data<Mutex<DataStructure>>) -> impl Responder {
//     let argument_given: Argument = req_body.into_inner();
//
//     let publisher_username = argument_given.publisher;
//     let publisher_password = db.lock().unwrap().getCredentials(publisher_username.as_str()).unwrap();
//
//     let publisher: Publisher = Publisher::new(publisher_username, publisher_password.clone());
//
//     let this_publisher = match db.lock().unwrap().get(publisher) {
//         Some(publisher_ref) => publisher_ref,
//         None => return web::Json(Result::new(
//             false,
//             "Publisher not found".to_string(),
//             vec![]
//         )),
//     };
//
//     let sheets = this_publisher.get_sheet_list();
//
//     let mut list_of_arguments : Vec<Argument> = vec![];
//
//     for sheet in sheets {
//         let add_argument : Argument = Argument::new(
//             (argument_given.clone()).publisher,
//             (argument_given.clone()).sheet,
//             "".to_string(),
//             "".to_string(),
//         );
//         list_of_arguments.push(add_argument);
//     }
//
//     let result = Result::new(
//         true,
//         "Sheets retrieved successfully".to_string(),
//         list_of_arguments);
//
//     return web::Json(result);
// }
//
// /* Written by Brooklyn Schmidt
// - Deserializes Json Object
// - Retrieves list of sheets from given Publisher
// - Deletes sheet of name "sheet" from vector
// - Update database
// */
//
// #[delete("/api/vi/deleteSheet")]
// async fn deleteSheet(req_body: web::Json<Argument>, db: web::Data<Mutex<DataStructure>>) -> impl Responder {
//     let argument_given: Argument = req_body.into_inner();
//     let publisher_username = argument_given.publisher;
//     let publisher_password = db.lock().unwrap().getCredentials(publisher_username.as_str()).unwrap();
//
//     let publisher: Publisher = Publisher::new(publisher_username, publisher_password.clone());
//
//     let this_publisher = match db.lock().unwrap().get(publisher) {
//         Some(publisher_ref) => publisher_ref,
//         None => return web::Json(Result::new(
//             false,
//             "Publisher not found".to_string(),
//             vec![]
//         )),
//     };
//
//     let mut publisher_sheet_list = this_publisher.get_sheet_list();
//
//     let mut count = 0;
//     let mut found = false;
//     for sheet in publisher_sheet_list {
//         if sheet.name == &argument_given.sheet {
//             found = true;
//             break;
//         }
//         count += 1;
//     }
//
//     if found {
//         publisher_sheet_list.remove(count);
//     } else {
//         return web::Json(Result::new(
//             false,
//             "Sheet name not found".to_string(),
//             vec![],
//         ))
//     }
//
//     let successful_result = Result::new(
//         true,
//         "Deleted sheet".to_string(),
//          vec![]);
//
//     db.lock().unwrap().update(publisher.clone(), successful_result.clone());
//
//     web::Json(successful_result)
// }

// #[get("/api/vi/getUpdatesForSubscription")]
// async fn getUpdatesForSubscription(req_body: Argument) {}

// #[get("/api/vi/getUpdatesForPublished")]
// async fn getUpdatesForPublished(req_body: Argument) {}

// #[post("/api/vi/updatePublished")]
// async fn updatePublished(req_body: Argument) {}
// #[post("/api/vi/updateSubscription")]
// async fn updateSubscription(req_body: Argument) {}

#[get("/api/v1/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

fn decoded_sheet(encoded_sheet: &String, sheet_title: &String) -> RustResult<NewSheetElem, String> {
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
        id: 0,
        title: sheet_title.clone(),
        sheet_row: 1,
        sheet_value: value.to_string(),
        sheet_column_identifier: meta_sheet_data.chars().nth(1).expect("PArsing issue").to_string(),
        sheet_id: 0,
    })
}