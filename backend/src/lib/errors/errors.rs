use rej_derive::derive_rej;
use serde_derive::Serialize;
use warp::{
    reject::{self, Reject},
    Rejection, Reply,
};

#[derive_rej("UNAUTHORIZED")]
#[derive(Debug)]
pub struct AuthError {}

#[derive_rej("INVALID_LOGIN_DATA")]
#[derive(Debug)]
pub struct InvalidLoginDataError {}

#[derive_rej("INVALID_REGISTRATION_DATA")]
#[derive(Debug)]
pub struct InvalidRegistrationDataError {}

#[derive_rej("INTERNAL_DATABASE_ERROR")]
#[derive(Debug)]
pub struct InternalDataBaseError {}

#[derive_rej("ERR")]
#[derive(Debug)]
pub struct InvalidRequest {}

#[derive_rej("INVALID_DATA_FORMAT")]
#[derive(Debug)]
pub struct InvalidUserDataFormat {}

#[derive(Serialize)]
pub struct ErrorModel {
    pub message: String,
    pub code: u16,
}

impl Reply for ErrorModel {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}
