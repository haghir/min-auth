use aes_gcm::{Aes256Gcm, Key as AesKey};
use bytes::Bytes;
use http_body_util::Full;
use hyper::{body::Incoming, Method, Request, Response};
use min_auth_common::{config::admin::AdminConfig, data::users::User, error::Error, DynError};
use redis::Client as RedisClient;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
use tokio::sync::{Mutex, RwLock};

mod login;
mod update;
mod users;

#[derive(Debug, Clone)]
pub struct Service {
    pub config: Arc<RwLock<AdminConfig>>,
    pub redis: Arc<Mutex<RedisClient>>,
    pub session_key: Arc<RwLock<AesKey<Aes256Gcm>>>,
}

impl hyper::service::Service<Request<Incoming>> for Service {
    type Response = Response<Full<Bytes>>;
    type Error = DynError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let config = Arc::clone(&self.config);
        let redis = Arc::clone(&self.redis);
        let session_key = Arc::clone(&self.session_key);

        Box::pin(async move {
            let method = req.method();
            let path = req.uri().path();
            match (method, path) {
                (&Method::POST, "/login") => login::login(req, &redis, &session_key, &config).await,
                (&Method::GET, "/users") => {
                    users::get_users(req, &redis, &session_key, &config).await
                }
                (&Method::POST, "/update") => {
                    update::update(req, &redis, &session_key, &config).await
                }
                (method, path) => {
                    Err(Error::new(format!("Illegal request ({} {})", method, path)).into())
                }
            }
        })
    }
}
