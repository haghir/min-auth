use serde::{Deserialize, Serialize};

// ===================================================================
// Access Control Type
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum AccessControlType {
    Allow,
    Deny,
}

// ===================================================================
// Users
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    id: String,
    username: String,
    email: String,
    salt: String,
    password_hash: String,
    pubkey_fpr: String,
    acl: Vec<AccessControl>,
}

// ===================================================================
// Access Control
// ===================================================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AccessControl {
    control: AccessControlType,
    service: String,
}