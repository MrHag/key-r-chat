use warp::{
    reject::{self, Reject},
    Rejection,
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
