use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use hmac::crypto_mac::InvalidKeyLength;
use hmac::{Hmac, Mac, NewMac};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rbatis::core::db::DBExecResult;
use serde_derive::{Deserialize, Serialize};
use sha2::{Sha256};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock};

use warp::ws::{Message, WebSocket};
use warp::{Filter, Rejection, Reply};

#[macro_use]
extern crate rbatis;

#[macro_use]
extern crate lazy_static;

use chrono::*;
use rbatis::crud::*;
use rbatis::rbatis::*;
use rbatis::Error;

#[crud_table(table_name:users)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct User {
    pub id: Option<u32>,
    pub login: Option<String>,
    pub password_hash: Option<String>,
}

#[crud_table(table_name:tokens)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Token {
    id: Option<u32>,
    token: Option<String>,
    exp_date: Option<NaiveDateTime>,
    user_id: Option<u32>,
}

impl User {
    fn new(login: String, password_hash: String) -> User {
        User {
            id: None,
            login: Some(login),
            password_hash: Some(password_hash),
        }
    }
}

trait Validation {
    fn validate(&self) -> Result<(), &str>;
}

#[derive(Serialize)]
struct LoginReply {
    pub token: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    pub login: String,
    pub password: String,
}

impl Validation for LoginRequest {
    fn validate(&self) -> Result<(), &str> {
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
struct RegistrationRequest {
    pub login: String,
    pub password: String,
}

impl Validation for RegistrationRequest {
    fn validate(&self) -> Result<(), &str> {
        if !(3..=32).contains(&self.login.len()) {
            return Err("Login length can be from 3 to 32");
        }
        if !self.login.chars().all(char::is_alphanumeric) {
            return Err("Login is not aplhanumeric");
        }

        if !(3..=32).contains(&self.password.len()) {
            return Err("Password length can be from 3 to 32");
        }
        if !self.password.chars().all(char::is_alphanumeric) {
            return Err("Password is not aplhanumeric");
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct WsRoute {
    header: String,
    body: String,
}


#[derive(Debug)]
struct ErrorResp {
    pub error: String,
}

impl warp::reject::Reject for ErrorResp {}

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
    static ref SC: &'static [u8] = b"jfhkfljfjdj";
    static ref USERS: Users = Users::default();
}

#[tokio::main]
async fn main() {
    start().await;
}

type Users = Arc<RwLock<HashMap<u32, SplitSink<WebSocket, Message>>>>;

async fn make_route() -> impl Filter<Extract = (impl warp::reply::Reply,), Error = warp::Rejection> + Copy
{
    let authorize = warp::any()
        .and(warp::header::<String>("authorization"))
        //.and(warp::cookie::<String>("authorization"))
        .and_then(move |token: String| async move { authorizefn(&token).await });

    async fn authorizefn(token: &String) -> Result<User, Rejection> {
        match get_token(&token).await {
            Err(_err) => Err(warp::reject::custom(ErrorResp {
                error: "Auth Error".to_owned(),
            })),
            Ok(token) => {
                delete_expired_tokens_by_userid(&token.user_id.unwrap())
                    .await
                    .unwrap();
                match token.exp_date.unwrap() >= Utc::now().naive_utc() {
                    false => Err(warp::reject::custom(ErrorResp {
                        error: "Auth Error".to_owned(),
                    })),
                    true => match user_by_id(&token.user_id.unwrap()).await {
                        Err(_err) => Err(warp::reject::custom(ErrorResp {
                            error: "DataBase Error".to_owned(),
                        })),
                        Ok(user) => Ok(user),
                    },
                }
            }
        }
    }

    async fn login(login: &String, password: &String) -> Result<impl Reply, Rejection> {
        let user = match user_by_login(&login).await {
            Err(_) => {
                return Err(warp::reject::custom(ErrorResp {
                    error: "Invalid Login or Password".to_owned(),
                }))
            }
            Ok(user) => user,
        };

        let phash = hash_string(&password, &SC).unwrap();

        if dbg!(phash) != dbg!(user.password_hash.unwrap()) {
            return Err(warp::reject::custom(ErrorResp {
                error: "Invalid Login or Password".to_owned(),
            }));
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

        match SaveObj(&token, &[]).await {
            Err(err) => Err(warp::reject::custom(ErrorResp { error: err })),
            Ok(_) => Ok(warp::reply::json(&LoginReply { token: token_str })),
        }
    }

    let login_route = warp::post()
        .and(warp::path("login"))
        .and(query_as!(LoginRequest))
        .and_then(move |req: LoginRequest| async move { login(&req.login, &req.password).await });

    let reg_route = warp::post()
        .and(warp::path("registration"))
        .and(query_as!(RegistrationRequest))
        .and_then(move |req: RegistrationRequest| async move {
            let hash = hash_string(&req.password, &SC).unwrap();

            let user = User::new(req.login.clone(), hash);

            match SaveObj(&user, &[]).await {
                Err(err) => Err(warp::reject::custom(ErrorResp { error: err })),
                Ok(_) => login(&req.login, &req.password).await,
            }
        });

    let handle_ws = move |ws: WebSocket, user: User, usrs: Users| async move {
        let (sender, mut receiver) = ws.split();
        usrs.write().await.insert(user.id.unwrap(), sender);

        while let Some(result) = receiver.next().await {
            match result {
                Err(_e) => {
                    eprintln!("error receiving ws message");
                    break;
                }
                Ok(msg) => match msg.is_ping() {
                    false => {
                        eprintln!("error receiving ws message");
                        break;
                    }
                    true => match usrs
                        .write()
                        .await
                        .get_mut(&user.id.unwrap())
                        .unwrap()
                        .send(Message::pong([]))
                        .await
                    {
                        Err(_) => {
                            eprintln!("error send pong");
                            break;
                        }
                        Ok(_) => (),
                    },
                },
            };
        }

        usrs.write().await.remove(&user.id.unwrap());
        println!("User disconnected");
    };

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(authorize)
        .and(warp::any().map(move || USERS.clone()))
        .and_then(move |ws: warp::ws::Ws, user: User, usrs| async move {
            println!("gg");
            return Ok(ws.on_upgrade(move |g| async move { handle_ws(g, user, usrs).await }));
            Err(warp::reject::custom(ErrorResp { error: "err".to_owned() }))
        });

    // let ws_route =
    //     warp::path("ws")
    //     .and(warp::ws())
    //     .and(authorize)
    //     .and_then(move |ws, us| async move{
    //         println!("gg");
    //         return Ok(warp::reply::json(&"".to_owned()));
    //         Err(warp::reject::custom(ErrorResp { error: "".to_owned() }))
    //     });

    // let some_route = warp::get()
    //     .and(warp::path("some"))
    //     .and(authorize)
    //     .and(query_as!(SomeRequest))
    //     .and_then(move |user: User, req: SomeRequest| async move {
    //         Ok(warp::reply::json(&SomeReply {
    //             data: "SomeData".to_owned(),
    //         }))
    //     });

    //let result: Option<User> = RB.fetch_by_column("id", &"1".to_string()).await.unwrap();
    // Start the server
    let route = reg_route.or(login_route).or(ws_route);
    route
}

async fn SaveObj<T>(obj: &T, skips: &[Skip<'_>]) -> Result<DBExecResult, String>
where
    T: CRUDTable,
{
    let db_result = RB.save(obj, skips).await;

    match db_result {
        Err(err) => Err(match err {
            Error::E(err) => err,
            Error::Deserialize(err) => err,
            Error::Database(err) => err,
            _ => "Uknown error".to_owned(),
        }),
        Ok(res) => Ok(res),
    }
}

async fn start() {
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();

    RB.link("mysql://lostback:U1234@localhost:3306/lostback")
        .await
        .unwrap();

    let route = make_route().await;

    let port = 8080;
    println!("starting server listening on ::{}", port);
    tokio::task::spawn(warp::serve(route).run(([0, 0, 0, 0], port)));

}

fn hash_string(str: &str, secret_key: &[u8]) -> Result<String, InvalidKeyLength> {
    let mac = Hmac::<Sha256>::new_from_slice(secret_key);
    match mac {
        Err(err) => Err(err),
        Ok(mut hash) => {
            hash.update(str.as_bytes());
            Ok(hex::encode(&hash.finalize().into_bytes()))
        }
    }
}

async fn get_token(token: &String) -> Result<Token, rbatis::Error> {
    RB.fetch_by_column::<Token, String>("token", token).await
}

async fn user_by_id(id: &u32) -> Result<User, rbatis::Error> {
    RB.fetch_by_column::<User, u32>("id", id).await
}

async fn user_by_login(login: &String) -> Result<User, rbatis::Error> {
    RB.fetch_by_column::<User, String>("login", login).await
}

async fn delete_expired_tokens_by_userid(id: &u32) -> Result<u64, rbatis::Error> {
    let wrapper = RB
        .new_wrapper()
        .eq("user_id", *id)
        .lt("exp_date", Utc::now().naive_utc());
    RB.remove_by_wrapper::<Token>(&wrapper).await
}

fn try_validate<T: Validation>(to_validate: T) -> Result<T, Rejection> {
    let result = to_validate.validate();
    match result {
        Ok(()) => Ok(to_validate),
        Err(err) => Err(warp::reject::custom(ErrorResp {
            error: err.to_owned(),
        })),
    }
}

#[macro_export]
macro_rules! query_as {
    ($x:ty) => {
        warp::any()
            .and(query!($x))
            .and_then(move |m: Option<$x>| async move {
                match m {
                    None => Err(warp::reject::custom(ErrorResp {
                        error: "Not valid request".to_owned(),
                    })),
                    Some(some) => try_validate(some),
                }
            })
    };
}

#[macro_export]
macro_rules! query {
    ($x:ty) => {
        warp::query::<$x>()
            .map(Some)
            .or_else(|_| async { Ok::<(Option<$x>,), std::convert::Infallible>((None,)) });
    };
}

#[tokio::test]
async fn start_test() {
    println!("0");
    start().await;
    println!("1");
    //assert!(!extract(some_query().await));
    //println!("2");
    //assert!(extract(some_query2().await));
    //println!("3");
    //assert!(extract(some_query3().await));
    some_query4().await;
}
async fn some_query() -> Result<impl warp::reply::Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request()
        .method("POST")
        .path("/registration?login=1&password=1");

    req.filter(&route).await
}

async fn some_query2() -> Result<impl warp::reply::Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request()
        .method("POST")
        .path("/registration?login=vasya&password=notgay");

    req.filter(&route).await
}

async fn some_query3() -> Result<impl warp::reply::Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request()
        .method("POST")
        .path("/login?login=vasya&password=notgay");

    req.filter(&route).await
}

async fn some_query4() -> () {
    let route = make_route().await;

    let req = warp::test::ws().path("/ws").header("authorization", "P07qSAEwYzKAwWAJNKzaPYapynm84ttptPpqgRukAd3k3HqwZXIrcXyHHj9DcJMc");

    let _connection = req.handshake(route).await.unwrap();
    //let gg = connection.recv().await.unwrap();

}

fn extract<T: std::fmt::Debug>(obj: Result<impl warp::reply::Reply, T>) -> bool {
    let is_ok = obj.is_ok();
    println!(
        "{}",
        match obj {
            Ok(reply) => format!("{:?}", reply.into_response()),
            Err(err) => format!("{:?}", err),
        }
    );
    is_ok
}

#[macro_export]
macro_rules! extract {
    ($e:tt) => {
        (|| async move {
            let is_ok = $e.is_ok();
            println!(
                "{}",
                match $e {
                    Ok(json) => format!("{:?}", json.into_response()),
                    Err(err) => format!("{:?}", err),
                }
            );
            is_ok
        })()
        .await
    };
}
