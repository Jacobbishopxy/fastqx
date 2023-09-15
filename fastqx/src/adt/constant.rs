//! file: constant.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:44:26 Wednesday
//! brief:

use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::value::FastqxValueType;

// ================================================================================================
// Const
// ================================================================================================

// https://docs.rs/sqlx-mysql/0.7.1/sqlx_mysql/types/index.html
pub(crate) static MYSQL_TMAP: Lazy<HashMap<&'static str, FastqxValueType>> = Lazy::new(|| {
    HashMap::from([
        ("TINYINT(1)", FastqxValueType::Bool),
        ("BOOLEAN", FastqxValueType::Bool),
        ("TINYINT UNSIGNED", FastqxValueType::U8),
        ("SMALLINT UNSIGNED", FastqxValueType::U16),
        ("INT UNSIGNED", FastqxValueType::U32),
        ("BIGINT UNSIGNED", FastqxValueType::U64),
        ("TINYINT", FastqxValueType::I8),
        ("SMALLINT", FastqxValueType::I16),
        ("INT", FastqxValueType::I32),
        ("BIGINT", FastqxValueType::I64),
        ("FLOAT", FastqxValueType::F32),
        ("DOUBLE", FastqxValueType::F64),
        ("VARCHAR", FastqxValueType::String),
        ("CHAR", FastqxValueType::String),
        ("TEXT", FastqxValueType::String),
        ("VARBINARY", FastqxValueType::Blob),
        ("BINARY", FastqxValueType::Blob),
        ("BLOB", FastqxValueType::Blob),
    ])
});

// https://docs.rs/sqlx-postgres/0.7.1/sqlx_postgres/types/index.html
pub(crate) static POSTGRES_TMAP: Lazy<HashMap<&'static str, FastqxValueType>> = Lazy::new(|| {
    HashMap::from([
        ("BOOL", FastqxValueType::Bool),
        ("CHAR", FastqxValueType::I8),
        ("SMALLINT", FastqxValueType::I16),
        ("SMALLSERIAL", FastqxValueType::I16),
        ("INT2", FastqxValueType::I16),
        ("INT", FastqxValueType::I32),
        ("SERIAL", FastqxValueType::I32),
        ("INT4", FastqxValueType::I32),
        ("BIGINT", FastqxValueType::I64),
        ("BIGSERIAL", FastqxValueType::I64),
        ("INT8", FastqxValueType::I64),
        ("REAL", FastqxValueType::F32),
        ("FLOAT4", FastqxValueType::F32),
        ("DOUBLE PRECISION", FastqxValueType::F64),
        ("FLOAT8", FastqxValueType::F64),
        ("VARCHAR", FastqxValueType::String),
        ("CHAR(N)", FastqxValueType::String),
        ("TEXT", FastqxValueType::String),
        ("NAME", FastqxValueType::String),
        ("BYTEA", FastqxValueType::Blob),
    ])
});

// https://docs.rs/sqlx-sqlite/0.7.1/sqlx_sqlite/types/index.html
pub(crate) static SQLITE_TMAP: Lazy<HashMap<&'static str, FastqxValueType>> = Lazy::new(|| {
    HashMap::from([
        ("BOOLEAN", FastqxValueType::Bool),
        ("INTEGER", FastqxValueType::I32),
        ("BIGINT", FastqxValueType::I64),
        ("INT8", FastqxValueType::I64),
        ("REAL", FastqxValueType::F64),
        ("VARCHAR", FastqxValueType::String),
        ("CHAR(N)", FastqxValueType::String),
        ("TEXT", FastqxValueType::String),
        ("BLOB", FastqxValueType::Blob),
    ])
});
