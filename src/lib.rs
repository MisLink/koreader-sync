use worker::*;
mod api;
mod db;
mod types;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .post_async("/users/create", api::create_user)
        .get_async("/users/auth", api::auth_user)
        .put_async("/syncs/progress", api::update_progress)
        .get_async("/syncs/progress/:document", api::get_progress)
        .get_async("/healthcheck", api::health_check)
        .run(req, env)
        .await
}
