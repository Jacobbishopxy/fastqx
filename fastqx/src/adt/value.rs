//! file: value.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:46:03 Wednesday
//! brief:

use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ================================================================================================
// RoughValueType & RoughValue
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxValueType")]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FastqxValueType {
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

#[pymethods]
impl FastqxValueType {
    pub fn __repr__(&self) -> &'static str {
        match self {
            FastqxValueType::Bool => "FqxValueType::Bool",
            FastqxValueType::U8 => "FqxValueType::U8",
            FastqxValueType::U16 => "FqxValueType::U16",
            FastqxValueType::U32 => "FqxValueType::U32",
            FastqxValueType::U64 => "FqxValueType::U64",
            FastqxValueType::I8 => "FqxValueType::I8",
            FastqxValueType::I16 => "FqxValueType::I16",
            FastqxValueType::I32 => "FqxValueType::I32",
            FastqxValueType::I64 => "FqxValueType::I64",
            FastqxValueType::F32 => "FqxValueType::F32",
            FastqxValueType::F64 => "FqxValueType::F64",
            FastqxValueType::String => "FqxValueType::String",
            FastqxValueType::Blob => "FqxValueType::Blob",
            FastqxValueType::Null => "FqxValueType::Null",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FastqxValue {
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

// ================================================================================================
// RoughValue <-> Rust types
// ================================================================================================

impl TryFrom<FastqxValue> for bool {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(v) => Ok(v),
            FastqxValue::U8(v) => Ok(v == 0),
            FastqxValue::U16(v) => Ok(v == 0),
            FastqxValue::U32(v) => Ok(v == 0),
            FastqxValue::U64(v) => Ok(v == 0),
            FastqxValue::I8(v) => Ok(v == 0),
            FastqxValue::I16(v) => Ok(v == 0),
            FastqxValue::I32(v) => Ok(v == 0),
            FastqxValue::I64(v) => Ok(v == 0),
            FastqxValue::F32(v) => Ok(v == 0.0),
            FastqxValue::F64(v) => Ok(v == 0.0),
            FastqxValue::String(v) => Ok(&v != "0"),
            FastqxValue::Blob(v) => Ok(!v.is_empty()),
            FastqxValue::Null => Ok(false),
        }
    }
}

impl TryFrom<FastqxValue> for u8 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u8")),
            FastqxValue::U8(v) => Ok(v),
            FastqxValue::U16(v) => Ok(v.try_into()?),
            FastqxValue::U32(v) => Ok(v.try_into()?),
            FastqxValue::U64(v) => Ok(v.try_into()?),
            FastqxValue::I8(v) => Ok(v.try_into()?),
            FastqxValue::I16(v) => Ok(v.try_into()?),
            FastqxValue::I32(v) => Ok(v.try_into()?),
            FastqxValue::I64(v) => Ok(v.try_into()?),
            FastqxValue::F32(_) => Err(anyhow!("failed to convert f32 into u8")),
            FastqxValue::F64(_) => Err(anyhow!("failed to convert f64 into u8")),
            FastqxValue::String(v) => Ok(v.parse::<u8>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u8")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into u8")),
        }
    }
}

impl TryFrom<FastqxValue> for u16 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u16")),
            FastqxValue::U8(v) => Ok(v.try_into()?),
            FastqxValue::U16(v) => Ok(v),
            FastqxValue::U32(v) => Ok(v.try_into()?),
            FastqxValue::U64(v) => Ok(v.try_into()?),
            FastqxValue::I8(v) => Ok(v.try_into()?),
            FastqxValue::I16(v) => Ok(v.try_into()?),
            FastqxValue::I32(v) => Ok(v.try_into()?),
            FastqxValue::I64(v) => Ok(v.try_into()?),
            FastqxValue::F32(_) => Err(anyhow!("failed to convert f32 into u16")),
            FastqxValue::F64(_) => Err(anyhow!("failed to convert f64 into u16")),
            FastqxValue::String(v) => Ok(v.parse::<u16>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u16")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into u16")),
        }
    }
}

impl TryFrom<FastqxValue> for u32 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u32")),
            FastqxValue::U8(v) => Ok(v.try_into()?),
            FastqxValue::U16(v) => Ok(v.try_into()?),
            FastqxValue::U32(v) => Ok(v),
            FastqxValue::U64(v) => Ok(v.try_into()?),
            FastqxValue::I8(v) => Ok(v.try_into()?),
            FastqxValue::I16(v) => Ok(v.try_into()?),
            FastqxValue::I32(v) => Ok(v.try_into()?),
            FastqxValue::I64(v) => Ok(v.try_into()?),
            FastqxValue::F32(_) => Err(anyhow!("failed to convert f32 into u32")),
            FastqxValue::F64(_) => Err(anyhow!("failed to convert f64 into u32")),
            FastqxValue::String(v) => Ok(v.parse::<u32>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u32")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into u32")),
        }
    }
}

