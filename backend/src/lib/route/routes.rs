use futures::{SinkExt, StreamExt};
use rbatis::executor::Executor;
use warp::ws::{Message, WebSocket};
use warp::{Filter, Rejection, Reply};

use crate::lib::context::*;
use crate::lib::database::entities::*;
use crate::lib::error::errors::*;
use crate::lib::requests::entities::*;
use crate::lib::route::functions::*;
use crate::query_as;
use rbatis::crud::*;

pub async fn make_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let with_context = warp::any().map(move || CONTEXT.clone());

    let auth = warp::any()
        .and(warp::header::<String>("authorization"))
        //.and(warp::cookie::<String>("authorization"))
        .and(with_context)
        .and_then(move |token: String, context: Context| async move {
            authorize(&token, context.rbdb.clone()).await
        });

    let login_route = warp::path("login")
        .and(warp::post())
        .and(query_as!(LoginRequest))
        .and(with_context)
        .and_then(move |req: LoginRequest, context: Context| async move {
            login(&req.login, &req.password, context.rbdb.clone()).await
        });

    let reg_route = warp::path("registration")
        .and(warp::post())
        .and(query_as!(RegistrationRequest))
        .and(with_context)
        .and_then(
            move |req: RegistrationRequest, context: Context| async move {
                let hash = hash_string(&req.password, &SC).unwrap();

                let user = User::new(req.login.clone(), hash);

                match context.rbdb.save_obj(&user, &[]).await {
                    Err(_) => Err(InvalidRegistrationDataError::rej()),
                    Ok(_) => login(&req.login, &req.password, context.rbdb.clone()).await,
                }
            },
        );

    let get_user_route = warp::path!("user" / u32)
        .and(warp::get())
        .and(auth)
        .and(with_context)
        .and_then(move |id: u32, _user_id: u32, context: Context| async move {
            get_user(&id, context.rbdb.clone()).await
        });

    let get_myself_user_route = warp::path("user")
        .and(warp::get())
        .and(auth)
        .and(with_context)
        .and_then(move |user_id: u32, context: Context| async move {
            get_user(&user_id, context.rbdb.clone()).await
        });

    let handle_ws = move |ws: WebSocket, user_id: u32, usrs: Users| async move {
        let (sender, mut receiver) = ws.split();
        usrs.write().await.insert(user_id, sender);

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
                    true => {
                        if usrs
                            .write()
                            .await
                            .get_mut(&user_id)
                            .unwrap()
                            .send(Message::pong([]))
                            .await
                            .is_err()
                        {
                            eprintln!("error send pong");
                            break;
                        }
                    }
                },
            };
        }

        usrs.write().await.remove(&user_id);
        println!("User disconnected");
    };

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(auth)
        .and(with_context)
        .and_then(
            move |ws: warp::ws::Ws, user_id: u32, context: Context| async move {
                return Ok(ws.on_upgrade(move |ws| async move {
                    handle_ws(ws, user_id, context.users.clone()).await
                }));
                Err(InvalidRequest::rej())
            },
        );

    #[cfg(debug_assertions)]
    let delete_route = warp::path("drop_db")
        .and(warp::get())
        .and(with_context)
        .and_then(move |context: Context| async move {
            match context
                .rbdb
                .rb
                .remove_by_wrapper::<User>(&context.rbdb.rb.new_wrapper())
                .await
            {
                Err(_err) => Err(InternalDataBaseError::rej()),
                Ok(res) => match context
                    .rbdb
                    .rb
                    .exec("ALTER TABLE users AUTO_INCREMENT = 1; ALTER TABLE tokens AUTO_INCREMENT = 1", &vec![]).await
                {
                    Ok(_) => Ok(warp::reply::json(&res)),
                    Err(_) => Err(InternalDataBaseError::rej()),
                },
            }
        });

    let api_filter = warp::path("api");

    // let some_route = warp::get()
    //     .and(warp::path("some"))
    //     .and(authorize)
    //     .and(query_as!(SomeRequest))
    //     .and_then(move |user: User, req: SomeRequest| async move {
    //         Ok(warp::reply::json(&SomeReply {
    //             data: "SomeData".to_owned(),
    //         }))
    //     });
    let native_route = reg_route
        .or(login_route)
        .or(get_user_route)
        .or(get_myself_user_route);
    #[cfg(debug_assertions)]
    return api_filter.and(native_route.or(delete_route)).or(ws_route);
    #[cfg(not(debug_assertions))]
    api_filter.and(native_route).or(ws_route)
}