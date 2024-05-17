use actix_web::{web, App, HttpServer};
use database::DataStructure;
use std::{collections::HashMap, fs::OpenOptions, future::IntoFuture, sync::Mutex};
mod database;
mod results;
mod serverRequest;
mod users;
use serverRequest::{echo, hello, manual_hello, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let new_db = DataStructure::default();
        let db: web::Data<Mutex<DataStructure>> = actix_web::web::Data::new(Mutex::new(new_db));
        App::new()
            .service(hello)
            .service(echo)
            .app_data(db)
            .service(register)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
