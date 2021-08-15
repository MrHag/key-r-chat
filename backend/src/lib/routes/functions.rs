use std::{convert::Infallible, error::Error};

use chrono::{Duration, Utc};
use hmac::{crypto_mac::InvalidKeyLength, Hmac, Mac, NewMac};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::lib::{
    context::*,
    database::entities::{Token, User},
    errors::*,
    replyes::entities::LoginReply,
    requests::validation::Validation,
};
use sha2::Sha256;
use warp::{cors::CorsForbidden, hyper::StatusCode, Rejection, Reply};

pub async fn authorizefn(token: &String, rbdb: ARbDb) -> Result<User, Rejection> {
    match rbdb.get_token(&token).await {
        Err(_err) => Err(AuthError::rej()),
        Ok(token) => {
            rbdb.delete_expired_tokens_by_userid(&token.user_id.unwrap())
                .await
                .unwrap();
            match token.exp_date.unwrap() >= Utc::now().naive_utc() {
                false => Err(AuthError::rej()),
                true => match rbdb.user_by_id(&token.user_id.unwrap()).await {
                    Err(_err) => Err(InternalDataBaseError::rej()),
                    Ok(user) => Ok(user),
                },
            }
        }
    }
}

pub async fn login(
    login: &String,
    password: &String,
    rbdb: ARbDb,
) -> Result<impl Reply, Rejection> {
    let user = match rbdb.user_by_login(&login).await {
        Err(_) => {
            return Err(LoginError::rej());
        }
        Ok(user) => user,
    };

    let phash = hash_string(&password, &SC).unwrap();

    if dbg!(phash) != dbg!(user.password_hash.unwrap()) {
        return Err(LoginError::rej());
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
        user_id: user.id,
    };

    match rbdb.save_obj(&token, &[]).await {
        Err(_err) => Err(InternalDataBaseError::rej()),
        Ok(_) => Ok(warp::reply::json(&LoginReply { token: token_str })),
    }
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

pub fn try_validate<T: Validation>(to_validate: T) -> Result<T, Rejection> {
    let result = to_validate.validate();
    match result {
        Ok(()) => Ok(to_validate),
        Err(_err) => Err(InvalidUserDataFormat::rej()),
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_owned();
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        message = match e.source() {
            Some(cause) => {
                format!("FIELD_ERROR: {}", cause.to_string())
            }
            None => "BAD_REQUEST".to_owned(),
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".to_owned();
    } else if let Some(_) = err.find::<CorsForbidden>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".to_owned();
    } else if let Some(_) = err.find::<AuthError>() {
        code = StatusCode::UNAUTHORIZED;
        message = "UNAUTHORIZED".to_owned();
    } else if let Some(_) = err.find::<LoginError>() {
        code = StatusCode::BAD_REQUEST;
        message = "BAD_REQUEST".to_owned();
    } else if let Some(_) = err.find::<DataBaseError>() {
        code = StatusCode::BAD_REQUEST;
        message = "BAD_REQUEST".to_owned();
    } else if let Some(_) = err.find::<InternalDataBaseError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "INTERNAL_SERVER_ERROR: DATABASE".to_owned();
    } else if let Some(_) = err.find::<InvalidUserDataFormat>() {
        code = StatusCode::BAD_REQUEST;
        message = "INVALID_USER_DATA_FORMAT".to_owned();
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = format!("UNHANDLED_REJECTION(IT'S A BUG): {:?}", err);
    }

    let error = ErrorModel {
        message,
        code: code.as_u16(),
    };

    Ok(warp::reply::with_status(error, code))
}
