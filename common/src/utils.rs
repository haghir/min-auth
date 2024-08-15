use mysql_async::{FromRowError, Row};
use mysql_async::prelude::FromValue;

pub fn get_from_row<T: FromValue>(row: &Row, idx: usize) -> Result<T, FromRowError> {
    match row.get(idx) {
        Some(v) => Ok(v),
        None => Err(FromRowError(row.clone()))
    }
}
