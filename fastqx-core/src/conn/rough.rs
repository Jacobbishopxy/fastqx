//! file: rough.rs
//! author: Jacob Xie
//! date: 2023/09/11 08:54:05 Monday
//! brief: for both dynamic query and Pyo3

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Row, TypeInfo};

// ================================================================================================
// Const
// ================================================================================================

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
// RoughValueType & RoughValue
// ================================================================================================

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
    Null,
}

impl<'source> FromPyObject<'source> for RoughValueType {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        ob.extract::<String>().and_then(|v| match v.as_str() {
            "Bool" => Ok(RoughValueType::Bool),
            "U8" => Ok(RoughValueType::U8),
            "U16" => Ok(RoughValueType::U16),
            "U32" => Ok(RoughValueType::U32),
            "U64" => Ok(RoughValueType::U64),
            "I8" => Ok(RoughValueType::I8),
            "I16" => Ok(RoughValueType::I16),
            "I32" => Ok(RoughValueType::I32),
            "I64" => Ok(RoughValueType::I64),
            "F32" => Ok(RoughValueType::F32),
            "F64" => Ok(RoughValueType::F64),
            "String" => Ok(RoughValueType::String),
            "Blob" => Ok(RoughValueType::Blob),
            "Null" => Ok(RoughValueType::Null),
            _ => Err(anyhow!("wrong type annotation").into()),
        })
    }
}

impl IntoPy<PyObject> for RoughValueType {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            RoughValueType::Bool => "Bool".into_py(py),
            RoughValueType::U8 => "U8".into_py(py),
            RoughValueType::U16 => "U16".into_py(py),
            RoughValueType::U32 => "U32".into_py(py),
            RoughValueType::U64 => "U64".into_py(py),
            RoughValueType::I8 => "I8".into_py(py),
            RoughValueType::I16 => "I16".into_py(py),
            RoughValueType::I32 => "I32".into_py(py),
            RoughValueType::I64 => "I64".into_py(py),
            RoughValueType::F32 => "F32".into_py(py),
            RoughValueType::F64 => "F64".into_py(py),
            RoughValueType::String => "String".into_py(py),
            RoughValueType::Blob => "Blob".into_py(py),
            RoughValueType::Null => "Null".into_py(py),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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

impl TryFrom<RoughValue> for bool {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(v) => Ok(v),
            RoughValue::U8(v) => Ok(v == 0),
            RoughValue::U16(v) => Ok(v == 0),
            RoughValue::U32(v) => Ok(v == 0),
            RoughValue::U64(v) => Ok(v == 0),
            RoughValue::I8(v) => Ok(v == 0),
            RoughValue::I16(v) => Ok(v == 0),
            RoughValue::I32(v) => Ok(v == 0),
            RoughValue::I64(v) => Ok(v == 0),
            RoughValue::F32(v) => Ok(v == 0.0),
            RoughValue::F64(v) => Ok(v == 0.0),
            RoughValue::String(v) => Ok(&v != "0"),
            RoughValue::Blob(v) => Ok(!v.is_empty()),
            RoughValue::Null => Ok(false),
        }
    }
}

impl TryFrom<RoughValue> for u8 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into u8")),
            RoughValue::U8(v) => Ok(v),
            RoughValue::U16(v) => Ok(v.try_into()?),
            RoughValue::U32(v) => Ok(v.try_into()?),
            RoughValue::U64(v) => Ok(v.try_into()?),
            RoughValue::I8(v) => Ok(v.try_into()?),
            RoughValue::I16(v) => Ok(v.try_into()?),
            RoughValue::I32(v) => Ok(v.try_into()?),
            RoughValue::I64(v) => Ok(v.try_into()?),
            RoughValue::F32(_) => Err(anyhow!("failed to convert f32 into u8")),
            RoughValue::F64(_) => Err(anyhow!("failed to convert f64 into u8")),
            RoughValue::String(v) => Ok(v.parse::<u8>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u8")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into u8")),
        }
    }
}

