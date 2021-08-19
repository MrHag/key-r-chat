use rej_derive::derive_rej;
use serde_derive::Serialize;
use warp::{Rejection, Reply, reject::Reject};

#[derive_rej("UNAUTHORIZED", 401)]
#[derive(Debug)]
pub struct AuthError {}

#[derive_rej("INVALID_LOGIN_DATA", 400)]
#[derive(Debug)]
pub struct InvalidLoginDataError {}

#[derive_rej("INVALID_REGISTRATION_DATA", 400)]
#[derive(Debug)]
pub struct InvalidRegistrationDataError {}

#[derive_rej("USER_NOT_FOUND", 400)]
#[derive(Debug)]
pub struct UserNotFoundError {}

#[derive_rej("INTERNAL_DATABASE_ERROR", 500)]
#[derive(Debug)]
pub struct InternalDataBaseError {}

#[derive_rej("INVALID_REQUEST", 400)]
#[derive(Debug)]
pub struct InvalidRequest {}

#[derive_rej("INVALID_USERDATA_FORMAT", 400)]
#[derive(Debug)]
pub struct InvalidUserDataFormat {}

#[derive(Serialize, Debug, Clone)]
pub struct ErrorModel {
    pub message: String,
    pub code: u16,
}
impl Reject for ErrorModel {}

impl Reply for ErrorModel {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}
