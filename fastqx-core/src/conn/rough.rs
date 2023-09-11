//! file: rough.rs
//! author: Jacob Xie
//! date: 2023/09/11 08:54:05 Monday
//! brief:

use std::collections::HashMap;

use anyhow::Result;
use once_cell::sync::Lazy;
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Row, TypeInfo};

// ================================================================================================
// RoughValueType & RoughValue
// ================================================================================================

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RoughValueType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Blob,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RoughValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Blob(Vec<u8>),
    Null,
}

// https://docs.rs/sqlx-mysql/0.7.1/sqlx_mysql/types/index.html
static MYSQL_TMAP: Lazy<HashMap<&'static str, RoughValueType>> = Lazy::new(|| {
    HashMap::from([
        ("TINYINT(1)", RoughValueType::Bool),
        ("BOOLEAN", RoughValueType::Bool),
        ("TINYINT UNSIGNED", RoughValueType::U8),
        ("SMALLINT UNSIGNED", RoughValueType::U16),
        ("INT UNSIGNED", RoughValueType::U32),
        ("BIGINT UNSIGNED", RoughValueType::U64),
        ("TINYINT", RoughValueType::I8),
        ("SMALLINT", RoughValueType::I16),
        ("INT", RoughValueType::I32),
        ("BIGINT", RoughValueType::I64),
        ("FLOAT", RoughValueType::F32),
        ("DOUBLE", RoughValueType::F64),
        ("VARCHAR", RoughValueType::String),
        ("CHAR", RoughValueType::String),
        ("TEXT", RoughValueType::String),
        ("VARBINARY", RoughValueType::Blob),
        ("BINARY", RoughValueType::Blob),
        ("BLOB", RoughValueType::Blob),
    ])
});

// https://docs.rs/sqlx-postgres/0.7.1/sqlx_postgres/types/index.html
static POSTGRES_TMAP: Lazy<HashMap<&'static str, RoughValueType>> = Lazy::new(|| {
    HashMap::from([
        ("BOOL", RoughValueType::Bool),
        ("CHAR", RoughValueType::I8),
        ("SMALLINT", RoughValueType::I16),
        ("SMALLSERIAL", RoughValueType::I16),
        ("INT2", RoughValueType::I16),
        ("INT", RoughValueType::I32),
        ("SERIAL", RoughValueType::I32),
        ("INT4", RoughValueType::I32),
        ("BIGINT", RoughValueType::I64),
        ("BIGSERIAL", RoughValueType::I64),
        ("INT8", RoughValueType::I64),
        ("REAL", RoughValueType::F32),
        ("FLOAT4", RoughValueType::F32),
        ("DOUBLE PRECISION", RoughValueType::F64),
        ("FLOAT8", RoughValueType::F64),
        ("VARCHAR", RoughValueType::String),
        ("CHAR(N)", RoughValueType::String),
        ("TEXT", RoughValueType::String),
        ("NAME", RoughValueType::String),
        ("BYTEA", RoughValueType::Blob),
    ])
});

// https://docs.rs/sqlx-sqlite/0.7.1/sqlx_sqlite/types/index.html
static SQLITE_TMAP: Lazy<HashMap<&'static str, RoughValueType>> = Lazy::new(|| {
    HashMap::from([
        ("BOOLEAN", RoughValueType::Bool),
        ("INTEGER", RoughValueType::I32),
        ("BIGINT", RoughValueType::I64),
        ("INT8", RoughValueType::I64),
        ("REAL", RoughValueType::F64),
        ("VARCHAR", RoughValueType::String),
        ("CHAR(N)", RoughValueType::String),
        ("TEXT", RoughValueType::String),
        ("BLOB", RoughValueType::Blob),
    ])
});

// ================================================================================================
// SqlxRow
// ================================================================================================

pub enum SqlxRow {
    M(MySqlRow),
    P(PgRow),
    S(SqliteRow),
}

impl From<MySqlRow> for SqlxRow {
    fn from(r: MySqlRow) -> Self {
        SqlxRow::M(r)
    }
}

impl From<PgRow> for SqlxRow {
    fn from(r: PgRow) -> Self {
        SqlxRow::P(r)
    }
}

impl From<SqliteRow> for SqlxRow {
    fn from(r: SqliteRow) -> Self {
        SqlxRow::S(r)
    }
}

/// generated code:
///
/// ```rs,ignore
/// let v: Option<bool> = r.try_get(idx)?;
/// Ok(v.map_or(RoughValue::Null, RoughValue::Bool))
/// ```
macro_rules! get_value {
    ($t:ty, $p:ident, $row:expr, $idx:expr) => {{
        let v: Option<$t> = $row.try_get($idx)?;
        Ok(v.map_or(RoughValue::Null, RoughValue::$p))
    }};
}

