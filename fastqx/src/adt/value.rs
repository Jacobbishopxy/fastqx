//! file: value.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:46:03 Wednesday
//! brief:

use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ================================================================================================
// FastqxValueType & FastqxValue
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxValueType")]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FqxValueType {
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
impl FqxValueType {
    pub fn __repr__(&self) -> &'static str {
        match self {
            FqxValueType::Bool => "FqxValueType::Bool",
            FqxValueType::U8 => "FqxValueType::U8",
            FqxValueType::U16 => "FqxValueType::U16",
            FqxValueType::U32 => "FqxValueType::U32",
            FqxValueType::U64 => "FqxValueType::U64",
            FqxValueType::I8 => "FqxValueType::I8",
            FqxValueType::I16 => "FqxValueType::I16",
            FqxValueType::I32 => "FqxValueType::I32",
            FqxValueType::I64 => "FqxValueType::I64",
            FqxValueType::F32 => "FqxValueType::F32",
            FqxValueType::F64 => "FqxValueType::F64",
            FqxValueType::String => "FqxValueType::String",
            FqxValueType::Blob => "FqxValueType::Blob",
            FqxValueType::Null => "FqxValueType::Null",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FqxValue {
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
// FastqxValue <-> Rust types
// ================================================================================================

impl TryFrom<FqxValue> for bool {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(v) => Ok(v),
            FqxValue::U8(v) => Ok(v == 0),
            FqxValue::U16(v) => Ok(v == 0),
            FqxValue::U32(v) => Ok(v == 0),
            FqxValue::U64(v) => Ok(v == 0),
            FqxValue::I8(v) => Ok(v == 0),
            FqxValue::I16(v) => Ok(v == 0),
            FqxValue::I32(v) => Ok(v == 0),
            FqxValue::I64(v) => Ok(v == 0),
            FqxValue::F32(v) => Ok(v == 0.0),
            FqxValue::F64(v) => Ok(v == 0.0),
            FqxValue::String(v) => Ok(&v != "0"),
            FqxValue::Blob(v) => Ok(!v.is_empty()),
            FqxValue::Null => Ok(false),
        }
    }
}

impl TryFrom<FqxValue> for u8 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u8")),
            FqxValue::U8(v) => Ok(v),
            FqxValue::U16(v) => Ok(v.try_into()?),
            FqxValue::U32(v) => Ok(v.try_into()?),
            FqxValue::U64(v) => Ok(v.try_into()?),
            FqxValue::I8(v) => Ok(v.try_into()?),
            FqxValue::I16(v) => Ok(v.try_into()?),
            FqxValue::I32(v) => Ok(v.try_into()?),
            FqxValue::I64(v) => Ok(v.try_into()?),
            FqxValue::F32(_) => Err(anyhow!("failed to convert f32 into u8")),
            FqxValue::F64(_) => Err(anyhow!("failed to convert f64 into u8")),
            FqxValue::String(v) => Ok(v.parse::<u8>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u8")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into u8")),
        }
    }
}

impl TryFrom<FqxValue> for u16 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u16")),
            FqxValue::U8(v) => Ok(v.try_into()?),
            FqxValue::U16(v) => Ok(v),
            FqxValue::U32(v) => Ok(v.try_into()?),
            FqxValue::U64(v) => Ok(v.try_into()?),
            FqxValue::I8(v) => Ok(v.try_into()?),
            FqxValue::I16(v) => Ok(v.try_into()?),
            FqxValue::I32(v) => Ok(v.try_into()?),
            FqxValue::I64(v) => Ok(v.try_into()?),
            FqxValue::F32(_) => Err(anyhow!("failed to convert f32 into u16")),
            FqxValue::F64(_) => Err(anyhow!("failed to convert f64 into u16")),
            FqxValue::String(v) => Ok(v.parse::<u16>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u16")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into u16")),
        }
    }
}

