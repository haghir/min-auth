use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use time::PrimitiveDateTime;
use crate::utils::get_from_row;

pub struct Request {
    pub id: String,
    pub issuer_id: String,
    pub request_type: String,
    pub status: i8,
    pub proc_id: Option<String>,
    pub description: Option<String>,
    pub rand: u32,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
    pub updated_by: String,
    pub updated_at: PrimitiveDateTime,
}

pub struct RequestTicket {
    pub id: String,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
}

pub struct NewUserRequest {
    pub id: String,
    pub username: Option<String>,
    pub email: String,
    pub pubkey: Vec<u8>,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
}

pub struct ChangingPubkeyRequest {
    pub id: String,
    pub user_id: String,
    pub pubkey: Vec<u8>,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
}

pub struct PasswordResetRequest {
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

impl FromRow for NewUserRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(NewUserRequest {
            id: get_from_row(&row, 0)?,
            username: get_from_row(&row, 1)?,
            email: get_from_row(&row, 2)?,
            pubkey: get_from_row(&row, 3)?,
            created_by: get_from_row(&row, 4)?,
            created_at: get_from_row(&row, 5)?,
        })
    }
}

impl FromRow for ChangingPubkeyRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(ChangingPubkeyRequest {
            id: get_from_row(&row, 0)?,
            user_id: get_from_row(&row, 1)?,
            pubkey: get_from_row(&row, 2)?,
            created_by: get_from_row(&row, 3)?,
            created_at: get_from_row(&row, 4)?,
        })
    }
}

impl FromRow for PasswordResetRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(PasswordResetRequest {
            id: get_from_row(&row, 0)?,
            user_id: get_from_row(&row, 1)?,
            created_by: get_from_row(&row, 2)?,
            created_at: get_from_row(&row, 3)?,
        })
    }
}