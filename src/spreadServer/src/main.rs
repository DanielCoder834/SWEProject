// Library Imports
use actix_web::{
    dev::ServiceRequest, error::ErrorUnauthorized, web, App, Error as ActixError, HttpServer,
};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;

// Our File Modules
mod database;
mod publisher;
mod results;
mod serverRequest;

// Our File Functions/Structs
use database::DataStructure;
use serverRequest::{echo, hello, manual_hello, register};

async fn do_auth(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    if creds.user_id() == "user" && creds.password() == Some("pass") {
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
            .wrap(HttpAuthentication::basic(do_auth))
            .service(hello)
            .service(echo)
            .app_data(db)
            .service(register)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind_openssl("localhost:9443", builder)?
    .run()
    .await
}