impl TryFrom<FqxValue> for u32 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u32")),
            FqxValue::U8(v) => Ok(v.try_into()?),
            FqxValue::U16(v) => Ok(v.try_into()?),
            FqxValue::U32(v) => Ok(v),
            FqxValue::U64(v) => Ok(v.try_into()?),
            FqxValue::I8(v) => Ok(v.try_into()?),
            FqxValue::I16(v) => Ok(v.try_into()?),
            FqxValue::I32(v) => Ok(v.try_into()?),
            FqxValue::I64(v) => Ok(v.try_into()?),
            FqxValue::F32(_) => Err(anyhow!("failed to convert f32 into u32")),
            FqxValue::F64(_) => Err(anyhow!("failed to convert f64 into u32")),
            FqxValue::String(v) => Ok(v.parse::<u32>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u32")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into u32")),
        }
    }
}

impl TryFrom<FqxValue> for u64 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u64")),
            FqxValue::U8(v) => Ok(v.try_into()?),
            FqxValue::U16(v) => Ok(v.try_into()?),
            FqxValue::U32(v) => Ok(v.try_into()?),
            FqxValue::U64(v) => Ok(v),
            FqxValue::I8(v) => Ok(v.try_into()?),
            FqxValue::I16(v) => Ok(v.try_into()?),
            FqxValue::I32(v) => Ok(v.try_into()?),
            FqxValue::I64(v) => Ok(v.try_into()?),
            FqxValue::F32(_) => Err(anyhow!("failed to convert f32 into u64")),
            FqxValue::F64(_) => Err(anyhow!("failed to convert f64 into u64")),
            FqxValue::String(v) => Ok(v.parse::<u64>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into u64")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into u64")),
        }
    }
}

impl TryFrom<FqxValue> for i8 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into i8")),
            FqxValue::U8(v) => Ok(v.try_into()?),
            FqxValue::U16(v) => Ok(v.try_into()?),
            FqxValue::U32(v) => Ok(v.try_into()?),
            FqxValue::U64(v) => Ok(v.try_into()?),
            FqxValue::I8(v) => Ok(v),
            FqxValue::I16(v) => Ok(v.try_into()?),
            FqxValue::I32(v) => Ok(v.try_into()?),
            FqxValue::I64(v) => Ok(v.try_into()?),
            FqxValue::F32(_) => Err(anyhow!("failed to convert f32 into i8")),
            FqxValue::F64(_) => Err(anyhow!("failed to convert f64 into i8")),
            FqxValue::String(v) => Ok(v.parse::<i8>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i8")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into i8")),
        }
    }
}

impl TryFrom<FqxValue> for i16 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into i16")),
            FqxValue::U8(v) => Ok(v.try_into()?),
            FqxValue::U16(v) => Ok(v.try_into()?),
            FqxValue::U32(v) => Ok(v.try_into()?),
            FqxValue::U64(v) => Ok(v.try_into()?),
            FqxValue::I8(v) => Ok(v.try_into()?),
            FqxValue::I16(v) => Ok(v),
            FqxValue::I32(v) => Ok(v.try_into()?),
            FqxValue::I64(v) => Ok(v.try_into()?),
            FqxValue::F32(_) => Err(anyhow!("failed to convert f32 into i16")),
            FqxValue::F64(_) => Err(anyhow!("failed to convert f64 into i16")),
            FqxValue::String(v) => Ok(v.parse::<i16>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i16")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into i16")),
        }
    }
}

impl TryFrom<FqxValue> for i32 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into i32")),
            FqxValue::U8(v) => Ok(v.try_into()?),
            FqxValue::U16(v) => Ok(v.try_into()?),
            FqxValue::U32(v) => Ok(v.try_into()?),
            FqxValue::U64(v) => Ok(v.try_into()?),
            FqxValue::I8(v) => Ok(v.try_into()?),
            FqxValue::I16(v) => Ok(v.try_into()?),
            FqxValue::I32(v) => Ok(v),
            FqxValue::I64(v) => Ok(v.try_into()?),
            FqxValue::F32(_) => Err(anyhow!("failed to convert f32 into i32")),
            FqxValue::F64(_) => Err(anyhow!("failed to convert f64 into i32")),
            FqxValue::String(v) => Ok(v.parse::<i32>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i32")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into i32")),
        }
    }
}

