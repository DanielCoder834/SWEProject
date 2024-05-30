// Needeed Functions
// Result getPublishers()
// Result createSheet(Argument)
// Result getSheets(Argument)
// Result deleteSheet(Argument)
// Result getUpdatesForSubscription(Argument)
// Result getUpdatesForPublished(Argument)
// Result updatePublished(Argument)
// Result updateSubscription(Argument)

use std::fmt::{Display};
use std::sync::Mutex;

// Third Party Libraries
use actix_web::{error, HttpRequest, web};
use actix_web::{get, HttpResponse, post, put, Responder};
use base64::prelude::*;

// Our files/structs
use crate::database;
use crate::publisher;
use crate::results;
use crate::results::Result;

// Type Aliasing
type Argument = results::Argument;
type DataStructure = database::DataStructure;


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
    db: web::Data<Mutex<DataStructure>>,
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
    if db.lock().unwrap().addCredentials(
        auth_vector[0],
        auth_vector[1]).is_err() {
        return web::Json(Result::error("Username already exists".to_string(), vec![]));
    }
    let successfull_result = Result::new(
        true,
        "Registered Successfully".to_string(),
        vec![]
    );
    db.lock().unwrap().add(
        publisher::Publisher::new(
            auth_vector[0].to_string(),
            auth_vector[1].to_string(),
        ),
        &successfull_result,
    );
    web::Json(successfull_result)
}

// #[get("/api/vi/getPublishers")]
// async fn getPublishers() {}

/* Written by Brooklyn Schmidt
- Deserializes Argument Json Object
- Gets the publisher from the database
- Creates a new sheet and updates database
*/

#[put("/api/vi/createSheet")]
async fn createSheet(req_body: web::Json<Argument>, db: web::Data<Mutex<DataStructure>>) {
    let argument_given: Argument = req_body.into_inner();

    let mut this_publisher = match db.get(&argument_given.publisher) {
        Some(publisher_ref) => publisher_ref,
        None => return web::Json(Result::new(
            false,
            "Publisher not found".to_string(),
            vec![]
        )),
    };

    let mut publisher_sheet_list = this_publisher.get_sheet_list();

    for sheet in publisher_sheet_list {
        if (sheet.name() == &argument_given.sheet && sheet.owner() == &argument_given.publisher.username()) {
            let failed_result = Result::new(
                false,
                "A sheet you own already has that name!".to_string(),
                vec![]
            );
            return web::Json(failed_result);
        }
    }
    
    publisher_sheet_list.push(Sheet::new(
        &argument_given.publisher, 
        &argument_given.sheet));

    let successful_result = Result::new(
        true, 
        "Created a new sheet!".to_string(),
         vec![]);

    db.lock().unwrap().update(this_publisher, &successful_result);

    web::Json(successful_result)
}

/* Written by Brooklyn Schmidt
- Deserializes Argument Json Object
- Gets the publisher from the database
- Gets list of sheets that they have
*/

#[get("/api/vi/getSheets")]
async fn getSheets(req_body: web::Json<Argument>, db: web::Data<Mutex<DataStructure>>) {
    let argument_given: Argument = req_body.into_inner();

    let this_publisher = match db.get(&argument_given.publisher) {
        Some(publisher_ref) => publisher_ref,
        None => return web::Json(Result::new(
            false,
            "Publisher not found".to_string(),
            vec![]
        )),
    };

    let sheets = this_publisher.get_sheet_list();

    let mut list_of_arguments : Vec<Argument> = vec![];

    for sheet in sheets {
        let add_argument : Argument = Argument::new(
            &argument_given.publisher,
            &argument_given.sheet,
            "".to_string(),
            "".to_string(),
        )
        list_of_arguments.push(add_argument);
    }

    let result = Result::new(
        true, 
        "Sheets retrieved successfully", 
        list_of_arguments);

    return web::Json(result);

}

/* Written by Brooklyn Schmidt
- Deserializes Json Object
- Retrieves list of sheets from given Publisher
- Deletes sheet of name "sheet" from vector
- Update database
*/

#[delete("/api/vi/deleteSheet")]
async fn deleteSheet(req_body: Argument) {
    let argument_given: Argument = req_body.into_inner();

    let this_publisher = match db.get(&argument_given.publisher) {
        Some(publisher_ref) => publisher_ref,
        None => return web::Json(Result::new(
            false,
            "Publisher not found".to_string(),
            vec![]
        )),
    };

    let mut publisher_sheet_list = this_publisher.get_sheet_list();

    let mut count = 0;
    let mut found = false;
    for sheet in publisher_sheet_list {
        if sheet.name == &argument_given.sheet {
            found = true;
            break;
        }
        count += 1;
    }

    if found {
        publisher_sheet_list.remove(count);
    } else {
        return web::Json(Result::new(
            false,
            "Sheet name not found",
            vec![],
        ))
    }

    let successful_result = Result::new(
        true, 
        "Deleted sheet".to_string(),
         vec![]);

    db.lock().unwrap().update(this_publisher, &successful_result);

    web::Json(successful_result);

}

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
