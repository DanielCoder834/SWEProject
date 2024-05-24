// use spreadServer;


#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use actix_web::{test, App, web};
    use libs::server_request::{register, hello};
    use libs::database::DataStructure;
    use actix_web_httpauth::extractors::basic::BasicAuth;
    use actix_web_httpauth::headers::authorization::Basic;
    use super::*;

    #[actix_web::test]
    async fn test_request_get() {
        let db_wrapped: web::Data<Mutex<DataStructure>> = actix_web::web::Data::new(Mutex::new(DataStructure::default()));
        let db_cloned = db_wrapped.clone();
        // let db = db_wrapped.lock().unwrap().storage;
        let app = test::init_service(
            App::new()
                .app_data(db_wrapped)
                .app_data(BasicAuth::from(
                    Basic::new("user", Some("pass"))))
                .service(register))
                .await;
        let req = test::TestRequest::put().uri("/api/v1/register").to_request();
        let resp = test::call_service(&app, req).await;
        // Make sure the hasmap get's updated
        println!("{:?}", db_wrapped.lock().unwrap().storage);
        assert!(resp.status().is_success());
        assert_ne!(db_wrapped.lock().unwrap().storage, db_cloned.lock().unwrap().storage);
    }

    #[actix_web::test]
    async fn test_get() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        println!("WHY: {}", resp.response());
        // assert!(resp.status().is_success());
    }
}