impl TryFrom<FqxValue> for i64 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into u64")),
            FqxValue::U8(v) => Ok(v.try_into()?),
            FqxValue::U16(v) => Ok(v.try_into()?),
            FqxValue::U32(v) => Ok(v.try_into()?),
            FqxValue::U64(v) => Ok(v.try_into()?),
            FqxValue::I8(v) => Ok(v.try_into()?),
            FqxValue::I16(v) => Ok(v.try_into()?),
            FqxValue::I32(v) => Ok(v.try_into()?),
            FqxValue::I64(v) => Ok(v),
            FqxValue::F32(_) => Err(anyhow!("failed to convert f32 into i64")),
            FqxValue::F64(_) => Err(anyhow!("failed to convert f64 into i64")),
            FqxValue::String(v) => Ok(v.parse::<i64>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into i64")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into i64")),
        }
    }
}

impl TryFrom<FqxValue> for f32 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into f32")),
            FqxValue::U8(v) => Ok(v.try_into()?),
            FqxValue::U16(v) => Ok(v.try_into()?),
            FqxValue::U32(_) => Err(anyhow!("failed to conver u32 into f32")),
            FqxValue::U64(_) => Err(anyhow!("failed to conver u64 into f32")),
            FqxValue::I8(v) => Ok(v.try_into()?),
            FqxValue::I16(v) => Ok(v.try_into()?),
            FqxValue::I32(_) => Err(anyhow!("failed to conver i32 into f32")),
            FqxValue::I64(_) => Err(anyhow!("failed to conver i64 into f32")),
            FqxValue::F32(v) => Ok(v),
            FqxValue::F64(v) => Ok(v as f32),
            FqxValue::String(v) => Ok(v.parse::<f32>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into f32")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into f32")),
        }
    }
}

impl TryFrom<FqxValue> for f64 {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into f64")),
            FqxValue::U8(v) => Ok(v.try_into()?),
            FqxValue::U16(v) => Ok(v.try_into()?),
            FqxValue::U32(v) => Ok(v.try_into()?),
            FqxValue::U64(_) => Err(anyhow!("failed to conver u64 into f64")),
            FqxValue::I8(v) => Ok(v.try_into()?),
            FqxValue::I16(v) => Ok(v.try_into()?),
            FqxValue::I32(v) => Ok(v.try_into()?),
            FqxValue::I64(_) => Err(anyhow!("failed to conver i64 into f64")),
            FqxValue::F32(v) => Ok(v.try_into()?),
            FqxValue::F64(v) => Ok(v),
            FqxValue::String(v) => Ok(v.parse::<f64>()?),
            FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into f64")),
            FqxValue::Null => Err(anyhow!("failed to convert Null into f64")),
        }
    }
}

impl TryFrom<FqxValue> for String {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Bool(v) => Ok(v.to_string()),
            FqxValue::U8(v) => Ok(v.to_string()),
            FqxValue::U16(v) => Ok(v.to_string()),
            FqxValue::U32(v) => Ok(v.to_string()),
            FqxValue::U64(v) => Ok(v.to_string()),
            FqxValue::I8(v) => Ok(v.to_string()),
            FqxValue::I16(v) => Ok(v.to_string()),
            FqxValue::I32(v) => Ok(v.to_string()),
            FqxValue::I64(v) => Ok(v.to_string()),
            FqxValue::F32(v) => Ok(v.to_string()),
            FqxValue::F64(v) => Ok(v.to_string()),
            FqxValue::String(v) => Ok(v),
            FqxValue::Blob(v) => Ok(String::from_utf8(v)?),
            FqxValue::Null => Ok("".to_string()),
        }
    }
}

impl TryFrom<FqxValue> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Blob(v) => Ok(v),
            _ => Err(anyhow!("convert to Vec<u8> failed")),
        }
    }
}

// ================================================================================================
// Conversion
// ================================================================================================

