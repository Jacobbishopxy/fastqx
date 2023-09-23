//! file: cvt.rs
//! author: Jacob Xie
//! date: 2023/09/23 22:24:33 Saturday
//! brief:

use anyhow::{anyhow, Result};
use pyo3::prelude::*;

use crate::adt::{FqxValue, FqxValueType};

// ================================================================================================
// FqxValue <-> Rust types
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

macro_rules! impl_try_from_value_for_numeric {
    ($t:ty) => {
        impl TryFrom<FqxValue> for $t {
            type Error = anyhow::Error;

            fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
                match value {
                    FqxValue::Bool(_) => Err(anyhow!("failed to convert bool into numeric")),
                    FqxValue::U8(v) => Ok(v as $t),
                    FqxValue::U16(v) => Ok(v as $t),
                    FqxValue::U32(v) => Ok(v as $t),
                    FqxValue::U64(v) => Ok(v as $t),
                    FqxValue::I8(v) => Ok(v as $t),
                    FqxValue::I16(v) => Ok(v as $t),
                    FqxValue::I32(v) => Ok(v as $t),
                    FqxValue::I64(v) => Ok(v as $t),
                    FqxValue::F32(v) => Ok(v as $t),
                    FqxValue::F64(v) => Ok(v as $t),
                    FqxValue::String(v) => Ok(v.parse::<$t>()?),
                    FqxValue::Blob(_) => Err(anyhow!("failed to convert Vec<u8> into numeric")),
                    FqxValue::Null => Err(anyhow!("failed to convert Null into numeric")),
                }
            }
        }
    };
}

impl_try_from_value_for_numeric!(u8);
impl_try_from_value_for_numeric!(u16);
impl_try_from_value_for_numeric!(u32);
impl_try_from_value_for_numeric!(u64);
impl_try_from_value_for_numeric!(i8);
impl_try_from_value_for_numeric!(i16);
impl_try_from_value_for_numeric!(i32);
impl_try_from_value_for_numeric!(i64);
impl_try_from_value_for_numeric!(f32);
impl_try_from_value_for_numeric!(f64);

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

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_from_rs_for_value {
    ($t:ty, $v:ident) => {
        impl From<$t> for FqxValue {
            fn from(value: $t) -> Self {
                FqxValue::$v(value)
            }
        }
    };
}

impl_from_rs_for_value!(bool, Bool);
impl_from_rs_for_value!(u8, U8);
impl_from_rs_for_value!(u16, U16);
impl_from_rs_for_value!(u32, U32);
impl_from_rs_for_value!(u64, U64);
impl_from_rs_for_value!(i8, I8);
impl_from_rs_for_value!(i16, I16);
impl_from_rs_for_value!(i32, I32);
impl_from_rs_for_value!(i64, I64);
impl_from_rs_for_value!(f32, F32);
impl_from_rs_for_value!(f64, F64);
impl_from_rs_for_value!(String, String);
impl_from_rs_for_value!(Vec<u8>, Blob);

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

// ================================================================================================
// TryCast
// ================================================================================================

pub trait TryCast {
    fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue>;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl TryCast for bool {
    fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
        match typ {
            FqxValueType::Bool => Ok(FqxValue::Bool(self)),
            FqxValueType::U8 => Err(anyhow!("cannot cast bool into u8")),
            FqxValueType::U16 => Err(anyhow!("cannot cast bool into u16")),
            FqxValueType::U32 => Err(anyhow!("cannot cast bool into u32")),
            FqxValueType::U64 => Err(anyhow!("cannot cast bool into u64")),
            FqxValueType::I8 => Err(anyhow!("cannot cast bool into i8")),
            FqxValueType::I16 => Err(anyhow!("cannot cast bool into i16")),
            FqxValueType::I32 => Err(anyhow!("cannot cast bool into i32")),
            FqxValueType::I64 => Err(anyhow!("cannot cast bool into i64")),
            FqxValueType::F32 => Err(anyhow!("cannot cast bool into f32")),
            FqxValueType::F64 => Err(anyhow!("cannot cast bool into f64")),
            FqxValueType::String => Ok(FqxValue::String(self.to_string())),
            FqxValueType::Blob => Err(anyhow!("cannot cast bool into blob")),
            FqxValueType::Null => Ok(FqxValue::Null),
        }
    }
}

