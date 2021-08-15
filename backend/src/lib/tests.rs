use futures::{Future, TryStreamExt};
use serde::de::DeserializeOwned;
use warp::{Rejection, Reply};

use crate::extract;
use crate::lib::replyes::entities::*;
use crate::link_db;

use super::routes::routes::make_route;

#[tokio::test]
async fn start_test() {
    link_db().await;
    println!("0");
    assert!(extract(db_drop_query().await));
    println!("1");
    assert!(!extract(error_registration_query().await));
    println!("2");
    assert!(extract(registration_query().await));
    println!("3");
    let reply = extract!(LoginReply, login_query()).await.unwrap();

    ws_auth_query(&reply.token).await;
}

async fn db_drop_query() -> Result<impl Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request().method("DELETE").path("/api/drop_db");

    req.filter(&route).await
}

async fn error_registration_query() -> Result<impl Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request()
        .method("POST")
        .path("/api/registration?login=1&password=1");

    req.filter(&route).await
}

async fn registration_query() -> Result<impl Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request()
        .method("POST")
        .path("/api/registration?login=vasya&password=notgay");

    req.filter(&route).await
}

async fn login_query() -> Result<impl Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request()
        .method("POST")
        .path("/api/login?login=vasya&password=notgay");

    req.filter(&route).await
}

async fn ws_auth_query(token: &str) -> () {
    let route = make_route().await;

    let req = warp::test::ws().path("/ws").header("authorization", token);

    let _connection = req.handshake(route).await.unwrap();
    //let gg = connection.recv().await.unwrap();
}

fn extract<T: std::fmt::Debug>(obj: Result<impl Reply, T>) -> bool {
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

async fn extract_value<N: DeserializeOwned>(vec: Result<Vec<u8>, String>) -> Result<N, String> {
    match serde_json::from_slice(&vec?) {
        Err(err) => Err(format!("{:?}", err)),
        Ok(des_obj) => Ok(des_obj),
    }
}

async fn extract_slice<T: std::fmt::Debug, G: Reply>(
    obj: impl Future<Output = Result<G, T>>,
) -> Result<Vec<u8>, String> {
    match obj.await {
        Err(err) => Err(format!("{:?}", err)),
        Ok(reply) => match reply
            .into_response()
            .into_body()
            .try_fold(Vec::new(), |mut data, chunk| async move {
                data.extend_from_slice(&chunk);
                Ok(data)
            })
            .await
        {
            Err(err) => Err(format!("{:?}", err)),
            Ok(vec) => Ok(vec),
        },
    }
}

#[macro_export]
macro_rules! extract {
    ($y:tt, $x:expr) => {
        extract_value::<$y>(extract_slice($x).await)
    };
}