impl From<&FqxValue> for FqxValueType {
    fn from(value: &FqxValue) -> Self {
        match value {
            FqxValue::Bool(_) => FqxValueType::Bool,
            FqxValue::U8(_) => FqxValueType::U8,
            FqxValue::U16(_) => FqxValueType::U16,
            FqxValue::U32(_) => FqxValueType::U32,
            FqxValue::U64(_) => FqxValueType::U64,
            FqxValue::I8(_) => FqxValueType::I8,
            FqxValue::I16(_) => FqxValueType::I16,
            FqxValue::I32(_) => FqxValueType::I32,
            FqxValue::I64(_) => FqxValueType::I64,
            FqxValue::F32(_) => FqxValueType::F32,
            FqxValue::F64(_) => FqxValueType::F64,
            FqxValue::String(_) => FqxValueType::String,
            FqxValue::Blob(_) => FqxValueType::Blob,
            FqxValue::Null => FqxValueType::Null,
        }
    }
}

impl From<&mut FqxValue> for FqxValueType {
    fn from(value: &mut FqxValue) -> Self {
        match value {
            FqxValue::Bool(_) => FqxValueType::Bool,
            FqxValue::U8(_) => FqxValueType::U8,
            FqxValue::U16(_) => FqxValueType::U16,
            FqxValue::U32(_) => FqxValueType::U32,
            FqxValue::U64(_) => FqxValueType::U64,
            FqxValue::I8(_) => FqxValueType::I8,
            FqxValue::I16(_) => FqxValueType::I16,
            FqxValue::I32(_) => FqxValueType::I32,
            FqxValue::I64(_) => FqxValueType::I64,
            FqxValue::F32(_) => FqxValueType::F32,
            FqxValue::F64(_) => FqxValueType::F64,
            FqxValue::String(_) => FqxValueType::String,
            FqxValue::Blob(_) => FqxValueType::Blob,
            FqxValue::Null => FqxValueType::Null,
        }
    }
}

impl<'source> FromPyObject<'source> for FqxValue {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        if let Ok(v) = ob.extract::<bool>() {
            Ok(FqxValue::Bool(v))
        } else if let Ok(v) = ob.extract::<i64>() {
            Ok(FqxValue::I64(v))
        } else if let Ok(v) = ob.extract::<f64>() {
            Ok(FqxValue::F64(v))
        } else if let Ok(v) = ob.extract::<String>() {
            Ok(FqxValue::String(v))
        } else if let Ok(v) = ob.extract::<Vec<u8>>() {
            Ok(FqxValue::Blob(v))
        } else {
            Ok(FqxValue::Null)
        }
    }
}

impl IntoPy<PyObject> for FqxValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            FqxValue::Bool(v) => v.into_py(py),
            FqxValue::U8(v) => v.into_py(py),
            FqxValue::U16(v) => v.into_py(py),
            FqxValue::U32(v) => v.into_py(py),
            FqxValue::U64(v) => v.into_py(py),
            FqxValue::I8(v) => v.into_py(py),
            FqxValue::I16(v) => v.into_py(py),
            FqxValue::I32(v) => v.into_py(py),
            FqxValue::I64(v) => v.into_py(py),
            FqxValue::F32(v) => v.into_py(py),
            FqxValue::F64(v) => v.into_py(py),
            FqxValue::String(v) => v.into_py(py),
            FqxValue::Blob(v) => v.into_py(py),
            FqxValue::Null => py.None(),
        }
    }
}

// ================================================================================================
// ToString
// ================================================================================================

impl ToString for FqxValue {
    fn to_string(&self) -> String {
        match self {
            FqxValue::Bool(v) => v.to_string(),
            FqxValue::U8(v) => v.to_string(),
            FqxValue::U16(v) => v.to_string(),
            FqxValue::U32(v) => v.to_string(),
            FqxValue::U64(v) => v.to_string(),
            FqxValue::I8(v) => v.to_string(),
            FqxValue::I16(v) => v.to_string(),
            FqxValue::I32(v) => v.to_string(),
            FqxValue::I64(v) => v.to_string(),
            FqxValue::F32(v) => v.to_string(),
            FqxValue::F64(v) => v.to_string(),
            FqxValue::String(v) => v.to_string(),
            FqxValue::Blob(v) => String::from_utf8(v.to_vec())
                .unwrap_or("Invalid conversion from Vec<u8>".to_string()),
            FqxValue::Null => "".to_string(),
        }
    }
}

