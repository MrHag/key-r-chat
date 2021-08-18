use chrono::*;
use serde_derive::{Deserialize, Serialize};

#[crud_table(table_name:users)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u32>,
    pub login: Option<String>,
    pub password_hash: Option<String>,
    pub avatar: Option<String>,
    pub about_me: Option<String>,
}
impl User {
    pub fn new(login: String, password_hash: String) -> User {
        User {
            id: None,
            login: Some(login),
            password_hash: Some(password_hash),
            avatar: None,
            about_me: Some("".to_owned()),
        }
    }
}

#[crud_table(table_name:tokens)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub id: Option<u32>,
    pub token: Option<String>,
    pub exp_date: Option<NaiveDateTime>,
    pub user_id: Option<u32>,
}
