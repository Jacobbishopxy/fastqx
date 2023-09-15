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

// ================================================================================================
// RoughValue <-> Rust types
// ================================================================================================

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
            RoughValue::Blob(v) => Ok(String::from_utf8(v)?),
            RoughValue::Null => Ok("".to_string()),
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

// ================================================================================================
// Conversion
// ================================================================================================

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
// ToString
// ================================================================================================

impl ToString for RoughValue {
    fn to_string(&self) -> String {
        match self {
            RoughValue::Bool(v) => v.to_string(),
            RoughValue::U8(v) => v.to_string(),
            RoughValue::U16(v) => v.to_string(),
            RoughValue::U32(v) => v.to_string(),
            RoughValue::U64(v) => v.to_string(),
            RoughValue::I8(v) => v.to_string(),
            RoughValue::I16(v) => v.to_string(),
            RoughValue::I32(v) => v.to_string(),
            RoughValue::I64(v) => v.to_string(),
            RoughValue::F32(v) => v.to_string(),
            RoughValue::F64(v) => v.to_string(),
            RoughValue::String(v) => v.to_string(),
            RoughValue::Blob(v) => String::from_utf8(v.to_vec())
                .unwrap_or("Invalid conversion from Vec<u8>".to_string()),
            RoughValue::Null => "".to_string(),
        }
    }
}

pub fn try_from_str_with_type_hints(s: &str, type_hint: &RoughValueType) -> Result<RoughValue> {
    let res = match type_hint {
        RoughValueType::Bool => RoughValue::Bool(s.parse::<bool>()?),
        RoughValueType::U8 => RoughValue::U8(s.parse::<u8>()?),
        RoughValueType::U16 => RoughValue::U16(s.parse::<u16>()?),
        RoughValueType::U32 => RoughValue::U32(s.parse::<u32>()?),
        RoughValueType::U64 => RoughValue::U64(s.parse::<u64>()?),
        RoughValueType::I8 => RoughValue::I8(s.parse::<i8>()?),
        RoughValueType::I16 => RoughValue::I16(s.parse::<i16>()?),
        RoughValueType::I32 => RoughValue::I32(s.parse::<i32>()?),
        RoughValueType::I64 => RoughValue::I64(s.parse::<i64>()?),
        RoughValueType::F32 => RoughValue::F32(s.parse::<f32>()?),
        RoughValueType::F64 => RoughValue::F64(s.parse::<f64>()?),
        RoughValueType::String => RoughValue::String(s.into()),
        RoughValueType::Blob => RoughValue::Blob(s.as_bytes().to_vec()),
        RoughValueType::Null => RoughValue::Null,
    };

    Ok(res)
}
