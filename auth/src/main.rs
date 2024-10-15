use bytes::Bytes;
use getopts::Options;
use http_auth_basic::Credentials;
use http_body_util::Full;
use hyper::{
    body::Incoming, header, server::conn::http1, service::Service as HyperService, Method, Request,
    Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use log::error;
use min_auth_common::{
    config::auth::AuthConfig, data::credentials::Credentials as CredData, error::Error, DynError,
};
use redis::{AsyncCommands, Client as RedisClient};
use std::{
    collections::HashMap, env, future::Future, net::SocketAddr, pin::Pin, str::FromStr, sync::Arc,
};
use tokio::{
    net::TcpListener,
    sync::{Mutex, RwLock},
    task::JoinSet,
};

#[tokio::main]
async fn main() -> Result<(), DynError> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to a config file", "CONFIG");
    opts.optflag("u", "uri", "show service URIs");
    let matches = opts.parse(&args[1..])?;
    let config_path = match matches.opt_str("c") {
        Some(path) => path,
        None => return Err("No config path was specified.".into()),
    };

    let config = AuthConfig::load(&config_path)?;

    // If the "u" flag is speficied, just show all sockets.
    if matches.opt_present("u") {
        for socket in config.expose.sockets {
            println!("{}", socket);
        }
        return Ok(());
    }

    let sockets = config.expose.sockets.clone();
    let redis_uri = config.redis.uri.clone();

    let config = Arc::new(RwLock::new(config));

    let mut join_set: JoinSet<Result<(), DynError>> = JoinSet::new();

    // Authentication Service
    for socket in sockets {
        let config = Arc::clone(&config);
        let redis = RedisClient::open(redis_uri.as_str())?;
        let redis = Arc::new(Mutex::new(redis));

        join_set.spawn(async move {
            let addr = SocketAddr::from_str(socket.as_str())?;
            let listener = TcpListener::bind(addr).await?;
            let svc = Service { config, redis };
            loop {
                let (stream, _) = listener.accept().await?;
                let io = TokioIo::new(stream);
                let svc_clone = svc.clone();
                tokio::task::spawn(async move {
                    if let Err(err) = http1::Builder::new().serve_connection(io, svc_clone).await {
                        error!("{:?}", err);
                    }
                });
            }
        });
    }

    while let Some(join) = join_set.join_next().await {
        match join {
            Ok(ret) => {
                if let Err(e) = ret {
                    error!("{}", e);
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Service {
    config: Arc<RwLock<AuthConfig>>,
    redis: Arc<Mutex<RedisClient>>,
}

impl HyperService<Request<Incoming>> for Service {
    type Response = Response<Full<Bytes>>;
    type Error = DynError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let config = Arc::clone(&self.config);
        let redis = Arc::clone(&self.redis);

        Box::pin(async move {
            let method = req.method();
            let path = req.uri().path();
            match (method, path) {
                (&Method::GET, "/auth") => auth(req, &redis, &config).await,
                (method, path) => {
                    Err(Error::new(format!("Illegal request ({} {})", method, path)).into())
                }
            }
        })
    }
}

async fn auth(
    req: Request<Incoming>,
    redis: &Arc<Mutex<RedisClient>>,
    config: &Arc<RwLock<AuthConfig>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    match auth_body(req, redis, config).await {
        Ok(res) => Ok(res),
        Err(e) => {
            error!("{}", e);
            Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body("".to_string().into_bytes().into())?)
        }
    }
}

async fn auth_body(
    req: Request<Incoming>,
    redis: &Arc<Mutex<RedisClient>>,
    config: &Arc<RwLock<AuthConfig>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let secret = {
        let config = config.read().await;
        config.security.password_secret.clone()
    };

    // Retrieve credentials
    let basic = match req.headers().get(header::AUTHORIZATION) {
        Some(basic) => basic.to_str()?.to_string(),
        None => return Err(Error::new("No authorization header was found.").into()),
    };
    let basic = match Credentials::from_header(basic) {
        Ok(basic) => basic,
        Err(e) => return Err(Error::new(e).into()),
    };

    // Retrieve service name
    let query = match req.uri().query() {
        Some(query) => query,
        None => return Err(Error::new("No query was specified.").into()),
    };
    let query: HashMap<String, String> = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect();
    let service = match query.get("service") {
        Some(service) => service,
        None => return Err(Error::new("No service was speficied.").into()),
    };

    // Retrieve a credential JSON from the Redis server
    let cred = {
        let redis = redis.lock().await;
        let mut redis = redis.get_multiplexed_async_connection().await?;
        redis.get::<&String, String>(&basic.user_id).await?
    };
    let cred: CredData = (&cred).try_into()?;

    // Verify
    if !cred.verify(&secret, &basic.password) {
        return Err(Error::new(format!("Invalid password for {}.", cred.id)).into());
    }
    if !cred.allowed(&service) {
        return Err(Error::new(format!("{} is not allowed for {}.", service, cred.id)).into());
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("".to_string().into_bytes().into())?)
}
