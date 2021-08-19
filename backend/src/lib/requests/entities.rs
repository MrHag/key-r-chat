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
    pub avatar: String,
}

#[derive(Deserialize)]
pub struct ChangeAboutRequest {
    pub about: String,
}
