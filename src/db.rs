use worker::kv::KvStore;
use crate::types::{AppError, ProgressState, User};
use worker::{Result as WorkerResult};


fn user_key(user: &str) -> String {
    format!("user:{}:key", user)
}

fn doc_key(user: &str, doc: &str) -> String {
    format!("user:{}:document:{}", user, doc)
}

pub fn get_kv<D>(ctx: &worker::RouteContext<D>) -> WorkerResult<KvStore> {
    ctx.kv("KV_BINDING")
}


pub async fn create_user(kv: &KvStore, user: &str, key: &str) -> Result<(), AppError> {
    if kv.get(&user_key(user)).text().await?.is_some() {
        return Err(AppError::UsernameAlreadyRegistered);
    }
    kv.put(&user_key(user), key)?.execute().await?;
    Ok(())
}

pub async fn get_user(kv: &KvStore, username: &str) -> Result<User, AppError> {
    match kv.get(&user_key(username)).text().await {
        Ok(Some(key)) => Ok(User {
            name: username.to_string(),
            key,
        }),
        Ok(None) => Err(AppError::Unauthorized),
        Err(_) => Err(AppError::StoreError),
    }
}

pub async fn update_progress(
    kv: &KvStore,
    user: &str,
    progress: &ProgressState,
) -> Result<(), AppError> {
    kv.put(&doc_key(user, &progress.document), progress)?.execute().await?;
    Ok(())
}

pub async fn get_progress(
    kv: &KvStore,
    user: &str,
    document: &str,
) -> Result<Option<ProgressState>, AppError> {
    match kv.get(&doc_key(user, document)).json::<ProgressState>().await {
        Ok(Some(progress)) => Ok(Some(progress)),
        Ok(None) => Ok(None),
        Err(_) => Err(AppError::StoreError),
    }
}
