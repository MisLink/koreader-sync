use serde_json::json;
use worker::kv::KvStore;
use worker::{Request, Response,Result as WorkerResult, RouteContext};
use serde::Deserialize;
use time::OffsetDateTime;
use crate::db;
use crate::types::{AppError, ProgressState};

fn is_valid_key_field(s: &str) -> bool {
    !s.is_empty() && !s.contains(':')
}


#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
}

pub async fn create_user(mut req: Request, ctx: RouteContext<()>) -> WorkerResult<Response> {
    let request_data: CreateUserRequest = match req.json().await {
        Ok(data) => data,
        Err(_) => return AppError::InvalidRequest.into(),
    };

    if !is_valid_key_field(&request_data.username) || request_data.password.is_empty() {
        return AppError::InvalidRequest.into();
    }

    let kv = db::get_kv(&ctx)?;

    if let Err(err) = db::create_user(&kv, &request_data.username, &request_data.password).await {
        return err.into();
    }

    Response::from_json(&json!({
        "username": request_data.username
    })).map(|res| res.with_status(http::StatusCode::CREATED.as_u16()))
}

async fn authorize(kv: &KvStore, req: &Request) -> Result<String, AppError> {
    let username = req.headers().get("x-auth-user")
        .map_err(|_| AppError::Unauthorized)?
        .ok_or(AppError::Unauthorized)?;
    let password = req.headers().get("x-auth-key")
        .map_err(|_| AppError::Unauthorized)?
        .ok_or(AppError::Unauthorized)?;

    if !is_valid_key_field(&username) {
        return Err(AppError::Unauthorized);
    }

    let user = db::get_user(kv, &username).await?;

    if user.key != password {
        return Err(AppError::Unauthorized);
    }

    Ok(user.name)
}

pub async fn auth_user(req: Request, ctx: RouteContext<()>) -> WorkerResult<Response> {
    let kv = db::get_kv(&ctx)?;
    match authorize(&kv, &req).await {
        Ok(_) => Response::from_json(&json!({
            "authorized": "OK"
        })),
        Err(err) => err.into(),
    }
}


#[derive(Deserialize)]
struct UpdateProgressRequest {
    document: String,
    percentage: f32,
    progress: String,
    device: String,
    device_id: Option<String>,
}

impl From<UpdateProgressRequest> for ProgressState {
    fn from(req: UpdateProgressRequest) -> Self {
        ProgressState {
            document: req.document,
            percentage: req.percentage,
            progress: req.progress,
            device: req.device,
            device_id: req.device_id.unwrap_or_default(),
            timestamp: now_timestamp(),
        }
    }
}

fn now_timestamp() -> u64 {
    OffsetDateTime::now_utc().unix_timestamp() as u64
}

pub async fn update_progress(mut req: Request, ctx: RouteContext<()>) -> WorkerResult<Response> {
    let kv = db::get_kv(&ctx)?;
    let username = match authorize(&kv, &req).await {
        Ok(username) => username,
        Err(err) => return err.into(),
    };
    let request_data: UpdateProgressRequest = match req.json().await {
        Ok(data) => data,
        Err(_) => return AppError::InvalidRequest.into(),
    };

    if !is_valid_key_field(&request_data.document) {
        return AppError::DocumentNotProvided.into();
    }

    let progress_state: ProgressState = request_data.into();

    if let Err(err) = db::update_progress(&kv, &username, &progress_state).await {
        return err.into();
    }

    Response::from_json(&json!({
        "document": progress_state.document,
        "timestamp": progress_state.timestamp,
    }))
}

pub async fn get_progress(req: Request, ctx: RouteContext<()>) -> WorkerResult<Response> {
    let kv = db::get_kv(&ctx)?;
    let username = match authorize(&kv, &req).await {
        Ok(username) => username,
        Err(err) => return err.into(),
    };

    let document = match ctx.param("document") {
        Some(doc) => doc,
        None => return AppError::DocumentNotProvided.into(),
    };
    if !is_valid_key_field(document) {
        return AppError::DocumentNotProvided.into();
    }
    match db::get_progress(&kv, &username, document).await {
        Ok(Some(progress)) => Response::from_json(&progress),
        Ok(None) => Response::from_json(&json!({ "document": document })),
        Err(err) => err.into(),
    }
}

pub async fn health_check(_req: Request, _ctx: RouteContext<()>) -> WorkerResult<Response> {
    Response::from_json(&json!({ "status": "OK" }))
}
