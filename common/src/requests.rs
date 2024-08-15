use mysql_async::prelude::*;
use mysql_async::{FromRowError, FromValueError, Row, Value};
use time::PrimitiveDateTime;
use crate::utils::get_from_row;

// ===================================================================
// Values
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
// Entities
// ===================================================================

pub struct Request {
    pub id: String,
    pub issuer_id: String,
    pub request_type: RequestType,
    pub status: i8,
    pub proc_id: Option<String>,
    pub description: Option<String>,
    pub rand: u64,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
    pub updated_by: String,
    pub updated_at: PrimitiveDateTime,
}

pub struct CreateUserRequest {
    pub id: String,
    pub username: Option<String>,
    pub email: String,
    pub pubkey: Vec<u8>,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
}

pub struct ChangePubkeyRequest {
    pub id: String,
    pub user_id: String,
    pub pubkey: Vec<u8>,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
}

pub struct RenewPasswordRequest {
    pub id: String,
    pub user_id: String,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
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