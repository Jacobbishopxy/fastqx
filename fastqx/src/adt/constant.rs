//! file: constant.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:44:26 Wednesday
//! brief:

use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::value::RoughValueType;

// ================================================================================================
// Const
// ================================================================================================

// https://docs.rs/sqlx-mysql/0.7.1/sqlx_mysql/types/index.html
pub(crate) static MYSQL_TMAP: Lazy<HashMap<&'static str, RoughValueType>> = Lazy::new(|| {
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
pub(crate) static POSTGRES_TMAP: Lazy<HashMap<&'static str, RoughValueType>> = Lazy::new(|| {
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
pub(crate) static SQLITE_TMAP: Lazy<HashMap<&'static str, RoughValueType>> = Lazy::new(|| {
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
