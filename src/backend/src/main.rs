// Library Imports
use actix_web::{
    dev::ServiceRequest, error::ErrorUnauthorized, web, App, Error as ActixError, HttpServer,
};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;

// Our File Modules
pub mod server_request;
mod database;
mod publisher;
mod results;
mod schema;
mod sheet;

// Our File Functions/Structs
use database::DataStructure;
// use libs::database::DatabaseManager;
use server_request::{ping, register, createSheet};
use crate::database::password_and_username_in_db;

async fn do_auth(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let password = if creds.password().is_some() {
        creds.password().unwrap()
    } else {
        return Err((ErrorUnauthorized("Error on optional unwrap of password. Eg.\
         No password provided"), req));
    };

    if creds.user_id() == "user" && password == "pass" {
        return Ok(req)
    } else {
        return Err((ErrorUnauthorized("Not Authorized"), req))
    }
    if password_and_username_in_db(
            creds.user_id().to_string(),
            password.to_string()) {
        Ok(req)
    } else {
        Err((ErrorUnauthorized("Not Authorized"), req))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    HttpServer::new(|| {
        let new_db = DataStructure::default();
        let db: web::Data<Mutex<DataStructure>> = actix_web::web::Data::new(Mutex::new(new_db));
        App::new()
            .service(register)
            .wrap(HttpAuthentication::basic(do_auth))
            .service(ping)
            .service(createSheet)

    })
    .bind_openssl("localhost:9443", builder)?
    .run()
    .await
}
