use futures::{SinkExt, StreamExt};
use rbatis::executor::Executor;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use crate::lib::context::*;
use crate::lib::database::entities::*;
use crate::lib::errors::*;
use crate::lib::requests::entities::*;
use crate::lib::routes::functions::*;
use crate::query_as;
use rbatis::crud::*;

pub async fn make_route(
) -> impl Filter<Extract = (impl warp::reply::Reply,), Error = warp::Rejection> + Copy {
    let with_context = warp::any().map(move || CONTEXT.clone());

    let authorize = warp::any()
        .and(warp::header::<String>("authorization"))
        //.and(warp::cookie::<String>("authorization"))
        .and(with_context)
        .and_then(move |token: String, context: Context| async move {
            authorizefn(&token, context.rbdb.clone()).await
        });

    let login_route = warp::post()
        .and(warp::path("login"))
        .and(query_as!(LoginRequest))
        .and(with_context)
        .and_then(move |req: LoginRequest, context: Context| async move {
            login(&req.login, &req.password, context.rbdb.clone()).await
        });

    let reg_route = warp::post()
        .and(warp::path("registration"))
        .and(query_as!(RegistrationRequest))
        .and(with_context)
        .and_then(
            move |req: RegistrationRequest, context: Context| async move {
                let hash = hash_string(&req.password, &SC).unwrap();

                let user = User::new(req.login.clone(), hash);

                match context.rbdb.SaveObj(&user, &[]).await {
                    Err(_err) => Err(DataBaseError::rej()),
                    Ok(_) => login(&req.login, &req.password, context.rbdb.clone()).await,
                }
            },
        );

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
        .and(with_context)
        .and_then(
            move |ws: warp::ws::Ws, user: User, context: Context| async move {
                return Ok(ws.on_upgrade(move |g| async move {
                    handle_ws(g, user, context.users.clone()).await
                }));
                Err(InvalidRequest::rej())
            },
        );

    #[cfg(debug_assertions)]
    let delete_route = warp::path("drop_db")
        .and(warp::delete())
        .and(with_context)
        .and_then(move |context: Context| async move {
            match context
                .rbdb
                .rb
                .remove_by_wrapper::<User>(&context.rbdb.rb.new_wrapper())
                .await
            {
                Err(_err) => Err(DataBaseError::rej()),
                Ok(res) => match context
                    .rbdb
                    .rb
                    .exec("ALTER TABLE users AUTO_INCREMENT = 1; ALTER TABLE tokens AUTO_INCREMENT = 1", &vec![]).await
                {
                    Ok(_) => Ok(warp::reply::json(&res)),
                    Err(_) => Err(DataBaseError::rej()),
                },
            }
        });
    // let some_route = warp::get()
    //     .and(warp::path("some"))
    //     .and(authorize)
    //     .and(query_as!(SomeRequest))
    //     .and_then(move |user: User, req: SomeRequest| async move {
    //         Ok(warp::reply::json(&SomeReply {
    //             data: "SomeData".to_owned(),
    //         }))
    //     });
    let route = reg_route.or(login_route).or(ws_route);
    #[cfg(debug_assertions)]
    return route.or(delete_route);
    #[cfg(not(debug_assertions))]
    route
}