impl TryFrom<FastqxValue> for u64 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u64")),
            FastqxValue::U8(v) => Ok(v.try_into()?),
            FastqxValue::U16(v) => Ok(v.try_into()?),
            FastqxValue::U32(v) => Ok(v.try_into()?),
            FastqxValue::U64(v) => Ok(v),
            FastqxValue::I8(v) => Ok(v.try_into()?),
            FastqxValue::I16(v) => Ok(v.try_into()?),
            FastqxValue::I32(v) => Ok(v.try_into()?),
            FastqxValue::I64(v) => Ok(v.try_into()?),
            FastqxValue::F32(_) => Err(anyhow!("failed to convert f32 into u64")),
            FastqxValue::F64(_) => Err(anyhow!("failed to convert f64 into u64")),
            FastqxValue::String(v) => Ok(v.parse::<u64>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u64")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into u64")),
        }
    }
}

impl TryFrom<FastqxValue> for i8 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into i8")),
            FastqxValue::U8(v) => Ok(v.try_into()?),
            FastqxValue::U16(v) => Ok(v.try_into()?),
            FastqxValue::U32(v) => Ok(v.try_into()?),
            FastqxValue::U64(v) => Ok(v.try_into()?),
            FastqxValue::I8(v) => Ok(v),
            FastqxValue::I16(v) => Ok(v.try_into()?),
            FastqxValue::I32(v) => Ok(v.try_into()?),
            FastqxValue::I64(v) => Ok(v.try_into()?),
            FastqxValue::F32(_) => Err(anyhow!("failed to convert f32 into i8")),
            FastqxValue::F64(_) => Err(anyhow!("failed to convert f64 into i8")),
            FastqxValue::String(v) => Ok(v.parse::<i8>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i8")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into i8")),
        }
    }
}

impl TryFrom<FastqxValue> for i16 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into i16")),
            FastqxValue::U8(v) => Ok(v.try_into()?),
            FastqxValue::U16(v) => Ok(v.try_into()?),
            FastqxValue::U32(v) => Ok(v.try_into()?),
            FastqxValue::U64(v) => Ok(v.try_into()?),
            FastqxValue::I8(v) => Ok(v.try_into()?),
            FastqxValue::I16(v) => Ok(v),
            FastqxValue::I32(v) => Ok(v.try_into()?),
            FastqxValue::I64(v) => Ok(v.try_into()?),
            FastqxValue::F32(_) => Err(anyhow!("failed to convert f32 into i16")),
            FastqxValue::F64(_) => Err(anyhow!("failed to convert f64 into i16")),
            FastqxValue::String(v) => Ok(v.parse::<i16>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i16")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into i16")),
        }
    }
}

impl TryFrom<FastqxValue> for i32 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into i32")),
            FastqxValue::U8(v) => Ok(v.try_into()?),
            FastqxValue::U16(v) => Ok(v.try_into()?),
            FastqxValue::U32(v) => Ok(v.try_into()?),
            FastqxValue::U64(v) => Ok(v.try_into()?),
            FastqxValue::I8(v) => Ok(v.try_into()?),
            FastqxValue::I16(v) => Ok(v.try_into()?),
            FastqxValue::I32(v) => Ok(v),
            FastqxValue::I64(v) => Ok(v.try_into()?),
            FastqxValue::F32(_) => Err(anyhow!("failed to convert f32 into i32")),
            FastqxValue::F64(_) => Err(anyhow!("failed to convert f64 into i32")),
            FastqxValue::String(v) => Ok(v.parse::<i32>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i32")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into i32")),
        }
    }
}

impl TryFrom<FastqxValue> for i64 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u64")),
            FastqxValue::U8(v) => Ok(v.try_into()?),
            FastqxValue::U16(v) => Ok(v.try_into()?),
            FastqxValue::U32(v) => Ok(v.try_into()?),
            FastqxValue::U64(v) => Ok(v.try_into()?),
            FastqxValue::I8(v) => Ok(v.try_into()?),
            FastqxValue::I16(v) => Ok(v.try_into()?),
            FastqxValue::I32(v) => Ok(v.try_into()?),
            FastqxValue::I64(v) => Ok(v),
            FastqxValue::F32(_) => Err(anyhow!("failed to convert f32 into i64")),
            FastqxValue::F64(_) => Err(anyhow!("failed to convert f64 into i64")),
            FastqxValue::String(v) => Ok(v.parse::<i64>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i64")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into i64")),
        }
    }
}

