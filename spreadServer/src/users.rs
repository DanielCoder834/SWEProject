#[derive(PartialEq, Hash, Eq)]
pub struct User {
    username: String,
    password: String,
    is_authorized: bool,
}

enum AuthUserName {
    Brooklyn,
    Daniel,
    Leo,
}

impl AuthUserName {
    fn string_values(&self) -> &str {
        match *self {
            AuthUserName::Brooklyn => "Brooklyn",
            AuthUserName::Daniel => "Daniel",
            AuthUserName::Leo => "Leo",
        }
    }
    fn is_authorized(username: &str) -> bool {
        return AuthUserName::Brooklyn.string_values() == username
            || AuthUserName::Daniel.string_values() == username
            || AuthUserName::Leo.string_values() == username;
    }
}

impl User {
    pub fn default() -> Self {
        User {
            username: "".to_string(),
            password: "".to_string(),
            is_authorized: false,
        }
    }
    fn new(username: String, password: String) -> Self {
        User {
            username: username.clone(),
            password,
            is_authorized: AuthUserName::is_authorized(&username),
        }
    }
}