impl TryFrom<RoughValue> for u16 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into u16")),
            RoughValue::U8(v) => Ok(v.try_into()?),
            RoughValue::U16(v) => Ok(v),
            RoughValue::U32(v) => Ok(v.try_into()?),
            RoughValue::U64(v) => Ok(v.try_into()?),
            RoughValue::I8(v) => Ok(v.try_into()?),
            RoughValue::I16(v) => Ok(v.try_into()?),
            RoughValue::I32(v) => Ok(v.try_into()?),
            RoughValue::I64(v) => Ok(v.try_into()?),
            RoughValue::F32(_) => Err(anyhow!("failed to convert f32 into u16")),
            RoughValue::F64(_) => Err(anyhow!("failed to convert f64 into u16")),
            RoughValue::String(v) => Ok(v.parse::<u16>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u16")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into u16")),
        }
    }
}

impl TryFrom<RoughValue> for u32 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into u32")),
            RoughValue::U8(v) => Ok(v.try_into()?),
            RoughValue::U16(v) => Ok(v.try_into()?),
            RoughValue::U32(v) => Ok(v),
            RoughValue::U64(v) => Ok(v.try_into()?),
            RoughValue::I8(v) => Ok(v.try_into()?),
            RoughValue::I16(v) => Ok(v.try_into()?),
            RoughValue::I32(v) => Ok(v.try_into()?),
            RoughValue::I64(v) => Ok(v.try_into()?),
            RoughValue::F32(_) => Err(anyhow!("failed to convert f32 into u32")),
            RoughValue::F64(_) => Err(anyhow!("failed to convert f64 into u32")),
            RoughValue::String(v) => Ok(v.parse::<u32>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u32")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into u32")),
        }
    }
}

impl TryFrom<RoughValue> for u64 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into u64")),
            RoughValue::U8(v) => Ok(v.try_into()?),
            RoughValue::U16(v) => Ok(v.try_into()?),
            RoughValue::U32(v) => Ok(v.try_into()?),
            RoughValue::U64(v) => Ok(v),
            RoughValue::I8(v) => Ok(v.try_into()?),
            RoughValue::I16(v) => Ok(v.try_into()?),
            RoughValue::I32(v) => Ok(v.try_into()?),
            RoughValue::I64(v) => Ok(v.try_into()?),
            RoughValue::F32(_) => Err(anyhow!("failed to convert f32 into u64")),
            RoughValue::F64(_) => Err(anyhow!("failed to convert f64 into u64")),
            RoughValue::String(v) => Ok(v.parse::<u64>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u64")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into u64")),
        }
    }
}

impl TryFrom<RoughValue> for i8 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into i8")),
            RoughValue::U8(v) => Ok(v.try_into()?),
            RoughValue::U16(v) => Ok(v.try_into()?),
            RoughValue::U32(v) => Ok(v.try_into()?),
            RoughValue::U64(v) => Ok(v.try_into()?),
            RoughValue::I8(v) => Ok(v),
            RoughValue::I16(v) => Ok(v.try_into()?),
            RoughValue::I32(v) => Ok(v.try_into()?),
            RoughValue::I64(v) => Ok(v.try_into()?),
            RoughValue::F32(_) => Err(anyhow!("failed to convert f32 into i8")),
            RoughValue::F64(_) => Err(anyhow!("failed to convert f64 into i8")),
            RoughValue::String(v) => Ok(v.parse::<i8>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i8")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into i8")),
        }
    }
}

impl TryFrom<RoughValue> for i16 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into i16")),
            RoughValue::U8(v) => Ok(v.try_into()?),
            RoughValue::U16(v) => Ok(v.try_into()?),
            RoughValue::U32(v) => Ok(v.try_into()?),
            RoughValue::U64(v) => Ok(v.try_into()?),
            RoughValue::I8(v) => Ok(v.try_into()?),
            RoughValue::I16(v) => Ok(v),
            RoughValue::I32(v) => Ok(v.try_into()?),
            RoughValue::I64(v) => Ok(v.try_into()?),
            RoughValue::F32(_) => Err(anyhow!("failed to convert f32 into i16")),
            RoughValue::F64(_) => Err(anyhow!("failed to convert f64 into i16")),
            RoughValue::String(v) => Ok(v.parse::<i16>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i16")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into i16")),
        }
    }
}

