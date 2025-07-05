use tracing_web::{performance_layer, MakeConsoleWriter};
use tracing_subscriber::{fmt::{format::Pretty, time::UtcTime}, prelude::*};
use worker::*;
mod api;
mod db;
mod types;

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(MakeConsoleWriter);
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

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