pub fn try_from_str_with_type_hints(s: &str, type_hint: &FqxValueType) -> Result<FqxValue> {
    let res = match type_hint {
        FqxValueType::Bool => FqxValue::Bool(s.parse::<bool>()?),
        FqxValueType::U8 => FqxValue::U8(s.parse::<u8>()?),
        FqxValueType::U16 => FqxValue::U16(s.parse::<u16>()?),
        FqxValueType::U32 => FqxValue::U32(s.parse::<u32>()?),
        FqxValueType::U64 => FqxValue::U64(s.parse::<u64>()?),
        FqxValueType::I8 => FqxValue::I8(s.parse::<i8>()?),
        FqxValueType::I16 => FqxValue::I16(s.parse::<i16>()?),
        FqxValueType::I32 => FqxValue::I32(s.parse::<i32>()?),
        FqxValueType::I64 => FqxValue::I64(s.parse::<i64>()?),
        FqxValueType::F32 => FqxValue::F32(s.parse::<f32>()?),
        FqxValueType::F64 => FqxValue::F64(s.parse::<f64>()?),
        FqxValueType::String => FqxValue::String(s.into()),
        FqxValueType::Blob => FqxValue::Blob(s.as_bytes().to_vec()),
        FqxValueType::Null => FqxValue::Null,
    };

    Ok(res)
}

// ================================================================================================
// MsSql specification
// ================================================================================================

impl From<tiberius::ColumnType> for FqxValueType {
    fn from(value: tiberius::ColumnType) -> Self {
        match value {
            tiberius::ColumnType::Null => FqxValueType::Null,
            tiberius::ColumnType::Bit => FqxValueType::Bool,
            tiberius::ColumnType::Int1 => FqxValueType::I8,
            tiberius::ColumnType::Int2 => FqxValueType::I16,
            tiberius::ColumnType::Int4 => FqxValueType::I32,
            tiberius::ColumnType::Int8 => FqxValueType::I64,
            tiberius::ColumnType::Datetime4 => unimplemented!(),
            tiberius::ColumnType::Float4 => FqxValueType::F32,
            tiberius::ColumnType::Float8 => FqxValueType::F64,
            tiberius::ColumnType::Money => unimplemented!(),
            tiberius::ColumnType::Datetime => unimplemented!(),
            tiberius::ColumnType::Money4 => unimplemented!(),
            tiberius::ColumnType::Guid => unimplemented!(),
            tiberius::ColumnType::Intn => FqxValueType::I32,
            tiberius::ColumnType::Bitn => unimplemented!(),
            tiberius::ColumnType::Decimaln => unimplemented!(),
            tiberius::ColumnType::Numericn => unimplemented!(),
            tiberius::ColumnType::Floatn => FqxValueType::F64,
            tiberius::ColumnType::Datetimen => unimplemented!(),
            tiberius::ColumnType::Daten => unimplemented!(),
            tiberius::ColumnType::Timen => unimplemented!(),
            tiberius::ColumnType::Datetime2 => unimplemented!(),
            tiberius::ColumnType::DatetimeOffsetn => unimplemented!(),
            tiberius::ColumnType::BigVarBin => unimplemented!(),
            tiberius::ColumnType::BigVarChar => FqxValueType::String,
            tiberius::ColumnType::BigBinary => FqxValueType::String,
            tiberius::ColumnType::BigChar => FqxValueType::String,
            tiberius::ColumnType::NVarchar => FqxValueType::String,
            tiberius::ColumnType::NChar => FqxValueType::String,
            tiberius::ColumnType::Xml => FqxValueType::String,
            tiberius::ColumnType::Udt => FqxValueType::String,
            tiberius::ColumnType::Text => FqxValueType::String,
            tiberius::ColumnType::Image => unimplemented!(),
            tiberius::ColumnType::NText => unimplemented!(),
            tiberius::ColumnType::SSVariant => unimplemented!(),
        }
    }
}
