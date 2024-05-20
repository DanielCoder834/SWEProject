// Needeed Functions
// Result getPublishers()
// Result createSheet(Argument)
// Result getSheets(Argument)
// Result deleteSheet(Argument)
// Result getUpdatesForSubscription(Argument)
// Result getUpdatesForPublished(Argument)
// Result updatePublished(Argument)
// Result updateSubscription(Argument)

use std::fmt::{Display, Formatter};
use std::sync::Mutex;

// Third Party Libraries
use actix_web::{error, HttpRequest, web};
use actix_web::{get, HttpResponse, post, put, Responder};
use actix_web::http::StatusCode;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::headers::authorization::Basic;
use base64::prelude::*;

// Our files/structs
use crate::database;
use crate::publisher;
use crate::results;
use crate::results::Result;

// Type Aliasing
type Argument = results::Argument;
type DataStructure = database::DataStructure;

#[derive(Debug)]
enum CustomError {
    UsernameAlreadyExistsErr,
    UserNameOrPasswordNotProvidedErr,
    DecodingErr,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            &CustomError::UsernameAlreadyExistsErr => write!(f, "{}", "User name already exists. Please be original"),
            &CustomError::UserNameOrPasswordNotProvidedErr => write!(f, "{}", "Missing Password or Username"),
            &CustomError::DecodingErr => write!(f, "{}", "Oops, unable to decode text"),
        }
    }
}

impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::UsernameAlreadyExistsErr => StatusCode::NOT_ACCEPTABLE,
            CustomError::UserNameOrPasswordNotProvidedErr => StatusCode::PARTIAL_CONTENT,
            CustomError::DecodingErr => StatusCode::NOT_ACCEPTABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .body(self.to_string())
    }
}

#[put("/api/v1/register")]
pub async fn register(
    db: web::Data<Mutex<DataStructure>>,
    req: HttpRequest,
) -> std::result::Result<impl Responder, CustomError> {
    let res: Result = Result::default();
    let user_pass_base64_vec = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .split(' ').collect::<Vec<&str>>();
    let user_pass_base64 = if user_pass_base64_vec.len() != 2 {
        return Err(CustomError::UserNameOrPasswordNotProvidedErr);
    } else {
        user_pass_base64_vec[1]
    };
    let decoded_user_name_res = String::from_utf8(match BASE64_STANDARD.decode(user_pass_base64) {
        Err(_) => return Err(CustomError::DecodingErr),
        Ok(v) => v,
    });
    let decoded_user_name = match decoded_user_name_res {
        Err(_) => return Err(CustomError::DecodingErr),
        Ok(v) => v,
    };
    let authArr= if decoded_user_name.split(":").collect::<Vec<&str>>().len() >= 2 {
        decoded_user_name.split(":").collect::<Vec<&str>>()
    } else {
        return Err(CustomError::UserNameOrPasswordNotProvidedErr);
    };
    db.lock().unwrap().add(
        publisher::Publisher::new(
            authArr[0].to_string(),
            authArr[1].to_string(),
        ),
        res,
    );
    if db.lock().unwrap().addCredentials(authArr[0], authArr[1]).is_err() {
        return Err(CustomError::UsernameAlreadyExistsErr);
    }
    Ok(web::Json(""))
}

// #[get("/api/vi/getPublishers")]
// async fn getPublishers() {}

// #[put("/api/vi/createSheet")]
// async fn createSheet(req_body: Argument) {}

// #[get("/api/vi/getSheets")]
// async fn getSheets(req_body: Argument) {}

// #[delete("/api/vi/deleteSheet")]
// async fn deleteSheet(req_body: Argument) {}

// #[get("/api/vi/getUpdatesForSubscription")]
// async fn getUpdatesForSubscription(req_body: Argument) {}

// #[get("/api/vi/getUpdatesForPublished")]
// async fn getUpdatesForPublished(req_body: Argument) {}

// #[post("/api/vi/updatePublished")]
// async fn updatePublished(req_body: Argument) {}
// #[post("/api/vi/updateSubscription")]
// async fn updateSubscription(req_body: Argument) {}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
