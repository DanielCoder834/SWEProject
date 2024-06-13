#[cfg(test)]
mod tests {
    // Tip: Use dbg!(<value>) to debug
    use super::*;
    use actix_web::{test, App, web, HttpRequest};
    use actix_web::body::BoxBody;
    use actix_web::dev::{HttpServiceFactory, Service, ServiceResponse, WebService};
    use actix_web::http::header;
    use actix_web::web::Data;
    use libs::server_request::{register, ping};
    use libs::results::Result;
    use actix_web_httpauth::extractors::basic::BasicAuth;
    use actix_web_httpauth::headers::authorization::Basic;
    use actix_http::Request;

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_request_get() {
        let app = make_app(vec![register]).await;
        let resp: Result = get_route_result_with_auth(
            "/api/v1/register",
            app,
            Basic::new("user", Some("pass"))).await;
        assert!(resp.success);
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_get() {
        let app = test::init_service(App::new().service(ping)).await;
        let req = test::TestRequest::get().uri("/api/v1/ping").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }



    /// @author Daniel Kaplan
    /// # Arguments
    ///
    /// * `routes`: The various routes you want to pass in
    ///
    /// # Examples
    ///
    // ```
    // The Register Test
    // ```
    // Don't worry about the logic with impl and T. They are a thing called traits,
    // which are like required functions that needed to be implemented.
    async fn make_app<T: HttpServiceFactory + 'static>(routes: Vec<T>)
        -> impl Service<Request, Response = ServiceResponse<BoxBody>, Error = actix_web::Error> {
        test::init_service(
            App::new()
        .service(register)).await
    }

    // @author Daniel Kaplan
    async fn get_route_result_with_auth<T: Service<Request, Response = ServiceResponse<BoxBody>,
        Error = actix_web::Error>>(
        path: &str,
        app: T,
        basicAuth: Basic,
    ) -> Result {
        let req = test::TestRequest::get().uri(path)
            .insert_header((
                header::AUTHORIZATION,
                basicAuth
            )).to_request();
        let resp: Result = test::call_and_read_body_json(&app, req).await;
        resp
    }

    // @author Daniel Kaplan
    async fn post_route_result_with_auth<T: Service<Request, Response = ServiceResponse<BoxBody>,
        Error = actix_web::Error>>(
        path: &str,
        app: T,
        basicAuth: Basic,
    ) -> Result {
        let req = test::TestRequest::post().uri(path)
            .insert_header((
                header::AUTHORIZATION,
                basicAuth
            )).to_request();
        let resp: Result = test::call_and_read_body_json(&app, req).await;
        resp
    }
}