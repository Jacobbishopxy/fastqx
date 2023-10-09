//! file: constant.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:44:26 Wednesday
//! brief:

use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::adt::FqxValueType;

// ================================================================================================
// Const
// ================================================================================================

// https://docs.rs/sqlx-mysql/0.7.2/sqlx_mysql/types/index.html
pub(crate) static MYSQL_TMAP: Lazy<HashMap<&'static str, FqxValueType>> = Lazy::new(|| {
    HashMap::from([
        ("TINYINT(1)", FqxValueType::Bool),
        ("BOOLEAN", FqxValueType::Bool),
        ("TINYINT UNSIGNED", FqxValueType::U8),
        ("SMALLINT UNSIGNED", FqxValueType::U16),
        ("INT UNSIGNED", FqxValueType::U32),
        ("BIGINT UNSIGNED", FqxValueType::U64),
        ("TINYINT", FqxValueType::I8),
        ("SMALLINT", FqxValueType::I16),
        ("INT", FqxValueType::I32),
        ("BIGINT", FqxValueType::I64),
        ("FLOAT", FqxValueType::F32),
        ("DOUBLE", FqxValueType::F64),
        ("VARCHAR", FqxValueType::String),
        ("CHAR", FqxValueType::String),
        ("TEXT", FqxValueType::String),
        ("VARBINARY", FqxValueType::Blob),
        ("BINARY", FqxValueType::Blob),
        ("BLOB", FqxValueType::Blob),
    ])
});

// https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html
pub(crate) static POSTGRES_TMAP: Lazy<HashMap<&'static str, FqxValueType>> = Lazy::new(|| {
    HashMap::from([
        ("BOOL", FqxValueType::Bool),
        ("CHAR", FqxValueType::I8),
        ("SMALLINT", FqxValueType::I16),
        ("SMALLSERIAL", FqxValueType::I16),
        ("INT2", FqxValueType::I16),
        ("INT", FqxValueType::I32),
        ("SERIAL", FqxValueType::I32),
        ("INT4", FqxValueType::I32),
        ("BIGINT", FqxValueType::I64),
        ("BIGSERIAL", FqxValueType::I64),
        ("INT8", FqxValueType::I64),
        ("REAL", FqxValueType::F32),
        ("FLOAT4", FqxValueType::F32),
        ("DOUBLE PRECISION", FqxValueType::F64),
        ("FLOAT8", FqxValueType::F64),
        ("VARCHAR", FqxValueType::String),
        ("CHAR(N)", FqxValueType::String),
        ("TEXT", FqxValueType::String),
        ("NAME", FqxValueType::String),
        ("BYTEA", FqxValueType::Blob),
    ])
});

// https://docs.rs/sqlx-sqlite/0.7.2/sqlx_sqlite/types/index.html
pub(crate) static SQLITE_TMAP: Lazy<HashMap<&'static str, FqxValueType>> = Lazy::new(|| {
    HashMap::from([
        ("BOOLEAN", FqxValueType::Bool),
        ("INTEGER", FqxValueType::I32),
        ("BIGINT", FqxValueType::I64),
        ("INT8", FqxValueType::I64),
        ("REAL", FqxValueType::F64),
        ("VARCHAR", FqxValueType::String),
        ("CHAR(N)", FqxValueType::String),
        ("TEXT", FqxValueType::String),
        ("BLOB", FqxValueType::Blob),
    ])
});
