use lib::context::CONTEXT;
use warp::Filter;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rbatis;

use crate::lib::route::{functions::handle_rejection, routes::make_route};
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
    dotenv::dotenv().expect(".env file not found");
    link_db().await;

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["authorization"])
        // .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
        .allow_methods(vec!["GET", "POST", "DELETE"]);

    let route = warp::any()
        .and(make_route().await)
        .recover(handle_rejection)
        .with(cors);

    let port = 8080;
    println!("starting server listening on ::{}", port);
    warp::serve(route).run(([0, 0, 0, 0], port)).await
}
