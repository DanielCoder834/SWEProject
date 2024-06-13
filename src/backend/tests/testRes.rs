#[cfg(test)]
mod tests {
    // Tip: Use dbg!(<value>) to debug
    use super::*;
    use actix_web::{test, App, web, HttpRequest};
    use actix_web::body::BoxBody;
    use actix_web::dev::{HttpServiceFactory, Service, ServiceResponse, WebService};
    use actix_web::http::header;
    use actix_web::web::Data;
    use libs::server_request::{register, ping, getPublishers, createSheet, getSheets, deleteSheet};
    use libs::results::{Argument, optional_to_string, optional_to_vector, Result, vector_to_optional};
    use base64::prelude::*;
    use actix_web_httpauth::extractors::basic::BasicAuth;
    use actix_web_httpauth::headers::authorization::Basic;
    use actix_http::Request;
    use uuid::Uuid;
    use libs::{getUpdatesForPublished, getUpdatesForSubscription, updatePublished, updateSubscription};


    /* Positive Tests */

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_request_get() {
        let app = make_app(vec![register]).await;
        let resp: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            &Basic::new(Uuid::new_v4().to_string(), Some("pass"))).await;
        assert!(resp.success);
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_ping() {
        let app = test::init_service(App::new().service(ping)).await;
        let req = test::TestRequest::get().uri("/api/v1/ping").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // @author Leo Zhao
    #[actix_web::test]
    async fn test_registration() {
        // Establish a database connection
        // let conn = establish_test_connection();

        // Create the application with the register endpoint
        let app = test::init_service(App::new().service(register)).await;

        // Generate test username and password
        // let username = generate_random_username();
        // let password = generate_random_password();
        let auth_value = format!("{}:{}", Uuid::new_v4().to_string(), Uuid::new_v4().to_string());
        let encoded_auth = BASE64_STANDARD.encode(auth_value);

        // Create the request with Authorization header
        let req = test::TestRequest::get()
            .uri("/api/v1/register")
            .insert_header((header::AUTHORIZATION, format!("Basic {}", encoded_auth)))
            .to_request();

        // Call the endpoint
        let resp = test::call_service(&app, req).await;

        // Validate the response
        assert!(resp.status().is_success());

        // Parse the response body
        let body: Result = test::read_body_json(resp).await;

        // Check if registration was successful
        assert!(body.success); // Registration should be successful
        assert_eq!(optional_to_string(body.message), "Registered Successfully");
    }

    // @author Leo Zhao
    #[actix_web::test]
    async fn test_get_publishers() {
        // Establish a database connection
        // let conn = establish_test_connection();

        // Create the application with the getPublishers endpoint
        let app = test::init_service(App::new().service(getPublishers)).await;

        // Make a GET request to the endpoint
        let req = test::TestRequest::get()
            .uri("/api/v1/getPublishers")
            .to_request();

        // Call the endpoint
        let resp = test::call_service(&app, req).await;

        // Validate the response
        assert!(resp.status().is_success());

        // Parse the response body
        let body: Result = test::read_body_json(resp).await;

        // Check if the response indicates success and contains the list of publishers
        assert!(body.success);
        assert_eq!(optional_to_string(body.message), "Successfully got all publishers");
        assert!(!optional_to_vector(body.value).is_empty()); // Ensure the list of publishers is not empty
    }

    // #[actix_web::test]
    // async fn test_get_publishers_empty_list() {
    //
    //     // Create the application with the getPublishers endpoint
    //     let app = test::init_service(App::new().service(getPublishers)).await;
    //
    //     // Ensure the database is empty before making the request
    //
    //     // Make a GET request to the endpoint
    //     let req = test::TestRequest::get()
    //         .uri("/api/v1/getPublishers")
    //         .to_request();
    //
    //     // Call the endpoint
    //     let resp = test::call_service(&app, req).await;
    //
    //     // Validate the response
    //     assert!(resp.status().is_success());
    //
    //     // Parse the response body
    //     let body: Result = test::read_body_json(resp).await;
    //
    //     // Check if the response indicates success
    //     assert!(body.success);
    //
    //     // Check if the message indicates no publishers found
    //     // assert_eq!(body.get_message(), "No publishers found");
    //
    //     // Check if the list of publishers is empty
    //     assert!(optional_to_vector(body.value).is_empty());
    // }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_create_sheet_correctness() {
        // let routes = vec![
        //     web::resource("/api/v1/register").to(register),
        //     web::resource("/api/v1/createSheet").to(createSheet),
        //     web::resource("/api/v1/getSheets").to(getSheets),
        // ];

        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "".to_string());
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, auth, arg).await;
        assert!(resp_create_sheet.success)
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_get_sheet_correctness() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)
            .service(getSheets)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "".to_string());
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_get_sheets: Result = post_route_result_with_auth("/api/v1/getSheets", &app, auth, arg).await;
        assert!(resp_get_sheets.success)
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_delete_sheet_correctness() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)
            .service(deleteSheet)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "".to_string());
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_delete_sheets: Result = post_route_result_with_auth("/api/v1/deleteSheet", &app, auth, arg).await;
        assert!(resp_delete_sheets.success)
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_update_subscription_correctness() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)
            .service(updateSubscription)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "$A0\nValue".to_string());
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_update_subscription: Result = post_route_result_with_auth("/api/v1/updateSubscription", &app, auth, arg).await;
        assert!(resp_update_subscription.success)
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_update_publisher_correctness() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)
            .service(updatePublished)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "$A0\nValue".to_string());
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_update_publisher: Result = post_route_result_with_auth("/api/v1/updatePublished", &app, auth, arg).await;
        assert!(resp_update_publisher.success)
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_get_update_subscription_correctness() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)
            .service(updateSubscription)
            .service(getUpdatesForSubscription)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let payload = "$A0\nValue".to_string();

        let arg = Argument::new(publisher, sheet_name, "0".to_string(), payload);
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_update_subscription: Result = post_route_result_with_auth("/api/v1/updateSubscription", &app, &auth.clone(), arg.clone()).await;

        let resp_get_update_subscription: Result = post_route_result_with_auth("/api/v1/getUpdatesForSubscription", &app, auth, arg).await;
        dbg!(resp_get_update_subscription.clone());
        assert!(resp_get_update_subscription.success)
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_get_update_publishers_correctness() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)
            .service(updatePublished)
            .service(getUpdatesForPublished)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let payload = "$A0\nValue".to_string();

        let arg = Argument::new(publisher, sheet_name, "0".to_string(), payload);
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_update_publisher: Result = post_route_result_with_auth("/api/v1/updatePublished", &app, &auth.clone(), arg.clone()).await;

        let resp_get_update_publisher: Result = post_route_result_with_auth("/api/v1/getUpdatesForPublished", &app, auth, arg).await;
        // dbg!(resp_get_update_subscription.clone());
        assert!(resp_get_update_publisher.success)
    }

    /* Negative Tests */
    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_update_subscription_no_payload_provided() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)
            .service(updateSubscription)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "".to_string());
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_update_subscription: Result = post_route_result_with_auth("/api/v1/updateSubscription", &app, auth, arg).await;
        assert!(!resp_update_subscription.success &&
            resp_update_subscription.clone().message.unwrap_or_else(|| "".to_string())
                == "Failed to update sheet. Error: No $".to_string())
    }

    // @author Leo Zhao
    #[actix_web::test]
    async fn test_username_already_exists_register() {
        // Create the application with the register endpoint
        let app = test::init_service(App::new().service(register)).await;

        // Generate test username and password
        let username = "testuser";
        let password = "testpass";
        let auth_value = format!("{}:{}", username, password);
        let encoded_auth = BASE64_STANDARD.encode(auth_value);

        // Create the request with Authorization header
        let req = test::TestRequest::get()
            .uri("/api/v1/register")
            .insert_header((header::AUTHORIZATION, format!("Basic {}", encoded_auth)))
            .to_request();

        // Call the endpoint
        let resp = test::call_service(&app, req).await;

        // Validate the response
        assert!(resp.status().is_success());

        // Parse the response body
        let body: Result = test::read_body_json(resp).await;
        // Print the body to the console
        println!("{:?}", body);
        // Check if registration was successful

        assert!(!body.success); // Registration should fail
        assert_eq!(optional_to_string(body.message), "Username or Password already exists");
    }

    // Daniel Kaplan Fixed Issues with the code
    // @author Leo Zhao
    #[actix_web::test]
    async fn test_username_password_not_decoded_register() {
        // Create the application with the register endpoint
        let app = test::init_service(App::new().service(register)).await;

        // Generate test username and password
        let username = "";
        let password = "";
        let auth_value = format!("{}:{}", username, password);
        // let encoded_auth = BASE64_STANDARD.encode(auth_value);

        // Create the request with Authorization header
        let req = test::TestRequest::get()
            .uri("/api/v1/register")
            .insert_header((header::AUTHORIZATION, format!("Basic {}", auth_value)))
            .to_request();

        let resp: Result = test::call_and_read_body_json(&app, req).await;
        // println!("{:?}", body);
        // Check if registration failed due to missing username or password
        assert!(!resp.success);
        assert_eq!(resp.message.unwrap_or_else(|| "".to_string()), "Issue with decoding string to utf".to_string());
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_auth_not_correct_length_register() {
        // Create the application with the register endpoint
        let app = test::init_service(App::new().service(register)).await;

        // Create the request with Authorization header
        let req = test::TestRequest::get()
            .uri("/api/v1/register")
            .insert_header((header::AUTHORIZATION, "Basic"))
            .to_request();

        let resp: Result = test::call_and_read_body_json(&app, req).await;
        // println!("{:?}", body);
        // Check if registration failed due to missing username or password
        assert!(!resp.success);
        assert_eq!(resp.message.unwrap_or_else(|| "".to_string()), "Passed in more than one string for authentication\
        . \n Accept Format (Username and password both encoded 64): \
        Basic username:password \nDenied Format: Basic username1:password1 username2:password2".to_string());
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_username_password_not_provided_register() {
        // Create the application with the register endpoint
        let app = test::init_service(App::new().service(register)).await;

        let auth = BASE64_STANDARD.encode("::");
        // Create the request with Authorization header
        let req = test::TestRequest::get()
            .uri("/api/v1/register")
            .insert_header((header::AUTHORIZATION, format!("Basic {auth}")))
            .to_request();

        let resp: Result = test::call_and_read_body_json(&app, req).await;
        // println!("{:?}", body);
        // Check if registration failed due to missing username or password
        assert!(!resp.success);
        assert_eq!(resp.message.unwrap_or_else(|| "".to_string()), "Username or password are not provided".to_string());
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
                                                       -> impl Service<Request, Response=ServiceResponse<BoxBody>, Error=actix_web::Error> {
        test::init_service(
            App::new()
                .service(routes)).await
    }

    // @author Daniel Kaplan
    async fn get_route_result_with_auth<T: Service<Request, Response=ServiceResponse<BoxBody>,
        Error=actix_web::Error>>(
        path: &str,
        app: &T,
        basicAuth: &Basic,
    ) -> Result {
        let req = test::TestRequest::get().uri(path)
            .insert_header((
                header::AUTHORIZATION,
                basicAuth.clone()
            )).to_request();
        let resp: Result = test::call_and_read_body_json(app, req).await;
        resp
    }

    // @author Daniel Kaplan
    async fn post_route_result_with_auth<T: Service<Request, Response=ServiceResponse<BoxBody>,
        Error=actix_web::Error>>(
        path: &str,
        app: &T,
        basicAuth: &Basic,
        argument: Argument,
    ) -> Result {
        let req = test::TestRequest::post().uri(path)
            .insert_header((
                header::AUTHORIZATION,
                basicAuth.clone()
            ))
            .set_json(argument).to_request();
        let resp: Result = test::call_and_read_body_json(app, req).await;
        resp
    }
}