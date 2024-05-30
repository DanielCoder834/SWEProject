#[derive(PartialEq, Hash, Eq, Debug, serde::Deserialize)]
pub struct Publisher {
    username: String,
    password: String,
    is_authorized: bool,
    sheet_list: Vec<Sheet>,
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

impl Publisher {
    pub fn default() -> Self {
        Publisher {
            username: "".to_string(),
            password: "".to_string(),
            is_authorized: false,
            sheet_list = vec![],
        }
    }
    pub fn new(username: String, password: String) -> Self {
        Publisher {
            username: username.clone(),
            password,
            is_authorized: AuthUserName::is_authorized(&username),
            sheet_list = vec![],
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_sheet_list(&self) -> &Vec<Sheet> {
        &self.sheet_list
    }
}