impl TryFrom<FastqxValue> for f32 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into f32")),
            FastqxValue::U8(v) => Ok(v.try_into()?),
            FastqxValue::U16(v) => Ok(v.try_into()?),
            FastqxValue::U32(_) => Err(anyhow!("failed to conver u32 into f32")),
            FastqxValue::U64(_) => Err(anyhow!("failed to conver u64 into f32")),
            FastqxValue::I8(v) => Ok(v.try_into()?),
            FastqxValue::I16(v) => Ok(v.try_into()?),
            FastqxValue::I32(_) => Err(anyhow!("failed to conver i32 into f32")),
            FastqxValue::I64(_) => Err(anyhow!("failed to conver i64 into f32")),
            FastqxValue::F32(v) => Ok(v),
            FastqxValue::F64(v) => Ok(v as f32),
            FastqxValue::String(v) => Ok(v.parse::<f32>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into f32")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into f32")),
        }
    }
}

impl TryFrom<FastqxValue> for f64 {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(_) => Err(anyhow!("failed to convert bool into f64")),
            FastqxValue::U8(v) => Ok(v.try_into()?),
            FastqxValue::U16(v) => Ok(v.try_into()?),
            FastqxValue::U32(v) => Ok(v.try_into()?),
            FastqxValue::U64(_) => Err(anyhow!("failed to conver u64 into f64")),
            FastqxValue::I8(v) => Ok(v.try_into()?),
            FastqxValue::I16(v) => Ok(v.try_into()?),
            FastqxValue::I32(v) => Ok(v.try_into()?),
            FastqxValue::I64(_) => Err(anyhow!("failed to conver i64 into f64")),
            FastqxValue::F32(v) => Ok(v.try_into()?),
            FastqxValue::F64(v) => Ok(v),
            FastqxValue::String(v) => Ok(v.parse::<f64>()?),
            FastqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into f64")),
            FastqxValue::Null => Err(anyhow!("failed to convert Null into f64")),
        }
    }
}

impl TryFrom<FastqxValue> for String {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Bool(v) => Ok(v.to_string()),
            FastqxValue::U8(v) => Ok(v.to_string()),
            FastqxValue::U16(v) => Ok(v.to_string()),
            FastqxValue::U32(v) => Ok(v.to_string()),
            FastqxValue::U64(v) => Ok(v.to_string()),
            FastqxValue::I8(v) => Ok(v.to_string()),
            FastqxValue::I16(v) => Ok(v.to_string()),
            FastqxValue::I32(v) => Ok(v.to_string()),
            FastqxValue::I64(v) => Ok(v.to_string()),
            FastqxValue::F32(v) => Ok(v.to_string()),
            FastqxValue::F64(v) => Ok(v.to_string()),
            FastqxValue::String(v) => Ok(v),
            FastqxValue::Blob(v) => Ok(String::from_utf8(v)?),
            FastqxValue::Null => Ok("".to_string()),
        }
    }
}

impl TryFrom<FastqxValue> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: FastqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FastqxValue::Blob(v) => Ok(v),
            _ => Err(anyhow!("convert to Vec<u8> failed")),
        }
    }
}

// ================================================================================================
// Conversion
// ================================================================================================

impl From<&FastqxValue> for FastqxValueType {
    fn from(value: &FastqxValue) -> Self {
        match value {
            FastqxValue::Bool(_) => FastqxValueType::Bool,
            FastqxValue::U8(_) => FastqxValueType::U8,
            FastqxValue::U16(_) => FastqxValueType::U16,
            FastqxValue::U32(_) => FastqxValueType::U32,
            FastqxValue::U64(_) => FastqxValueType::U64,
            FastqxValue::I8(_) => FastqxValueType::I8,
            FastqxValue::I16(_) => FastqxValueType::I16,
            FastqxValue::I32(_) => FastqxValueType::I32,
            FastqxValue::I64(_) => FastqxValueType::I64,
            FastqxValue::F32(_) => FastqxValueType::F32,
            FastqxValue::F64(_) => FastqxValueType::F64,
            FastqxValue::String(_) => FastqxValueType::String,
            FastqxValue::Blob(_) => FastqxValueType::Blob,
            FastqxValue::Null => FastqxValueType::Null,
        }
    }
}