impl SqlxRow {
    pub fn get_type<S: AsRef<str>>(&self, type_name: S) -> RoughValueType {
        let tn = type_name.as_ref();
        match self {
            SqlxRow::M(_) => MYSQL_TMAP
                .get(tn)
                .cloned()
                .unwrap_or(RoughValueType::String),
            SqlxRow::P(_) => POSTGRES_TMAP
                .get(tn)
                .cloned()
                .unwrap_or(RoughValueType::String),
            SqlxRow::S(_) => SQLITE_TMAP
                .get(tn)
                .cloned()
                .unwrap_or(RoughValueType::String),
        }
    }

    pub fn get_value(
        &self,
        idx: usize,
        value_type: &RoughValueType,
    ) -> Result<RoughValue, sqlx::Error> {
        match &self {
            SqlxRow::M(r) => match value_type {
                RoughValueType::Bool => get_value!(bool, Bool, r, idx),
                RoughValueType::U8 => get_value!(u8, U8, r, idx),
                RoughValueType::U16 => get_value!(u16, U16, r, idx),
                RoughValueType::U32 => get_value!(u32, U32, r, idx),
                RoughValueType::U64 => get_value!(u64, U64, r, idx),
                RoughValueType::I8 => get_value!(i8, I8, r, idx),
                RoughValueType::I16 => get_value!(i16, I16, r, idx),
                RoughValueType::I32 => get_value!(i32, I32, r, idx),
                RoughValueType::I64 => get_value!(i64, I64, r, idx),
                RoughValueType::F32 => get_value!(f32, F32, r, idx),
                RoughValueType::F64 => get_value!(f64, F64, r, idx),
                RoughValueType::String => get_value!(String, String, r, idx),
                RoughValueType::Blob => get_value!(Vec<u8>, Blob, r, idx),
            },
            SqlxRow::P(r) => match value_type {
                RoughValueType::Bool => get_value!(bool, Bool, r, idx),
                RoughValueType::I8 => get_value!(i8, I8, r, idx),
                RoughValueType::I16 => get_value!(i16, I16, r, idx),
                RoughValueType::I32 => get_value!(i32, I32, r, idx),
                RoughValueType::I64 => get_value!(i64, I64, r, idx),
                RoughValueType::F32 => get_value!(f32, F32, r, idx),
                RoughValueType::F64 => get_value!(f64, F64, r, idx),
                RoughValueType::String => get_value!(String, String, r, idx),
                RoughValueType::Blob => get_value!(Vec<u8>, Blob, r, idx),
                _ => get_value!(String, String, r, idx),
            },
            SqlxRow::S(r) => match value_type {
                RoughValueType::Bool => get_value!(bool, Bool, r, idx),
                RoughValueType::U8 => get_value!(u8, U8, r, idx),
                RoughValueType::U16 => get_value!(u16, U16, r, idx),
                RoughValueType::U32 => get_value!(u32, U32, r, idx),
                RoughValueType::I8 => get_value!(i8, I8, r, idx),
                RoughValueType::I16 => get_value!(i16, I16, r, idx),
                RoughValueType::I32 => get_value!(i32, I32, r, idx),
                RoughValueType::I64 => get_value!(i64, I64, r, idx),
                RoughValueType::F32 => get_value!(f32, F32, r, idx),
                RoughValueType::F64 => get_value!(f64, F64, r, idx),
                RoughValueType::String => get_value!(String, String, r, idx),
                RoughValueType::Blob => get_value!(Vec<u8>, Blob, r, idx),
                _ => get_value!(String, String, r, idx),
            },
        }
    }
}

// ================================================================================================
// SqlxRowProcessor
// ================================================================================================

pub struct SqlxRowProcessor {
    cache: Option<Vec<(String, RoughValueType)>>,
}

macro_rules! cache_info_branch {
    ($row:expr,$r:expr,$s:expr) => {{
        let c = $r
            .columns()
            .iter()
            .map(|e| {
                let name = e.name().to_string();
                let ty = $row.get_type(e.type_info().name());
                (name, ty)
            })
            .collect::<Vec<_>>();
        $s.cache = Some(c)
    }};
}

impl SqlxRowProcessor {
    pub fn new() -> Self {
        Self { cache: None }
    }

    pub fn cache(&self) -> Option<&[(String, RoughValueType)]> {
        self.cache.as_deref()
    }

    fn cache_info(&mut self, row: &SqlxRow) -> &[(String, RoughValueType)] {
        if self.cache.is_none() {
            match row {
                SqlxRow::M(r) => cache_info_branch!(row, r, self),
                SqlxRow::P(r) => cache_info_branch!(row, r, self),
                SqlxRow::S(r) => cache_info_branch!(row, r, self),
            }
        }

        self.cache.as_deref().unwrap()
    }

    pub fn process<S: Into<SqlxRow>>(&mut self, row: S) -> Result<Vec<RoughValue>, sqlx::Error> {
        let row: SqlxRow = row.into();
        let cache = self.cache_info(&row);

        let res = cache
            .iter()
            .enumerate()
            .map(|(idx, (_, vt))| row.get_value(idx, vt))
            .collect::<Result<Vec<_>, sqlx::Error>>()?;

        Ok(res)
    }
}
