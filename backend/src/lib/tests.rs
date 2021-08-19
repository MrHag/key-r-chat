use futures::{Future, TryStreamExt};
use serde::de::DeserializeOwned;
use warp::{Filter, Rejection, Reply};

use crate::extract;
use crate::lib::replies::entities::*;
use crate::link_db;

use super::route::routes::make_route;

struct TestPrinter(u32);
impl TestPrinter {
    fn next(&mut self) {
        println!("test №{}", self.0);
        self.0 += 1;
    }
}

#[tokio::test]
async fn start_test() {
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();
    link_db().await;

    let mut p = TestPrinter(0);

    p.next();
    assert!(extract(db_drop_query().await));
    p.next();
    assert!(extract(error_registration_query().await));
    p.next();
    assert!(!extract(error_registration_query().await));
    p.next();
    assert!(extract(registration_query().await));
    p.next();
    let token = extract!(LoginReply, login_query()).await.unwrap().token;
    p.next();
    assert!(extract(get_user_query(&token).await));
    p.next();
    assert!(extract(get_user_by_id_query(1).await));

    ws_auth_query(&token).await;
}

async fn db_drop_query() -> Result<impl Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request().method("GET").path("/api/drop_db");

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

async fn get_user_query(token: &str) -> Result<impl Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request()
        .method("GET")
        .header("authorization", token)
        .path("/api/user");

    req.filter(&route).await
}

async fn get_user_by_id_query(id: u32) -> Result<impl Reply, Rejection> {
    let route = make_route().await;

    let req = warp::test::request()
        .method("GET")
        .path(&format!("/api/user/{}", id));

    req.filter(&route).await
}
async fn ws_auth_query(token: &str) {
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