impl From<&mut FastqxValue> for FastqxValueType {
    fn from(value: &mut FastqxValue) -> Self {
        match value {
            FastqxValue::Bool(_) => FastqxValueType::Bool,
            FastqxValue::U8(_) => FastqxValueType::U8,
            FastqxValue::U16(_) => FastqxValueType::U16,
            FastqxValue::U32(_) => FastqxValueType::U32,
            FastqxValue::U64(_) => FastqxValueType::U64,
            FastqxValue::I8(_) => FastqxValueType::I8,
            FastqxValue::I16(_) => FastqxValueType::I16,
            FastqxValue::I32(_) => FastqxValueType::I32,
            FastqxValue::I64(_) => FastqxValueType::I64,
            FastqxValue::F32(_) => FastqxValueType::F32,
            FastqxValue::F64(_) => FastqxValueType::F64,
            FastqxValue::String(_) => FastqxValueType::String,
            FastqxValue::Blob(_) => FastqxValueType::Blob,
            FastqxValue::Null => FastqxValueType::Null,
        }
    }
}

impl<'source> FromPyObject<'source> for FastqxValue {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        if let Ok(v) = ob.extract::<bool>() {
            Ok(FastqxValue::Bool(v))
        } else if let Ok(v) = ob.extract::<i64>() {
            Ok(FastqxValue::I64(v))
        } else if let Ok(v) = ob.extract::<f64>() {
            Ok(FastqxValue::F64(v))
        } else if let Ok(v) = ob.extract::<String>() {
            Ok(FastqxValue::String(v))
        } else if let Ok(v) = ob.extract::<Vec<u8>>() {
            Ok(FastqxValue::Blob(v))
        } else {
            Ok(FastqxValue::Null)
        }
    }
}

impl IntoPy<PyObject> for FastqxValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            FastqxValue::Bool(v) => v.into_py(py),
            FastqxValue::U8(v) => v.into_py(py),
            FastqxValue::U16(v) => v.into_py(py),
            FastqxValue::U32(v) => v.into_py(py),
            FastqxValue::U64(v) => v.into_py(py),
            FastqxValue::I8(v) => v.into_py(py),
            FastqxValue::I16(v) => v.into_py(py),
            FastqxValue::I32(v) => v.into_py(py),
            FastqxValue::I64(v) => v.into_py(py),
            FastqxValue::F32(v) => v.into_py(py),
            FastqxValue::F64(v) => v.into_py(py),
            FastqxValue::String(v) => v.into_py(py),
            FastqxValue::Blob(v) => v.into_py(py),
            FastqxValue::Null => py.None(),
        }
    }
}

// ================================================================================================
// ToString
// ================================================================================================

impl ToString for FastqxValue {
    fn to_string(&self) -> String {
        match self {
            FastqxValue::Bool(v) => v.to_string(),
            FastqxValue::U8(v) => v.to_string(),
            FastqxValue::U16(v) => v.to_string(),
            FastqxValue::U32(v) => v.to_string(),
            FastqxValue::U64(v) => v.to_string(),
            FastqxValue::I8(v) => v.to_string(),
            FastqxValue::I16(v) => v.to_string(),
            FastqxValue::I32(v) => v.to_string(),
            FastqxValue::I64(v) => v.to_string(),
            FastqxValue::F32(v) => v.to_string(),
            FastqxValue::F64(v) => v.to_string(),
            FastqxValue::String(v) => v.to_string(),
            FastqxValue::Blob(v) => String::from_utf8(v.to_vec())
                .unwrap_or("Invalid conversion from Vec<u8>".to_string()),
            FastqxValue::Null => "".to_string(),
        }
    }
}

pub fn try_from_str_with_type_hints(s: &str, type_hint: &FastqxValueType) -> Result<FastqxValue> {
    let res = match type_hint {
        FastqxValueType::Bool => FastqxValue::Bool(s.parse::<bool>()?),
        FastqxValueType::U8 => FastqxValue::U8(s.parse::<u8>()?),
        FastqxValueType::U16 => FastqxValue::U16(s.parse::<u16>()?),
        FastqxValueType::U32 => FastqxValue::U32(s.parse::<u32>()?),
        FastqxValueType::U64 => FastqxValue::U64(s.parse::<u64>()?),
        FastqxValueType::I8 => FastqxValue::I8(s.parse::<i8>()?),
        FastqxValueType::I16 => FastqxValue::I16(s.parse::<i16>()?),
        FastqxValueType::I32 => FastqxValue::I32(s.parse::<i32>()?),
        FastqxValueType::I64 => FastqxValue::I64(s.parse::<i64>()?),
        FastqxValueType::F32 => FastqxValue::F32(s.parse::<f32>()?),
        FastqxValueType::F64 => FastqxValue::F64(s.parse::<f64>()?),
        FastqxValueType::String => FastqxValue::String(s.into()),
        FastqxValueType::Blob => FastqxValue::Blob(s.as_bytes().to_vec()),
        FastqxValueType::Null => FastqxValue::Null,
    };

    Ok(res)
}
