use std::{convert::Infallible, error::Error};

use chrono::{Duration, Utc};
use hmac::{crypto_mac::InvalidKeyLength, Hmac, Mac, NewMac};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::lib::{context::*, database::entities::Token, error::errors::*, replies::entities::{GetAvatarReply, GetUserReply, LoginReply}, requests::validation::Validation};
use sha2::Sha256;
use warp::{cors::CorsForbidden, hyper::StatusCode, Rejection, Reply};

pub async fn authorize(token: &str, rbdb: ARbDb) -> Result<u32, Rejection> {
    #[crud_table(table_name:users)]
    struct QueryUser {
        pub id: u32,
    }
    match rbdb.get_token(token).await {
        Err(_err) => Err(AuthError::rej()),
        Ok(token) => {
            rbdb.delete_expired_tokens_by_userid(&token.user_id.unwrap())
                .await
                .unwrap();
            match token.exp_date.unwrap() >= Utc::now().naive_utc() {
                false => Err(AuthError::rej()),
                true => match rbdb.user_by_id::<QueryUser>(&token.user_id.unwrap()).await {
                    Err(_err) => Err(InternalDataBaseError::rej()),
                    Ok(user) => Ok(user.id),
                },
            }
        }
    }
}

pub async fn login(login: &str, password: &str, rbdb: ARbDb) -> Result<impl Reply, Rejection> {
    #[crud_table(table_name:users)]
    struct QueryUser {
        pub id: u32,
        pub password_hash: String,
    }

    let user = match rbdb.user_by_login::<QueryUser>(login).await {
        Err(_) => {
            return Err(InvalidLoginDataError::rej());
        }
        Ok(user) => user,
    };

    let phash = hash_string(password, &SC).unwrap();

    if dbg!(phash) != dbg!(user.password_hash) {
        return Err(InvalidLoginDataError::rej());
    }

    let token_str: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let token_duration = Duration::days(5);

    let exp_date = Utc::now()
        .checked_add_signed(token_duration)
        .unwrap()
        .naive_utc();

    let token = Token {
        id: None,
        token: Some(token_str.clone()),
        exp_date: Some(exp_date),
        user_id: Some(user.id),
    };

    match rbdb.save_obj(&token, &[]).await {
        Err(_err) => Err(InternalDataBaseError::rej()),
        Ok(_) => Ok(warp::reply::json(&LoginReply { token: token_str })),
    }
}

pub async fn get_user(user_id: &u32, rbdb: ARbDb) -> Result<impl Reply, Rejection> {
    #[crud_table(table_name:users)]
    struct QueryUser {
        pub id: u32,
        pub login: String,
        pub about_me: String,
    }
    let other_user = match rbdb.user_by_id::<QueryUser>(user_id).await {
        Err(_) => return Err(UserNotFoundError::rej()),
        Ok(user) => user,
    };
    Ok(warp::reply::json(&GetUserReply {
        id: other_user.id,
        login: other_user.login,
        avatar_id: other_user.id,
        about_me: other_user.about_me,
    }))
}

pub async fn get_avatar(user_id: &u32, rbdb: ARbDb) -> Result<impl Reply, Rejection> {
    #[crud_table(table_name:users)]
    struct QueryUser {
        pub avatar: String,
    }
    let other_user = match rbdb.user_by_id::<QueryUser>(user_id).await {
        Err(_) => return Err(UserNotFoundError::rej()),
        Ok(user) => user,
    };
    Ok(warp::reply::json(&GetAvatarReply {
        id: user_id.clone(),
        avatar: other_user.avatar,
    }))
}

pub fn hash_string(str: &str, secret_key: &[u8]) -> Result<String, InvalidKeyLength> {
    let mac = Hmac::<Sha256>::new_from_slice(secret_key);
    match mac {
        Err(err) => Err(err),
        Ok(mut hash) => {
            hash.update(str.as_bytes());
            Ok(hex::encode(&hash.finalize().into_bytes()))
        }
    }
}
#[cfg(not(debug_assertions))]
pub fn try_validate<T: Validation>(to_validate: T) -> Result<T, Rejection> {
    let result = to_validate.validate();
    match result {
        Ok(()) => Ok(to_validate),
        Err(_err) => Err(InvalidUserDataFormat::rej()),
    }
}
#[cfg(debug_assertions)]
pub fn try_validate<T: Validation>(to_validate: T) -> Result<T, Rejection> {
    Ok(to_validate)
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {

    let error = if let Some(obj) = err.find::<ErrorModel>() {
        (*obj).clone()
    } else {
        let (code, message) = if err.is_not_found() {
            (StatusCode::NOT_FOUND, "NOT_FOUND".to_owned())
        } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
            let message = match e.source() {
                Some(cause) => {
                    format!("FIELD_ERROR: {}", cause.to_string())
                }
                None => "BAD_REQUEST".to_owned(),
            };
            (StatusCode::BAD_REQUEST, message)
        } else if let Some(_err) = err.find::<warp::reject::MethodNotAllowed>() {
            (
                StatusCode::METHOD_NOT_ALLOWED,
                "METHOD_NOT_ALLOWED".to_owned(),
            )
        } else if let Some(_err) = err.find::<warp::reject::MissingHeader>() {
            (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED".to_owned(),
            )
        } else if let Some(_err) = err.find::<CorsForbidden>() {
            (
                StatusCode::METHOD_NOT_ALLOWED,
                "METHOD_NOT_ALLOWED".to_owned(),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("UNHANDLED_REJECTION(IT'S A BUG): {:?}", err),
            )
        };

        ErrorModel {
            message,
            code: code.as_u16(),
        }
    };

    let code = StatusCode::from_u16(error.code).unwrap();
    Ok(warp::reply::with_status(error, code))
}
