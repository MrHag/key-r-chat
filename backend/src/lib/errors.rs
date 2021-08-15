use serde_derive::Serialize;
use warp::{
    reject::{self, Reject},
    Rejection, Reply,
};

pub trait ToRejection: Reject + Default {
    fn rej() -> Rejection;
}

#[derive(Debug, Default)]
pub struct AuthError {}
impl Reject for AuthError {}

#[derive(Debug, Default)]
pub struct LoginError {}
impl Reject for LoginError {}

#[derive(Debug, Default)]
pub struct DataBaseError {}
impl Reject for DataBaseError {}

#[derive(Debug, Default)]
pub struct InternalDataBaseError {}
impl Reject for InternalDataBaseError {}

#[derive(Debug, Default)]
pub struct InvalidRequest {}
impl Reject for InvalidRequest {}

#[derive(Debug, Default)]
pub struct InvalidUserDataFormat {}
impl Reject for InvalidUserDataFormat {}

impl<T: Reject + Default> ToRejection for T {
    fn rej() -> Rejection {
        reject::custom(Self::default())
    }
}

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
