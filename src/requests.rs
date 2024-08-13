use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use time::PrimitiveDateTime;

pub struct Request {
    pub id: String,
    pub issuer_id: String,
    pub request_type: String,
    pub status: i8,
    pub proc_id: Option<String>,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: PrimitiveDateTime,
    pub updated_by: String,
    pub updated_at: PrimitiveDateTime,
}

pub struct RequestTicket {
    pub id: String,
    pub rnd: u32,
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
            id: row.get(0).unwrap(),
            issuer_id: row.get(1).unwrap(),
            request_type: row.get(2).unwrap(),
            status: row.get(3).unwrap(),
            proc_id: row.get(4),
            description: row.get(5),
            created_by: row.get(6).unwrap(),
            created_at: row.get(7).unwrap(),
            updated_by: row.get(8).unwrap(),
            updated_at: row.get(9).unwrap(),
        })
    }
}

impl FromRow for RequestTicket {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(RequestTicket {
            id: row.get(0).unwrap(),
            rnd: row.get(1).unwrap(),
            created_by: row.get(2).unwrap(),
            created_at: row.get(3).unwrap(),
        })
    }
}

impl FromRow for NewUserRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(NewUserRequest {
            id: row.get(0).unwrap(),
            username: row.get(1),
            email: row.get(2).unwrap(),
            pubkey: row.get(3).unwrap(),
            created_by: row.get(4).unwrap(),
            created_at: row.get(5).unwrap(),
        })
    }
}

impl FromRow for ChangingPubkeyRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(ChangingPubkeyRequest {
            id: row.get(0).unwrap(),
            user_id: row.get(1).unwrap(),
            pubkey: row.get(2).unwrap(),
            created_by: row.get(3).unwrap(),
            created_at: row.get(4).unwrap(),
        })
    }
}

impl FromRow for PasswordResetRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(PasswordResetRequest {
            id: row.get(0).unwrap(),
            user_id: row.get(1).unwrap(),
            created_by: row.get(2).unwrap(),
            created_at: row.get(3).unwrap(),
        })
    }
}