impl TryFrom<RoughValue> for i32 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into i32")),
            RoughValue::U8(v) => Ok(v.try_into()?),
            RoughValue::U16(v) => Ok(v.try_into()?),
            RoughValue::U32(v) => Ok(v.try_into()?),
            RoughValue::U64(v) => Ok(v.try_into()?),
            RoughValue::I8(v) => Ok(v.try_into()?),
            RoughValue::I16(v) => Ok(v.try_into()?),
            RoughValue::I32(v) => Ok(v),
            RoughValue::I64(v) => Ok(v.try_into()?),
            RoughValue::F32(_) => Err(anyhow!("failed to convert f32 into i32")),
            RoughValue::F64(_) => Err(anyhow!("failed to convert f64 into i32")),
            RoughValue::String(v) => Ok(v.parse::<i32>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i32")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into i32")),
        }
    }
}

impl TryFrom<RoughValue> for i64 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into u64")),
            RoughValue::U8(v) => Ok(v.try_into()?),
            RoughValue::U16(v) => Ok(v.try_into()?),
            RoughValue::U32(v) => Ok(v.try_into()?),
            RoughValue::U64(v) => Ok(v.try_into()?),
            RoughValue::I8(v) => Ok(v.try_into()?),
            RoughValue::I16(v) => Ok(v.try_into()?),
            RoughValue::I32(v) => Ok(v.try_into()?),
            RoughValue::I64(v) => Ok(v),
            RoughValue::F32(_) => Err(anyhow!("failed to convert f32 into i64")),
            RoughValue::F64(_) => Err(anyhow!("failed to convert f64 into i64")),
            RoughValue::String(v) => Ok(v.parse::<i64>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i64")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into i64")),
        }
    }
}

impl TryFrom<RoughValue> for f32 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into f32")),
            RoughValue::U8(v) => Ok(v.try_into()?),
            RoughValue::U16(v) => Ok(v.try_into()?),
            RoughValue::U32(_) => Err(anyhow!("failed to conver u32 into f32")),
            RoughValue::U64(_) => Err(anyhow!("failed to conver u64 into f32")),
            RoughValue::I8(v) => Ok(v.try_into()?),
            RoughValue::I16(v) => Ok(v.try_into()?),
            RoughValue::I32(_) => Err(anyhow!("failed to conver i32 into f32")),
            RoughValue::I64(_) => Err(anyhow!("failed to conver i64 into f32")),
            RoughValue::F32(v) => Ok(v),
            RoughValue::F64(v) => Ok(v as f32),
            RoughValue::String(v) => Ok(v.parse::<f32>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into f32")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into f32")),
        }
    }
}

impl TryFrom<RoughValue> for f64 {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(_) => Err(anyhow!("failed to convert bool into f64")),
            RoughValue::U8(v) => Ok(v.try_into()?),
            RoughValue::U16(v) => Ok(v.try_into()?),
            RoughValue::U32(v) => Ok(v.try_into()?),
            RoughValue::U64(_) => Err(anyhow!("failed to conver u64 into f64")),
            RoughValue::I8(v) => Ok(v.try_into()?),
            RoughValue::I16(v) => Ok(v.try_into()?),
            RoughValue::I32(v) => Ok(v.try_into()?),
            RoughValue::I64(_) => Err(anyhow!("failed to conver i64 into f64")),
            RoughValue::F32(v) => Ok(v.try_into()?),
            RoughValue::F64(v) => Ok(v),
            RoughValue::String(v) => Ok(v.parse::<f64>()?),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into f64")),
            RoughValue::Null => Err(anyhow!("failed to convert Null into f64")),
        }
    }
}

impl TryFrom<RoughValue> for String {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Bool(v) => Ok(v.to_string()),
            RoughValue::U8(v) => Ok(v.to_string()),
            RoughValue::U16(v) => Ok(v.to_string()),
            RoughValue::U32(v) => Ok(v.to_string()),
            RoughValue::U64(v) => Ok(v.to_string()),
            RoughValue::I8(v) => Ok(v.to_string()),
            RoughValue::I16(v) => Ok(v.to_string()),
            RoughValue::I32(v) => Ok(v.to_string()),
            RoughValue::I64(v) => Ok(v.to_string()),
            RoughValue::F32(v) => Ok(v.to_string()),
            RoughValue::F64(v) => Ok(v.to_string()),
            RoughValue::String(v) => Ok(v),
            RoughValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into String")),
            RoughValue::Null => Ok("Null".to_string()),
        }
    }
}

