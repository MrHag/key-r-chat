#[macro_export]
macro_rules! query_as {
    ($x:ty) => {
        warp::any()
            .and(
                warp::query::<$x>()
                    .map(Some)
                    .or_else(|_| async { Ok::<(Option<$x>,), std::convert::Infallible>((None,)) }),
            )
            .and_then(move |m: Option<$x>| async move {
                match m {
                    None => Err(InvalidRequest::rej()),
                    Some(some) => try_validate(some),
                }
            })
    };
}
