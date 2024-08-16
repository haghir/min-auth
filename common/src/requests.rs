use mysql_async::prelude::*;
use mysql_async::{FromRowError, FromValueError, Row, Value};
use time::PrimitiveDateTime;
use crate::utils::get_from_row;

// ===================================================================
// Request Type
// ===================================================================

pub enum RequestType {
    CreateUserRequest,
    ChangePubkeyRequest,
    RenewPasswordRequest,
}

impl Into<Value> for RequestType {
    fn into(self) -> Value {
        match self {
            RequestType::CreateUserRequest => Value::Int(1),
            RequestType::ChangePubkeyRequest => Value::Int(2),
            RequestType::RenewPasswordRequest => Value::Int(3),
        }
    }
}

impl TryFrom<Value> for RequestType {
    type Error = FromValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(code) => match code {
                1 => Ok(RequestType::CreateUserRequest),
                2 => Ok(RequestType::ChangePubkeyRequest),
                3 => Ok(RequestType::RenewPasswordRequest),
                _ => Err(FromValueError(value))
            },
            _ => Err(FromValueError(value)),
        }
    }
}

impl FromValue for RequestType {
    type Intermediate = RequestType;
}

// ===================================================================
// Request Status
// ===================================================================

pub enum RequestStatus {
    New,
    InProgress,
    Succeeded,
    Failed(i8),
}

impl Into<Value> for RequestStatus {
    fn into(self) -> Value {
        match self {
            RequestStatus::New => Value::Int(0),
            RequestStatus::InProgress => Value::Int(1),
            RequestStatus::Succeeded => Value::Int(2),
            RequestStatus::Failed(e) => Value::Int(e as i64),
        }
    }
}

impl TryFrom<Value> for RequestStatus {
    type Error = FromValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(code) => match code {
                0 => Ok(Self::New),
                1 => Ok(Self::InProgress),
                2 => Ok(Self::Succeeded),
                n => if i8::MIN as i64 <= n && n < 0 {
                    Ok(Self::Failed(n as i8))
                } else {
                    Err(FromValueError(value))
                }
            },
            _ => Err(FromValueError(value)),
        }
    }
}

impl FromValue for RequestStatus {
    type Intermediate = RequestStatus;
}

// ===================================================================
// Request
// ===================================================================

pub struct Request {
    pub id: String,
    pub issuer_id: String,
    pub request_type: RequestType,
    pub status: RequestStatus,
    pub proc_id: Option<String>,
    pub description: Option<String>,
    pub rand: u32,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
    pub updated_by: String,
    pub updated_at: PrimitiveDateTime,
}

impl FromRow for Request {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(Request {
            id: get_from_row(&row, 0)?,
            issuer_id: get_from_row(&row, 1)?,
            request_type: get_from_row(&row, 2)?,
            status: get_from_row(&row, 3)?,
            proc_id: get_from_row(&row, 4)?,
            description: row.get(5),
            rand: get_from_row(&row, 6)?,
            created_by: get_from_row(&row, 7)?,
            created_at: get_from_row(&row, 8)?,
            updated_by: get_from_row(&row, 9)?,
            updated_at: get_from_row(&row, 10)?,
        })
    }
}

// ===================================================================
// Creating a User
// ===================================================================

pub struct CreateUserRequest {
    pub id: String,
    pub username: Option<String>,
    pub email: String,
    pub pubkey: Vec<u8>,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
}

impl FromRow for CreateUserRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(CreateUserRequest {
            id: get_from_row(&row, 0)?,
            username: get_from_row(&row, 1)?,
            email: get_from_row(&row, 2)?,
            pubkey: get_from_row(&row, 3)?,
            created_by: get_from_row(&row, 4)?,
            created_at: get_from_row(&row, 5)?,
        })
    }
}

// ===================================================================
// Changing the Public Key
// ===================================================================

pub struct ChangePubkeyRequest {
    pub id: String,
    pub user_id: String,
    pub pubkey: Vec<u8>,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
}

impl FromRow for ChangePubkeyRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(ChangePubkeyRequest {
            id: get_from_row(&row, 0)?,
            user_id: get_from_row(&row, 1)?,
            pubkey: get_from_row(&row, 2)?,
            created_by: get_from_row(&row, 3)?,
            created_at: get_from_row(&row, 4)?,
        })
    }
}

// ===================================================================
// Renewing the Password
// ===================================================================

pub struct RenewPasswordRequest {
    pub id: String,
    pub user_id: String,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
}

impl FromRow for RenewPasswordRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(RenewPasswordRequest {
            id: get_from_row(&row, 0)?,
            user_id: get_from_row(&row, 1)?,
            created_by: get_from_row(&row, 2)?,
            created_at: get_from_row(&row, 3)?,
        })
    }
}
