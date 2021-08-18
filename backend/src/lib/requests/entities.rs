use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct RegistrationRequest {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangeAvatarRequest {
    pub login: String,
    pub password: String,
}
