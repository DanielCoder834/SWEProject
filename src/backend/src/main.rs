use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};

use std::env;
use actix_cors::Cors;
// Library Imports
use actix_web::{
    dev::ServiceRequest, error::ErrorUnauthorized, web, App, Error as ActixError, HttpServer,
};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// Our File Modules
pub mod server_request;
mod database;
mod publisher;
mod results;
mod schema;
mod sheet;
mod updates;

// Our File Functions/Structs
use server_request::{ping, register, createSheet, deleteSheet, getSheets, getPublishers};
use database::{password_and_username_in_db, establish_connection};
use crate::server_request::{getUpdatesForPublished, getUpdatesForSubscription, updatePublished, updateSubscription};

pub const MIGRATION: EmbeddedMigrations = embed_migrations!("./migrations");


// Written by Daniel Kaplan
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
    if password_and_username_in_db(
            creds.user_id(),
            password) {
        Ok(req)
    } else {
        Err((ErrorUnauthorized("Not Authorized"), req))
    }
}

// Written by Daniel Kaplan
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = &mut establish_connection();
    conn.run_pending_migrations(MIGRATION).unwrap();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    HttpServer::new(|| {
        dotenv().ok();

        let cors = Cors::default()
            .allowed_origin(&env::var("CORS_URL").unwrap())
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(env::var("CORS_ENDING_URL").unwrap().as_bytes())
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(7900);

        let authorized_routes = web::scope("")
            .wrap(HttpAuthentication::basic(do_auth))
            .service(createSheet)
            .service(getSheets)
            .service(deleteSheet)
            .service(getPublishers)
            .service(updatePublished)
            .service(updateSubscription)
            .service(getUpdatesForPublished)
            .service(getUpdatesForSubscription);
        App::new()
            .wrap(cors)
            .service(register)
            .service(ping)
            .service(authorized_routes)
    })
        // .bind(("0.0.0.0", 9443))?
    .bind_openssl("0.0.0.0:9443", builder)?
    .run()
    .await
}
