use std::{env, net::SocketAddr, str::FromStr, sync::Arc};

use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use getopts::Options;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use log::error;
use min_auth_common::{config::admin::AdminConfig, data::users::User, DynError};
use tokio::{net::TcpListener, sync::RwLock, task::JoinSet};

#[tokio::main]
async fn main() -> Result<(), DynError> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "path to a config file", "CONFIG");
    let matches = opts.parse(&args[1..])?;
    let config_path = match matches.opt_str("c") {
        Some(path) => path,
        None => return Err("No config path was specified.".into()),
    };

    let config = AdminConfig::load(&config_path)?;
    let addrs = config.expose.sockets.clone();
    let users = User::load_all(&config.file_system.users)?;
    let session_key = Aes256Gcm::generate_key(OsRng);

    let config = Arc::new(RwLock::new(config));
    let users = Arc::new(RwLock::new(users));
    let session_key = Arc::new(RwLock::new(session_key));

    let mut join_set: JoinSet<Result<(), DynError>> = JoinSet::new();

    for addr in addrs {
        let config = Arc::clone(&config);
        let users = Arc::clone(&users);
        let session_key = Arc::clone(&session_key);

        join_set.spawn(async move {
            let addr = SocketAddr::from_str(addr.as_str())?;
            let listener = TcpListener::bind(addr).await?;
            let svc = AdminService {
                config,
                users,
                session_key,
            };
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
