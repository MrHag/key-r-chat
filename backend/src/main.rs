use lib::context::CONTEXT;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rbatis;

use crate::lib::routes::routes::make_route;
mod lib;

#[tokio::main]
async fn main() {
    start().await;
}

async fn start() {
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();

    CONTEXT
        .rbdb
        .rb
        .link("mysql://lostback:U1234@localhost:3306/lostback")
        .await
        .unwrap();

    let route = make_route().await;

    let port = 8080;
    println!("starting server listening on ::{}", port);
    tokio::task::spawn(warp::serve(route).run(([0, 0, 0, 0], port)));
}
