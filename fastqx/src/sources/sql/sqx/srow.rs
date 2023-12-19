//! file: srow.rs
//! author: Jacob Xie
//! date: 2023/12/18 20:10:34 Monday
//! brief:

use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

// ================================================================================================
// FromSqlxRow
// ================================================================================================

pub trait FromSqlxRow<R>: Sized + Unpin + Send + Sync
where
    R: Row,
{
    fn from_row(row: R) -> Result<Self, sqlx::Error>;
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

macro_rules! impl_try_get_from_mysqlrow {
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
    ($r:ty, $t:ty) => {
        impl TryGetFromSqlxRow<MySqlRow> for $r {
            fn try_get(row: &MySqlRow, col_name: &str) -> Result<Self> {
                let val: $t = row.try_get(col_name)?;

                Ok(val as $r)
            }
        }

        impl TryGetFromSqlxRow<MySqlRow> for Option<$r> {
            fn try_get(row: &MySqlRow, col_name: &str) -> Result<Self> {
                let val: Option<$t> = row.try_get(col_name)?;

                Ok(val.map(|e| e as $r))
            }
        }
    };
}

macro_rules! impl_try_get_from_pgrow {
    ($t:ty) => {
        impl TryGetFromSqlxRow<PgRow> for $t {
            fn try_get(row: &PgRow, col_name: &str) -> Result<Self> {
                let val: $t = row.try_get(col_name)?;

                Ok(val)
            }
        }

        impl TryGetFromSqlxRow<PgRow> for Option<$t> {
            fn try_get(row: &PgRow, col_name: &str) -> Result<Self> {
                let val: Option<$t> = row.try_get(col_name)?;

                Ok(val)
            }
        }
    };
    ($r:ty, $t:ty) => {
        impl TryGetFromSqlxRow<PgRow> for $r {
            fn try_get(row: &PgRow, col_name: &str) -> Result<Self> {
                let val: $t = row.try_get(col_name)?;

                Ok(val as $r)
            }
        }

        impl TryGetFromSqlxRow<PgRow> for Option<$r> {
            fn try_get(row: &PgRow, col_name: &str) -> Result<Self> {
                let val: Option<$t> = row.try_get(col_name)?;

                Ok(val.map(|e| e as $r))
            }
        }
    };
}

macro_rules! impl_try_get_from_sqliterow {
    ($t:ty) => {
        impl TryGetFromSqlxRow<SqliteRow> for $t {
            fn try_get(row: &SqliteRow, col_name: &str) -> Result<Self> {
                let val: $t = row.try_get(col_name)?;

                Ok(val)
            }
        }

        impl TryGetFromSqlxRow<SqliteRow> for Option<$t> {
            fn try_get(row: &SqliteRow, col_name: &str) -> Result<Self> {
                let val: Option<$t> = row.try_get(col_name)?;

                Ok(val)
            }
        }
    };
    ($r:ty, $t:ty) => {
        impl TryGetFromSqlxRow<SqliteRow> for $r {
            fn try_get(row: &SqliteRow, col_name: &str) -> Result<Self> {
                let val: $t = row.try_get(col_name)?;

                Ok(val as $r)
            }
        }

        impl TryGetFromSqlxRow<SqliteRow> for Option<$r> {
            fn try_get(row: &SqliteRow, col_name: &str) -> Result<Self> {
                let val: Option<$t> = row.try_get(col_name)?;

                Ok(val.map(|e| e as $r))
            }
        }
    };
}

impl_try_get_from_mysqlrow!(bool);
impl_try_get_from_pgrow!(bool);
impl_try_get_from_sqliterow!(bool);

impl_try_get_from_mysqlrow!(u8);
impl_try_get_from_pgrow!(u8, i16);
impl_try_get_from_sqliterow!(u8);

impl_try_get_from_mysqlrow!(u16);
impl_try_get_from_pgrow!(u16, i32);
impl_try_get_from_sqliterow!(u16);

impl_try_get_from_mysqlrow!(u32);
impl_try_get_from_pgrow!(u32, i64);
impl_try_get_from_sqliterow!(u32);

impl_try_get_from_mysqlrow!(u64);
impl_try_get_from_pgrow!(u64, i64);
impl_try_get_from_sqliterow!(u64, i64);

impl_try_get_from_mysqlrow!(i8);
impl_try_get_from_pgrow!(i8);
impl_try_get_from_sqliterow!(i8);

impl_try_get_from_mysqlrow!(i16);
impl_try_get_from_pgrow!(i16);
impl_try_get_from_sqliterow!(i16);

impl_try_get_from_mysqlrow!(i32);
impl_try_get_from_pgrow!(i32);
impl_try_get_from_sqliterow!(i32);

impl_try_get_from_mysqlrow!(i64);
impl_try_get_from_pgrow!(i64);
impl_try_get_from_sqliterow!(i64);

impl_try_get_from_mysqlrow!(String);
impl_try_get_from_pgrow!(String);
impl_try_get_from_sqliterow!(String);

impl_try_get_from_mysqlrow!(Vec<u8>);
impl_try_get_from_pgrow!(Vec<u8>);
impl_try_get_from_sqliterow!(Vec<u8>);

impl_try_get_from_mysqlrow!(DateTime<Local>);
impl_try_get_from_pgrow!(DateTime<Local>);
impl_try_get_from_sqliterow!(DateTime<Local>);

impl_try_get_from_mysqlrow!(NaiveDateTime);
impl_try_get_from_pgrow!(NaiveDateTime);
impl_try_get_from_sqliterow!(NaiveDateTime);

impl_try_get_from_mysqlrow!(NaiveDate);
impl_try_get_from_pgrow!(NaiveDate);
impl_try_get_from_sqliterow!(NaiveDate);

impl_try_get_from_mysqlrow!(NaiveTime);
impl_try_get_from_pgrow!(NaiveTime);
impl_try_get_from_sqliterow!(NaiveTime);
