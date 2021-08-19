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

                let mut user = User::new(req.login.clone(), hash);
                user.avatar = Some(AVATAR.to_string());
                match context.rbdb.save_obj(&user, &[]).await {
                    Err(_) => Err(InvalidRegistrationDataError::rej()),
                    Ok(_) => login(&req.login, &req.password, context.rbdb.clone()).await,
                }
            },
        );

    let user_filter = warp::path("user");

    let get_user_route = warp::get()
        .and(warp::path::end().and(auth).or(warp::path::param::<u32>()).unify())
        .and(with_context)
        .and_then(move |id: u32, context: Context| async move {
            get_user(&id, context.rbdb.clone()).await
        });

    let get_user_avatar_route = warp::path("avatar")
        .and(warp::get())
        .and(warp::path::param::<u32>().or(auth).unify())
        .and(with_context)
        .and_then(move |id: u32, context: Context| async move {
            get_avatar(&id, context.rbdb.clone()).await
        });

    let change_user_avatar_route = warp::path("avatar")
        .and(warp::post())
        .and(auth)
        .and(query_as!(ChangeAvatarRequest))
        .and(with_context)
        .and_then(
            move |id: u32, req: ChangeAvatarRequest, context: Context| async move {
                let mut obj = User::default();
                obj.id = Some(id);
                obj.avatar = Some(req.avatar);
                match context.rbdb.update_obj(&mut obj).await {
                    Err(_) => Err(InternalDataBaseError::rej()),
                    Ok(_) => Ok(warp::reply()),
                }
            },
        );

    let change_user_about_route = warp::path("about")
        .and(warp::post())
        .and(auth)
        .and(query_as!(ChangeAboutRequest))
        .and(with_context)
        .and_then(
            move |id: u32, req: ChangeAboutRequest, context: Context| async move {
                let mut obj = User::default();
                obj.id = Some(id);
                obj.about_me = Some(req.about);
                match context.rbdb.update_obj(&mut obj).await {
                    Err(_) => Err(InternalDataBaseError::rej()),
                    Ok(_) => Ok(warp::reply()),
                }
            },
        );

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

    //TODO: Simplify user_routes: (A && B) || A || C
    let user_routes = user_filter.and(
        get_user_route
            .or(get_user_avatar_route)
            .or(change_user_avatar_route)
            .or(change_user_about_route),
    );

    let auth_route = reg_route.or(login_route);
    let debug_route = delete_route;

    let native_route = auth_route.or(user_routes);

    #[cfg(debug_assertions)]
    return api_filter.and(native_route.or(debug_route)).or(ws_route);
    #[cfg(not(debug_assertions))]
    api_filter.and(native_route).or(ws_route)
}
