use aes_gcm::{Aes256Gcm, Key as AesKey};
use bytes::Bytes;
use http_body_util::Full;
use hyper::{body::Incoming, service::Service, Method, Request, Response};
use min_auth_common::{config::admin::AdminConfig, data::users::User, DynError};
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AdminService {
    pub config: Arc<RwLock<AdminConfig>>,
    pub users: Arc<RwLock<HashMap<String, User>>>,
    pub session_key: Arc<RwLock<AesKey<Aes256Gcm>>>,
}

impl Service<Request<Incoming>> for AdminService {
    type Response = Response<Full<Bytes>>;
    type Error = DynError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let config = Arc::clone(&self.config);

        Box::pin(async move {
            let method = req.method();
            let path = req.uri().path();
            match (method, path) {
                (&Method::GET, "/users") => get_users(req, &tera, &table).await,
                (&Method::POST, "/createuser") => create_user(req, &table).await,
                (&Method::POST, "/updateuser") => update_user(req, &tera, &table).await,
                (&Method::POST, "/deleteuser") => delete_user(req, &table).await,
                (method, path) => Err(format!("Illegal request ({} {})", method, path).into()),
            }
        })
    }
}
