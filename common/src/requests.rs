use serde::{Deserialize, Serialize};

// ===================================================================
// Request Type
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum RequestContent {
    CreateUser(CreateUserRequest),
    UpdateUser(UpdateUserRequest),
    RenewPassword(RenewPasswordRequest),
}

// ===================================================================
// Request
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Request {
    pub id: String,
    pub issuer: String,
    pub timestamp: String,
    pub content: RequestContent,
    pub rand: u64,
}

// ===================================================================
// Creating a User
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub superuser: bool,
}

// ===================================================================
// Changing the Public Key
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateUserRequest {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub superuser: Option<bool>,
    pub renew_password: bool,
}

// ===================================================================
// Renewing the Password
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RenewPasswordRequest {
    pub user_id: String,
}