impl TryFrom<RoughValue> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: RoughValue) -> std::result::Result<Self, Self::Error> {
        match value {
            RoughValue::Blob(v) => Ok(v),
            _ => Err(anyhow!("convert to Vec<u8> failed")),
        }
    }
}

impl From<&RoughValue> for RoughValueType {
    fn from(value: &RoughValue) -> Self {
        match value {
            RoughValue::Bool(_) => RoughValueType::Bool,
            RoughValue::U8(_) => RoughValueType::U8,
            RoughValue::U16(_) => RoughValueType::U16,
            RoughValue::U32(_) => RoughValueType::U32,
            RoughValue::U64(_) => RoughValueType::U64,
            RoughValue::I8(_) => RoughValueType::I8,
            RoughValue::I16(_) => RoughValueType::I16,
            RoughValue::I32(_) => RoughValueType::I32,
            RoughValue::I64(_) => RoughValueType::I64,
            RoughValue::F32(_) => RoughValueType::F32,
            RoughValue::F64(_) => RoughValueType::F64,
            RoughValue::String(_) => RoughValueType::String,
            RoughValue::Blob(_) => RoughValueType::Blob,
            RoughValue::Null => RoughValueType::Null,
        }
    }
}

impl From<&mut RoughValue> for RoughValueType {
    fn from(value: &mut RoughValue) -> Self {
        match value {
            RoughValue::Bool(_) => RoughValueType::Bool,
            RoughValue::U8(_) => RoughValueType::U8,
            RoughValue::U16(_) => RoughValueType::U16,
            RoughValue::U32(_) => RoughValueType::U32,
            RoughValue::U64(_) => RoughValueType::U64,
            RoughValue::I8(_) => RoughValueType::I8,
            RoughValue::I16(_) => RoughValueType::I16,
            RoughValue::I32(_) => RoughValueType::I32,
            RoughValue::I64(_) => RoughValueType::I64,
            RoughValue::F32(_) => RoughValueType::F32,
            RoughValue::F64(_) => RoughValueType::F64,
            RoughValue::String(_) => RoughValueType::String,
            RoughValue::Blob(_) => RoughValueType::Blob,
            RoughValue::Null => RoughValueType::Null,
        }
    }
}

impl<'source> FromPyObject<'source> for RoughValue {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        if let Ok(v) = ob.extract::<bool>() {
            Ok(RoughValue::Bool(v))
        } else if let Ok(v) = ob.extract::<i64>() {
            Ok(RoughValue::I64(v))
        } else if let Ok(v) = ob.extract::<f64>() {
            Ok(RoughValue::F64(v))
        } else if let Ok(v) = ob.extract::<String>() {
            Ok(RoughValue::String(v))
        } else if let Ok(v) = ob.extract::<Vec<u8>>() {
            Ok(RoughValue::Blob(v))
        } else {
            Ok(RoughValue::Null)
        }
    }
}

impl IntoPy<PyObject> for RoughValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            RoughValue::Bool(v) => v.into_py(py),
            RoughValue::U8(v) => v.into_py(py),
            RoughValue::U16(v) => v.into_py(py),
            RoughValue::U32(v) => v.into_py(py),
            RoughValue::U64(v) => v.into_py(py),
            RoughValue::I8(v) => v.into_py(py),
            RoughValue::I16(v) => v.into_py(py),
            RoughValue::I32(v) => v.into_py(py),
            RoughValue::I64(v) => v.into_py(py),
            RoughValue::F32(v) => v.into_py(py),
            RoughValue::F64(v) => v.into_py(py),
            RoughValue::String(v) => v.into_py(py),
            RoughValue::Blob(v) => v.into_py(py),
            RoughValue::Null => py.None(),
        }
    }
}

// ================================================================================================
// RoughData
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxData", get_all)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoughData {
    pub(crate) columns: Vec<String>,
    pub(crate) types: Vec<RoughValueType>,
    pub(crate) data: Vec<Vec<RoughValue>>,
}

