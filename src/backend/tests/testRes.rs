#[cfg(test)]
mod tests {
    // Tip: Use dbg!(<value>) to debug
    use actix_web::{test, App, web};
    use actix_web::body::BoxBody;
    use actix_web::dev::{HttpServiceFactory, Service, ServiceResponse};
    use actix_web::http::header;
    use libs::server_request::
    {register, ping, getPublishers, createSheet, getSheets, deleteSheet};
    use libs::results::{Argument, optional_to_string, Result, optional_to_vector};
    use base64::prelude::*;
    use libs::do_auth;
    use actix_web_httpauth::headers::authorization::Basic;
    use actix_http::Request;
    use actix_web_httpauth::middleware::HttpAuthentication;
    use diesel::dsl::Update;
    use uuid::Uuid;
    use libs::{getUpdatesForPublished, getUpdatesForSubscription, updatePublished, updateSubscription};

    /* Positive Tests */

    /* Positive Route/Auth Tests */
    // Tests the register route
    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_request_register() {
        let app = make_app(vec![register]).await;
        let resp: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            &Basic::new(Uuid::new_v4().to_string(), Some("pass"))).await;
        assert!(resp.success);
    }

    // Basic test example
    // Tests the ping route
    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_ping() {
        let app = test::init_service(App::new().service(ping)).await;
        let req = test::TestRequest::get().uri("/api/v1/ping").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // An in-depth test on the register function
    // @author Leo Zhao
    #[actix_web::test]
    async fn test_registration() {
        // Create the application with the register endpoint
        let app = test::init_service(App::new().service(register)).await;

        // Generate test username and password
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

    // Tests the basic authentication using getPublishers and Register
    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_authentication_correctness() {
        let auth_route = web::scope("")
            .wrap(HttpAuthentication::basic(do_auth))
            .service(getPublishers);
        let app = test::init_service(
            App::new()
                .service(register)
                .service(auth_route)
        ).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let _sheet_name = Uuid::new_v4().to_string();
        let auth = Basic::new(publisher.clone(), Some("pass"));

        let req_register = test::TestRequest::get().uri("/api/v1/register")
            .insert_header((
                header::AUTHORIZATION,
                auth.clone()
            )).to_request();

        test::call_service(&app, req_register).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/getPublishers")
            .insert_header((
                header::AUTHORIZATION,
                auth.clone()
            ))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success())
    }

    // Tests if the get_publishers works
    // @author Leo Zhao
    #[actix_web::test]
    async fn test_get_publishers() {

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

    // Tests that the create sheet route function is correct with the necessary values
    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_create_sheet_correctness() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "".to_string());
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, auth, arg).await;
        assert!(resp_create_sheet.success)
    }

    // Tests that the get sheet route function is correct with the necessary values
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
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "".to_string());
        let _resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_get_sheets: Result = post_route_result_with_auth("/api/v1/getSheets", &app, auth, arg).await;
        assert!(resp_get_sheets.success)
    }

    // Tests that the delete sheet route function is correct with the necessary values
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
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "".to_string());
        let _resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_delete_sheets: Result = post_route_result_with_auth("/api/v1/deleteSheet", &app, auth, arg).await;
        assert!(resp_delete_sheets.success)
    }

    // Tests that the update subscriber route function is correct with the necessary values
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
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "$A0\nValue".to_string());
        let _resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_update_subscription: Result = post_route_result_with_auth("/api/v1/updateSubscription", &app, auth, arg).await;
        assert!(resp_update_subscription.success)
    }

    // Tests that the update publisher route function is correct
    // with the necessary values
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
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "$A0\nValue".to_string());
        let _resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_update_publisher: Result = post_route_result_with_auth("/api/v1/updatePublished", &app, auth, arg).await;
        assert!(resp_update_publisher.success)
    }

    // Tests that the get update from subscriber route function is
    // correct with the necessary values
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
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let payload = "$A0\nValue".to_string();

        let arg = Argument::new(publisher, sheet_name, "0".to_string(), payload);
        let _resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let _resp_update_subscription: Result = post_route_result_with_auth("/api/v1/updateSubscription", &app, &auth.clone(), arg.clone()).await;

        let resp_get_update_subscription: Result = post_route_result_with_auth("/api/v1/getUpdatesForSubscription", &app, auth, arg).await;
        dbg!(resp_get_update_subscription.clone());
        assert!(resp_get_update_subscription.success)
    }

    // Tests that the get update from publisher route function is
    // correct with the necessary values
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
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let payload = "$A0\nValue".to_string();

        let arg = Argument::new(publisher, sheet_name, "0".to_string(), payload);
        let _resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let _resp_update_publisher: Result = post_route_result_with_auth("/api/v1/updatePublished", &app, &auth.clone(), arg.clone()).await;

        let resp_get_update_publisher: Result = post_route_result_with_auth("/api/v1/getUpdatesForPublished", &app, auth, arg).await;
        // dbg!(resp_get_update_subscription.clone());
        assert!(resp_get_update_publisher.success)
    }


    /* Negative Tests */
    /* Negative Route Tests */
    // Tests that the update from subscriber route function handles the issue
    // When no payload is provided
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
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "".to_string());
        let _resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let resp_update_subscription: Result = post_route_result_with_auth("/api/v1/updateSubscription", &app, auth, arg).await;
        assert!(!resp_update_subscription.success &&
            resp_update_subscription.clone().message.unwrap_or_else(|| "".to_string())
                == "Failed to update sheet. Error: No $".to_string())
    }

    // Daniel Kaplan Fixed Issues with the code
    // Tests that the register function handles the issue
    // When the username and password are not base64 decoded
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
    // Tests that the register function handles the issue
    // when the basic authentication fields are not the correct length
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

        // Check if registration failed due to missing username or password
        assert!(!resp.success);
        assert_eq!(resp.message.unwrap_or_else(|| "".to_string()), "Passed in more than one string for authentication\
        . \n Accept Format (Username and password both encoded 64): \
        Basic username:password \nDenied Format: Basic username1:password1 username2:password2".to_string());
    }

    // Tests when the username and password are not provided to the register route
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
        // Check if registration failed due to missing username or password
        assert!(!resp.success);
        assert_eq!(resp.message.unwrap_or_else(|| "".to_string()), "Username or password are not provided".to_string());
    }

    // Tests when the sheet_row id is too big with the create sheet route
    // (The sheet_row is a i32, which has a max of 2147483647)
    // @author Daniel Kaplan
    #[actix_web::test]
    async fn test_payload_row_number_too_large() {
        let over_max_sheet_row = u32::MAX;
        let payload = format!("$A{over_max_sheet_row}\nMaxValue\n");
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), payload);
        let resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet",
                                                                    &app, auth, arg).await;
        assert!(!resp_create_sheet.success &&
            resp_create_sheet.clone().message
                .unwrap_or_else(|| "".to_string())
                == "Sheet Encoding is not correct - Payload: $A4294967295\nMaxValue\n - \
                Error Msg: Could not parse to integer".to_string())
    }

    // @author Daniel Kaplan
    #[actix_web::test]
    async fn delete_non_existent_sheet() {
        let app = test::init_service(App::new()
            .service(register)
            .service(deleteSheet)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let arg = Argument::new(publisher, sheet_name, "".to_string(), "".to_string());
        let resp_delete_sheets: Result = post_route_result_with_auth("/api/v1/deleteSheet", &app, auth, arg).await;
        assert!(resp_delete_sheets.success)
    }

    // @author Daniel Kaplan
    // Test when the id is invalid, in this case empty, when used in the
    // getUpdatesForPublished function
    #[actix_web::test]
    async fn invalid_id_get_updates_publisher() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)
            .service(updatePublished)
            .service(getUpdatesForPublished)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let payload = "$A0\nValue".to_string();

        let arg = Argument::new(publisher, sheet_name, " ".to_string(), payload);
        let _resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let _resp_update_publisher: Result = post_route_result_with_auth("/api/v1/updatePublished", &app, &auth.clone(), arg.clone()).await;

        let resp_get_update_publisher: Result = post_route_result_with_auth("/api/v1/getUpdatesForPublished", &app, auth, arg).await;
        // dbg!(resp_get_update_subscription.clone());
        assert!(!resp_get_update_publisher.success)
    }

    // @author Daniel Kaplan
    // Test when the id is invalid, in this case empty, when used in the
    // getUpdatesForSubscription function
    #[actix_web::test]
    async fn invalid_id_get_updates_subscriber() {
        let app = test::init_service(App::new()
            .service(register)
            .service(createSheet)
            .service(updateSubscription)
            .service(getUpdatesForSubscription)).await;

        // Registering a new user
        let publisher = Uuid::new_v4().to_string();
        let sheet_name = Uuid::new_v4().to_string();
        let auth = &Basic::new(publisher.clone(), Some("pass"));
        let _resp_register: Result = get_route_result_with_auth(
            "/api/v1/register",
            &app,
            auth).await;

        let payload = "$A0\nValue".to_string();

        let arg = Argument::new(publisher, sheet_name, " ".to_string(), payload);
        let _resp_create_sheet: Result = post_route_result_with_auth("/api/v1/createSheet", &app, &auth.clone(), arg.clone()).await;

        let _resp_update_subscription: Result = post_route_result_with_auth("/api/v1/updateSubscription", &app, &auth.clone(), arg.clone()).await;

        let resp_get_update_subscription: Result = post_route_result_with_auth("/api/v1/getUpdatesForSubscription", &app, auth, arg).await;

        assert!(!resp_get_update_subscription.success &&
        resp_get_update_subscription.message.unwrap_or_else(|| "".to_string())
        == "Could not Parse Id".to_string())
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
    // Creates a mock based on the routes passed in.
    // Limited usage as the [#<http verb>(<route>)] ties the function to a type
    // limiting this functions usage
    async fn make_app<T: HttpServiceFactory + 'static>(routes: Vec<T>)
                                                       -> impl Service<Request, Response=ServiceResponse<BoxBody>, Error=actix_web::Error> {
        test::init_service(
            App::new()
                .service(routes)).await
    }

    /// @author Daniel Kaplan
    /// Calls the get endpoint of the provided api route using the authentication
    /// From the basicAuth value and the mock app
    /// # Arguments
    ///
    /// * `path`: The ending path of the api route (eg. /api/v1/foo)
    /// * `app`: The mock app used to make the request
    /// * `basicAuth`: The basic authentication to allow the user to make the request
    ///
    /// returns: Result
    /// The result is response from calling that get endpoint
    #[allow(non_snake_case)]
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

    /// @author Daniel Kaplan
    /// Calls the post endpoint of the provided api route using the authentication
    /// From the basicAuth value and the mock app
    /// # Arguments
    ///
    /// * `path`: The ending path of the api route (eg. /api/v1/foo)
    /// * `app`: The mock app used to make the request
    /// * `basicAuth`: The basic authentication to allow the user to make the request
    ///
    /// returns: Result
    /// The result is response from calling that post endpoint
    #[allow(non_snake_case)]
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