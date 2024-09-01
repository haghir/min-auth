use serde::{Deserialize, Serialize};

// ===================================================================
// Request Type
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum RequestType {
    CreateUserRequest(CreateUserRequest),
    ChangePubkeyRequest(ChangePubkeyRequest),
    RenewPasswordRequest(ChangePubkeyRequest),
}

// ===================================================================
// Request
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Request {
    pub id: String,
    pub issuer: String,
    pub timestamp: String,
    pub content: RequestType,
}

// ===================================================================
// Creating a User
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub pubkey_id: String,
}

// ===================================================================
// Changing the Public Key
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChangePubkeyRequest {
    pub user_id: String,
    pub pubkey_id: String,
}

// ===================================================================
// Renewing the Password
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RenewPasswordRequest {
    pub user_id: String,
}
