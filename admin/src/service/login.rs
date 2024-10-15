use aes_gcm::{Aes256Gcm, Key as AesKey};
use bytes::Bytes;
use http_body_util::Full;
use hyper::{body::Incoming, Request, Response, StatusCode};
use log::error;
use min_auth_common::{config::admin::AdminConfig, DynError};
use redis::Client as RedisClient;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub(crate) async fn login(
    req: Request<Incoming>,
    redis: &Arc<Mutex<RedisClient>>,
    session_key: &Arc<RwLock<AesKey<Aes256Gcm>>>,
    config: &Arc<RwLock<AdminConfig>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    match login_body(req, redis, session_key, config).await {
        Ok(res) => Ok(res),
        Err(e) => {
            error!("{}", e);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("".to_string().into_bytes().into())?)
        }
    }
}

async fn login_body(
    req: Request<Incoming>,
    redis: &Arc<Mutex<RedisClient>>,
    session_key: &Arc<RwLock<AesKey<Aes256Gcm>>>,
    config: &Arc<RwLock<AdminConfig>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    todo!()
}
