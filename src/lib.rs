mod api;
mod utils;

use worker::*;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in
    // the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/api/v1/members", |mut req, ctx| async move {
            api::v1::members::find_all(req, ctx).await
        })
        .run(req, env)
        .await
}
