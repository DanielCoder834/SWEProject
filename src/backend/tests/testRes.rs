// use backend;


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use actix_web::{test, App, web, HttpRequest};
    use actix_web::http::header;
    use actix_web::web::Data;
    use libs::server_request::{register, ping};
    use libs::database::DataStructure;
    use libs::results::Result;
    use actix_web_httpauth::extractors::basic::BasicAuth;
    use actix_web_httpauth::headers::authorization::Basic;

    #[actix_web::test]
    async fn test_request_get() {
        // TODO: add more boilerplate functions
        let db_wrapped: web::Data<Mutex<DataStructure>> = actix_web::web::Data::new(Mutex::new(DataStructure::default()));
        // let db = db_wrapped.lock().unwrap().storage;
        let app = test::init_service(
            App::new()
                .app_data(db_wrapped)
                .service(register))
                .await;
        let basicAuth = Basic::new("user", Some("pass"));
        let req = test::TestRequest::put().uri("/api/v1/register")
            .insert_header((
                    actix_web::http::header::AUTHORIZATION,
                    basicAuth
                )).to_request();
        let resp: Result = test::call_and_read_body_json(&app, req).await;
        assert!(resp.success);
    }

    #[actix_web::test]
    async fn test_get() {
        let app = test::init_service(App::new().service(ping)).await;
        let req = test::TestRequest::get().uri("/api/v1/ping").to_request();
        let resp = test::call_service(&app, req).await;
        // println!("WHY: {}", resp.response());
        // assert!(resp.status().is_success());
    }
}