impl RoughData {
    pub fn new(
        columns: Vec<String>,
        types: Vec<RoughValueType>,
        data: Vec<Vec<RoughValue>>,
    ) -> Result<Self> {
        let c_l = columns.len();
        let t_l = types.len();
        if c_l != t_l {
            return Err(anyhow!(format!("columns len: {c_l}, types len: {t_l}")).into());
        }

        for (idx, row) in data.iter().enumerate() {
            let r_l = row.len();
            if c_l != r_l {
                return Err(anyhow!(format!("columns len: {c_l}, row[{idx}] len: {r_l}")).into());
            }
        }

        Ok(Self {
            columns,
            types,
            data,
        })
    }

    pub fn type_coercion(&mut self) -> Result<()> {
        let types = &self.types;

        for row in self.data.iter_mut() {
            for (idx, e) in row.iter_mut().enumerate() {
                if matches!(e, RoughValue::Null) {
                    continue;
                }
                match &types[idx] {
                    RoughValueType::Bool => {
                        *e = RoughValue::Bool(bool::try_from(e.clone())?);
                    }
                    RoughValueType::U8 => {
                        *e = RoughValue::U8(u8::try_from(e.clone())?);
                    }
                    RoughValueType::U16 => {
                        *e = RoughValue::U16(u16::try_from(e.clone())?);
                    }
                    RoughValueType::U32 => {
                        *e = RoughValue::U32(u32::try_from(e.clone())?);
                    }
                    RoughValueType::U64 => {
                        *e = RoughValue::U64(u64::try_from(e.clone())?);
                    }
                    RoughValueType::I8 => {
                        *e = RoughValue::I8(i8::try_from(e.clone())?);
                    }
                    RoughValueType::I16 => {
                        *e = RoughValue::I16(i16::try_from(e.clone())?);
                    }
                    RoughValueType::I32 => {
                        *e = RoughValue::I32(i32::try_from(e.clone())?);
                    }
                    RoughValueType::I64 => {
                        *e = RoughValue::I64(i64::try_from(e.clone())?);
                    }
                    RoughValueType::F32 => {
                        *e = RoughValue::F32(f32::try_from(e.clone())?);
                    }
                    RoughValueType::F64 => {
                        *e = RoughValue::F64(f64::try_from(e.clone())?);
                    }
                    RoughValueType::String => {
                        *e = RoughValue::String(String::try_from(e.clone())?);
                    }
                    RoughValueType::Blob => {
                        *e = RoughValue::Blob(Vec::<u8>::try_from(e.clone())?);
                    }
                    RoughValueType::Null => {
                        // Do nothing
                    }
                }
            }
        }

        Ok(())
    }
}

#[pymethods]
impl RoughData {
    #[new]
    fn py_new(
        columns: Vec<String>,
        types: Vec<RoughValueType>,
        data: Vec<Vec<RoughValue>>,
    ) -> PyResult<Self> {
        Ok(RoughData::new(columns, types, data)?)
    }

    #[pyo3(name = "type_coercion")]
    fn py_type_coercion(&mut self) -> PyResult<()> {
        Ok(self.type_coercion()?)
    }

    #[pyo3(name = "to_json", text_signature = "($self)")]
    fn py_to_json(&self) -> Option<String> {
        serde_json::to_string(&self).ok()
    }

    #[pyo3(name = "to_json_pretty", text_signature = "($self)")]
    fn py_to_json_pretty(&self) -> Option<String> {
        serde_json::to_string_pretty(&self).ok()
    }
}

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
                _ => get_value!(String, String, r, idx),
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

    pub fn columns(&self) -> Option<Vec<String>> {
        self.cache
            .as_ref()
            .map(|v| v.into_iter().map(|(e, _)| e.clone()).collect())
    }

    pub fn types(&self) -> Option<Vec<RoughValueType>> {
        self.cache
            .as_ref()
            .map(|v| v.into_iter().map(|(_, t)| t.clone()).collect())
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

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_rough {
    use super::*;

    #[test]
    fn rough_value_print() {
        let foo = RoughValue::F64(123.456);
        println!("{:?}", serde_json::to_string(&foo));

        let foo = RoughValue::Null;
        println!("{:?}", serde_json::to_string(&foo));
    }
}
