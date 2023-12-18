//! file: srow.rs
//! author: Jacob Xie
//! date: 2023/12/18 20:10:34 Monday
//! brief:

use anyhow::Result;
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Database, Row};

// ================================================================================================
// FromSqlxRow
// ================================================================================================

pub trait FromSqlxRow<'r, R>: Sized
where
    R: Row,
{
    fn from_row(row: &'r R) -> Result<Self>;
}

pub trait TryGetFromSqlxRow<R>: Sized
where
    R: Row,
{
    fn try_get(row: &R, col_name: &str) -> Result<Self>;
}

// ================================================================================================
// Impl
// ================================================================================================

// TODO

macro_rules! impl_try_get_from_row {
    ($t:ty) => {
        impl TryGetFromSqlxRow<MySqlRow> for $t {
            fn try_get(row: &MySqlRow, col_name: &str) -> Result<Self> {
                let val: $t = row.try_get(col_name)?;

                Ok(val)
            }
        }

        impl TryGetFromSqlxRow<MySqlRow> for Option<$t> {
            fn try_get(row: &MySqlRow, col_name: &str) -> Result<Self> {
                let val: Option<$t> = row.try_get(col_name)?;

                Ok(val)
            }
        }
    };
}
