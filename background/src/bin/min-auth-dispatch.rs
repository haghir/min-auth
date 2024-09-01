use std::cell::RefCell;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use getopts::Options;
use mysql_async::{params, Conn, Opts, TxOpts};
use mysql_async::prelude::*;
use min_auth_common::config::BackgroundConfig;
use min_auth_common::error::Error;
use min_auth_common::requests::{ChangePubkeyRequest, CreateUserRequest, RenewPasswordRequest, Request, RequestState, RequestType};
use min_auth_common::Result;

// ===================================================================
// Load from DB
// ===================================================================

async fn get_request(conn: &RefCell<Conn>, id: &String) -> Result<Request> {
    let query = r#"SELECT
        `id`
    ,   `issuer_id`
    ,   `type`
    ,   `state`
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
        `state` = :state
    FOR UPDATE
    "#.with(params! {
        "id" => id,
        "state" => RequestState::New,
    });

    query.first(conn.borrow_mut().deref_mut()).await?.ok_or_else(||
        Error::from(format!("Request {} was not found.", id)))
}

async fn get_create_user_request(conn: &RefCell<Conn>, id: &String) -> Result<CreateUserRequest> {
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

    query.first(conn.borrow_mut().deref_mut()).await?.ok_or_else(||
        Error::from(format!("CreateUserRequest {} was not found.", id)))
}

async fn get_change_pubkey_request(conn: &RefCell<Conn>, id: &String) -> Result<ChangePubkeyRequest> {
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

    query.first(conn.borrow_mut().deref_mut()).await?.ok_or_else(||
        Error::from(format!("ChangePubkeyRequest {} was not found.", id)))
}

async fn get_renew_password_request(conn: &RefCell<Conn>, id: &String) -> Result<RenewPasswordRequest> {
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

    query.first(conn.borrow_mut().deref_mut()).await?.ok_or_else(||
        Error::from(format!("RenewPasswordRequest {} was not found.", id)))
}

// ===================================================================
// Update the request state
// ===================================================================

async fn update_state(conn: &RefCell<Conn>, id: &String) -> Result<()> {
    r"UPDATE requests SET state = :state WHERE id = :id".with(params! {
        "state" => RequestState::InProgress,
        "id" => id,
    })
        .ignore(conn.borrow_mut().deref_mut())
        .await?;

    Ok(())
}

// ===================================================================
// Request Handler
// ===================================================================

async fn handle_create_user_request<P: AsRef<Path>>(
    conn: &RefCell<Conn>, dir: P, request: &Request
) -> Result<()> {
    // Write the json to a file
    let sub_path = dir.as_ref().join("sub.json");
    let sub = get_create_user_request(conn, &request.id).await?;
    let sub_json = serde_json::to_string(&sub)?;
    let f = File::create_new(sub_path)?;
    write!(f, "{}", sub_json)?;

    // Write the pubkey to a file
    let pubkey_path = dir.as_ref().join("pubkey");
    let mut f = File::create_new(pubkey_path)?;
    f.write(sub.pubkey.as_slice())?;

    Ok(())
}

async fn handle_change_pubkey_request<P: AsRef<Path>>(
    conn: &RefCell<Conn>, dir: P, request: &Request
) -> Result<()> {
    // Write the json to a file
    let sub_path = dir.as_ref().join("sub.json");
    let sub = get_change_pubkey_request(conn, &request.id).await?;
    let sub_json = serde_json::to_string(&sub)?;
    let f = File::create_new(sub_path)?;
    write!(f, "{}", sub_json)?;

    // Write the pubkey to a file
    let pubkey_path = dir.as_ref().join("pubkey");
    let mut f = File::create_new(pubkey_path)?;
    f.write(sub.pubkey.as_slice())?;
    Ok(())
}

async fn handle_renew_password_request<P: AsRef<Path>>(
    conn: &RefCell<Conn>, dir: P, request: &Request
) -> Result<()> {
    // Write the json to a file
    let sub_path = dir.as_ref().join("sub.json");
    let sub = get_renew_password_request(conn, &request.id).await?;
    let sub_json = serde_json::to_string(&sub)?;
    let f = File::create_new(sub_path)?;
    write!(f, "{}", sub_json)?;
    Ok(())
}

// ===================================================================
// Dispatch
// ===================================================================

async fn dispatch(config: &BackgroundConfig, id: &String) -> Result<()> {
    // Initialize a MySQL client
    let opts: Opts = (&config.mysql).into();
    let mut conn = RefCell::new(Conn::new(opts).await?);
    let mut ref_conn = conn.borrow_mut();

    let tx_opts = TxOpts::default();
    let mut tx = ref_conn.start_transaction(tx_opts).await?;

    // Get the specified request
    let request = get_request(&conn, &id).await?;

    // Paths
    let tmp = "tmp".to_string();
    let tmp_dir: PathBuf = [&config.workspace_dir, &tmp, &id].iter().collect();

    create_dir_all(&tmp_dir)?;

    match request.request_type {
        RequestType::CreateUserRequest =>
            handle_create_user_request(&conn, &tmp_dir, &request).await?,
        RequestType::ChangePubkeyRequest =>
            handle_change_pubkey_request(&conn, &tmp_dir, &request).await?,
        RequestType::RenewPasswordRequest =>
            handle_renew_password_request(&conn, &tmp_dir, &request).await?,
    };

    update_state(&conn, &id).await?;

    // Print the worker index.
    print!("{}", request.rand % config.workers);

    tx.commit().await?;

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

    dispatch(&config, &id).await
}
