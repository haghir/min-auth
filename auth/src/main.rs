use bytes::Bytes;
use getopts::Options;
use http_auth_basic::Credentials;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::{body::Incoming, header, service::Service, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use log::error;
use min_auth_common::{
    config::auth::AuthConfig, data::credentials::Credentials as CredData, utils::emsg, DynError,
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

#[derive(Debug, Clone)]
pub struct AuthService {
    pub config: Arc<RwLock<AuthConfig>>,
    pub redis: Arc<Mutex<RedisClient>>,
}

impl Service<Request<Incoming>> for AuthService {
    type Response = Response<Full<Bytes>>;
    type Error = DynError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let config = Arc::clone(&self.config);
        let redis = Arc::clone(&self.redis);

        Box::pin(async move {
            let method = req.method();
            let path = req.uri().path();
            match match (method, path) {
                (&Method::GET, "/auth") => auth(req, &redis, &config).await,
                (method, path) => Err(emsg(format!("Illegal request ({} {})", method, path))),
            } {
                Ok(body) => Ok(body),
                Err(_) => Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("".to_string().into_bytes().into())?),
            }
        })
    }
}

async fn auth(
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
        None => return Err(emsg("No authorization header was found.")),
    };
    let basic = match Credentials::from_header(basic) {
        Ok(basic) => basic,
        Err(e) => return Err(emsg(e.to_string())),
    };

    // Retrieve service name
    let query = match req.uri().query() {
        Some(query) => query,
        None => return Err(emsg("No query was specified.")),
    };
    let query: HashMap<String, String> = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect();
    let service = match query.get("service") {
        Some(service) => service,
        None => return Err(emsg("No service was speficied.")),
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
        return Err(emsg(format!("Invalid password for {}.", cred.id)));
    }
    if !cred.allowed(&service) {
        return Err(emsg(format!("{} is not allowed for {}.", service, cred.id)));
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("".to_string().into_bytes().into())?)
}

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
    let redis = RedisClient::open(config.redis.uri.as_str())?;

    let config = Arc::new(RwLock::new(config));
    let redis = Arc::new(Mutex::new(redis));

    let mut join_set: JoinSet<Result<(), DynError>> = JoinSet::new();

    // Authentication Service
    for socket in sockets {
        let config = Arc::clone(&config);
        let redis = Arc::clone(&redis);

        join_set.spawn(async move {
            let addr = SocketAddr::from_str(socket.as_str())?;
            let listener = TcpListener::bind(addr).await?;
            let svc = AuthService { config, redis };
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
