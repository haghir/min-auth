use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use getopts::Options;
use mysql_async::{params, Conn, Opts, TxOpts};
use mysql_async::prelude::*;
use min_auth_common::config::BackgroundConfig;
use min_auth_common::error::Error;
use min_auth_common::requests::{ChangePubkeyRequest, CreateUserRequest, RenewPasswordRequest, Request, RequestStatus, RequestType};
use min_auth_common::Result;

// ===================================================================
// Load from DB
// ===================================================================

async fn get_request(conn: &mut Conn, id: &String) -> Result<Request> {
    let query = r#"SELECT
        `id`
    ,   `issuer_id`
    ,   `type`
    ,   `status`
    ,   `proc_id`
    ,   `description`
    ,   `rand`
    ,   `created_by`
    ,   `created_at`
    ,   `updated_by`
    ,   `updated_at`
    FROM
        `requests`
    WHERE
        `id` = :id AND
        `status` = :status
    FOR UPDATE
    "#.with(params! {
        "id" => id,
        "status" => RequestStatus::New,
    });

    query.first(conn).await?.ok_or_else(||
        Error::from(format!("Request {} was not found.", id)))
}

async fn get_create_user_request(conn: &mut Conn, id: &String) -> Result<CreateUserRequest> {
    let query = r#"SELECT
        `id`
    ,   `username`
    ,   `email`
    ,   `pubkey`
    ,   `created_by`
    ,   `created_at`
    FROM
        `create_user_requests`
    WHERE
        `id` = :id
    "#.with(params! {
        "id" => id,
    });

    query.first(conn).await?.ok_or_else(||
        Error::from(format!("CreateUserRequest {} was not found.", id)))
}

async fn get_change_pubkey_request(conn: &mut Conn, id: &String) -> Result<ChangePubkeyRequest> {
    let query = r#"SELECT
        `id`
    ,   `user_id`
    ,   `pubkey`
    ,   `created_by`
    ,   `created_at`
    FROM
        `change_pubkey_requests`
    WHERE
        `id` = :id
    "#.with(params! {
        "id" => id,
    });

    query.first(conn).await?.ok_or_else(||
        Error::from(format!("ChangePubkeyRequest {} was not found.", id)))
}

async fn get_renew_password_request(conn: &mut Conn, id: &String) -> Result<RenewPasswordRequest> {
    let query = r#"SELECT
        `id`
    ,   `user_id`
    ,   `created_by`
    ,   `created_at`
    FROM
        `renew_password_requests`
    WHERE
        `id` = :id
    "#.with(params! {
        "id" => id,
    });

    query.first(conn).await?.ok_or_else(||
        Error::from(format!("RenewPasswordRequest {} was not found.", id)))
}

// ===================================================================
// Request Handler
// ===================================================================

async fn handle_create_user_request<P: AsRef<Path>>(
    conn: &mut Conn, dir: P, request: &Request
) -> Result<()> {
    let sub = get_create_user_request(conn, &request.id).await?;
    Ok(())
}

async fn handle_change_pubkey_request<P: AsRef<Path>>(
    conn: &mut Conn, dir: P, request: &Request
) -> Result<()> {
    let sub = get_change_pubkey_request(conn, &request.id).await?;
    Ok(())
}

async fn handle_renew_password_request<P: AsRef<Path>>(
    conn: &mut Conn, dir: P, request: &Request
) -> Result<()> {
    let sub = get_renew_password_request(conn, &request.id).await?;
    Ok(())
}

// ===================================================================
// Entry Point
// ===================================================================

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "config", "path to config file", "CONFIG");
    opts.optopt("i", "id", "request ID", "ID");

    let matches = opts.parse(&args[1..])?;
    let config_path = matches.opt_str("c")
        .ok_or(Error::from("No config path was specified."))?;
    let id = matches.opt_str("i")
        .ok_or(Error::from("No ID was specified."))?;

    // Load a configuration file
    let config: String = std::fs::read_to_string(config_path)?;
    let config = <&String as TryInto<BackgroundConfig>>::try_into(&config)?;

    // Initialize a MySQL client
    let opts: Opts = (&config.mysql).into();
    let mut conn = Conn::new(opts).await?;

    let tx_opts = TxOpts::default();
    let mut tx = conn.start_transaction(tx_opts).await?;

    // Get the specified request
    let request = get_request(&mut conn, &id).await?;
    let worker = request.rand % config.workers;

    // Paths
    let tmp = "tmp".to_string();
    let tmp_dir: PathBuf = [&config.workspace_dir, &tmp, &id].iter().collect();
    let wk_dir: PathBuf = [&config.workspace_dir, &worker.to_string(), &id].iter().collect();

    create_dir_all(&tmp_dir)?;

    match request.request_type {
        RequestType::CreateUserRequest =>
            handle_create_user_request(&mut conn, &tmp_dir, &request).await?,
        RequestType::ChangePubkeyRequest =>
            handle_change_pubkey_request(&mut conn, &tmp_dir, &request).await?,
        RequestType::RenewPasswordRequest =>
            handle_renew_password_request(&mut conn, &tmp_dir, &request).await?,
    };

    tx.commit().await?;

    Ok(())
}