macro_rules! impl_try_cast_for_numeric {
    ($t:ty) => {
        impl TryCast for $t {
            fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
                match typ {
                    FqxValueType::Bool => Err(anyhow!("cannot cast numeric into bool")),
                    FqxValueType::U8 => Ok(FqxValue::U8(self as u8)),
                    FqxValueType::U16 => Ok(FqxValue::U16(self as u16)),
                    FqxValueType::U32 => Ok(FqxValue::U32(self as u32)),
                    FqxValueType::U64 => Ok(FqxValue::U64(self as u64)),
                    FqxValueType::I8 => Ok(FqxValue::I8(self as i8)),
                    FqxValueType::I16 => Ok(FqxValue::I16(self as i16)),
                    FqxValueType::I32 => Ok(FqxValue::I32(self as i32)),
                    FqxValueType::I64 => Ok(FqxValue::I64(self as i64)),
                    FqxValueType::F32 => Ok(FqxValue::F32(self as f32)),
                    FqxValueType::F64 => Ok(FqxValue::F64(self as f64)),
                    FqxValueType::String => Ok(FqxValue::String(self.to_string())),
                    FqxValueType::Blob => Err(anyhow!("cannot cast numeric into blob")),
                    FqxValueType::Null => Ok(FqxValue::Null),
                }
            }
        }
    };
}

impl_try_cast_for_numeric!(u8);
impl_try_cast_for_numeric!(u16);
impl_try_cast_for_numeric!(u32);
impl_try_cast_for_numeric!(u64);
impl_try_cast_for_numeric!(i8);
impl_try_cast_for_numeric!(i16);
impl_try_cast_for_numeric!(i32);
impl_try_cast_for_numeric!(i64);
impl_try_cast_for_numeric!(f32);
impl_try_cast_for_numeric!(f64);

impl TryCast for String {
    fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
        match typ {
            FqxValueType::Bool => Ok(FqxValue::Bool(str::parse(&self)?)),
            FqxValueType::U8 => Ok(FqxValue::U8(str::parse(&self)?)),
            FqxValueType::U16 => Ok(FqxValue::U16(str::parse(&self)?)),
            FqxValueType::U32 => Ok(FqxValue::U32(str::parse(&self)?)),
            FqxValueType::U64 => Ok(FqxValue::U64(str::parse(&self)?)),
            FqxValueType::I8 => Ok(FqxValue::I8(str::parse(&self)?)),
            FqxValueType::I16 => Ok(FqxValue::I16(str::parse(&self)?)),
            FqxValueType::I32 => Ok(FqxValue::I32(str::parse(&self)?)),
            FqxValueType::I64 => Ok(FqxValue::I64(str::parse(&self)?)),
            FqxValueType::F32 => Ok(FqxValue::F32(str::parse(&self)?)),
            FqxValueType::F64 => Ok(FqxValue::F64(str::parse(&self)?)),
            FqxValueType::String => Ok(FqxValue::String(self)),
            FqxValueType::Blob => Ok(FqxValue::Blob(self.as_bytes().to_vec())),
            FqxValueType::Null => Ok(FqxValue::Null),
        }
    }
}

impl TryCast for Vec<u8> {
    fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
        match typ {
            FqxValueType::Bool => Err(anyhow!("cannot cast Vec<u8> into bool")),
            FqxValueType::U8 => Err(anyhow!("cannot cast Vec<u8> into u8")),
            FqxValueType::U16 => Err(anyhow!("cannot cast Vec<u8> into u16")),
            FqxValueType::U32 => Err(anyhow!("cannot cast Vec<u8> into u32")),
            FqxValueType::U64 => Err(anyhow!("cannot cast Vec<u8> into u64")),
            FqxValueType::I8 => Err(anyhow!("cannot cast Vec<u8> into i8")),
            FqxValueType::I16 => Err(anyhow!("cannot cast Vec<u8> into i16")),
            FqxValueType::I32 => Err(anyhow!("cannot cast Vec<u8> into i32")),
            FqxValueType::I64 => Err(anyhow!("cannot cast Vec<u8> into i64")),
            FqxValueType::F32 => Err(anyhow!("cannot cast Vec<u8> into f32")),
            FqxValueType::F64 => Err(anyhow!("cannot cast Vec<u8> into f64")),
            FqxValueType::String => Ok(FqxValue::String(String::from_utf8(self)?)),
            FqxValueType::Blob => Ok(FqxValue::Blob(self)),
            FqxValueType::Null => Ok(FqxValue::Null),
        }
    }
}

// ================================================================================================
// MsSql specified
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
            tiberius::ColumnType::Intn => FqxValueType::I64,
            tiberius::ColumnType::Bitn => FqxValueType::Bool,
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
            tiberius::ColumnType::BigBinary => FqxValueType::Blob,
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
