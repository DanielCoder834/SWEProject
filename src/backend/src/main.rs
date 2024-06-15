use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};

use std::env;
use actix_cors::Cors;
// Library Imports
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::{middleware::HttpAuthentication};
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
mod auth;

// Our File Functions/Structs
use server_request::{ping, register, createSheet, deleteSheet, getSheets, getPublishers};
use database::{establish_connection};
use crate::server_request::{getUpdatesForPublished, getUpdatesForSubscription, updatePublished, updateSubscription};
use auth::do_auth;

pub const MIGRATION: EmbeddedMigrations = embed_migrations!("./migrations");


// @author Daniel Kaplan
// Initializes the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // For Setting up the Database
    let conn = &mut establish_connection();
    conn.run_pending_migrations(MIGRATION).unwrap();

    // For https and ssl configuration
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    HttpServer::new(|| {
        dotenv().ok();

        // Which websites to allow to call the server
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin(&env::var("CORS_URL").unwrap())
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(env::var("CORS_ENDING_URL").unwrap().as_bytes())
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(7900);

        // Basic Auth
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
        // The Server Init
        App::new()
            .wrap(cors)
            .service(register)
            .service(ping)
            .service(authorized_routes)
    })
    .bind_openssl("0.0.0.0:9443", builder)?
    .run()
    .await
}
