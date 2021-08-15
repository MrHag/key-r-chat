use lib::context::CONTEXT;
use warp::Filter;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rbatis;

use crate::lib::routes::{functions::handle_rejection, routes::make_route};
mod lib;

#[tokio::main]
async fn main() {
    start().await;
}

async fn link_db() {
    CONTEXT
        .rbdb
        .rb
        .link("mysql://lostback:U1234@localhost:3306/lostback")
        .await
        .unwrap();
}

async fn start() {
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();

    link_db().await;

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE"]);
    let route = warp::any()
        .and(make_route().await)
        .with(cors)
        .recover(handle_rejection);

    let port = 8080;
    println!("starting server listening on ::{}", port);
    warp::serve(route).run(([0, 0, 0, 0], port)).await
}
