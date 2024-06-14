use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web::{Error as ActixError};
use crate::database::password_and_username_in_db;

// @author Daniel Kaplan
pub async fn do_auth(
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