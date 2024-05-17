// Needeed Functions
// Result getPublishers()
// Result createSheet(Argument)
// Result getSheets(Argument)
// Result deleteSheet(Argument)
// Result getUpdatesForSubscription(Argument)
// Result getUpdatesForPublished(Argument)
// Result updatePublished(Argument)
// Result updateSubscription(Argument)

// Third Party Libraries
use actix_web::web;
use actix_web::{get, post, put, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use std::sync::Mutex;

// Our files/structs
use crate::database;
use crate::publisher;
use crate::results;
use crate::results::Result;

// Type Aliasing
type Argument = results::Argument;
type DataStructure = database::DataStructure;

#[put("/api/v1/register")]
pub async fn register(
    db: web::Data<Mutex<DataStructure>>,
    creds: BasicAuth,
) -> actix_web::Result<impl Responder> {
    let res: Result = Result::default();
    let res2: Result = Result::default();
    db.lock().unwrap().add(
        publisher::Publisher::new(
            creds.user_id().to_string(),
            creds.password().unwrap().to_string(),
        ),
        res,
    );
    Ok(web::Json(res2))
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
