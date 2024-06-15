use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web::{Error as ActixError};
use crate::database::password_and_username_in_db;

// @author Daniel Kaplan
// Checks the authentication through basic auth
pub async fn do_auth(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    // Checks that the password exists
    let password = if creds.password().is_some() {
        creds.password().unwrap()
    } else {
        return Err((ErrorUnauthorized("Error on optional unwrap of password. Eg.\
         No password provided"), req));
    };

    // Checks if the username and password exist in the database
    if password_and_username_in_db(
        creds.user_id(),
        password) {
        // They Pass
        Ok(req)
    } else {
        // They Shall Not Pass
        Err((ErrorUnauthorized("Not Authorized"), req))
    }
}