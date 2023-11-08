//! file: cvt.rs
//! author: Jacob Xie
//! date: 2023/09/23 22:24:33 Saturday
//! brief:

use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use pyo3::{
    prelude::*,
    types::{PyDate, PyDateAccess, PyDateTime, PyTime, PyTimeAccess},
};

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
            _ => bail!("cannot cast bool from time-like types"),
        }
    }
}

macro_rules! impl_try_from_value_for_numeric {
    ($t:ty) => {
        impl TryFrom<FqxValue> for $t {
            type Error = anyhow::Error;

            fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
                match value {
                    FqxValue::Bool(_) => bail!("failed to convert bool into numeric"),
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
                    FqxValue::Blob(_) => bail!("failed to convert Vec<u8> into numeric"),
                    FqxValue::Null => bail!("failed to convert Null into numeric"),
                    _ => bail!("cannot cast bool from time-like types"),
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
            FqxValue::Timestamp(v) => Ok(format!("{:?}", v)),
            FqxValue::DateTime(v) => Ok(format!("{:?}", v)),
            FqxValue::Date(v) => Ok(format!("{:?}", v)),
            FqxValue::Time(v) => Ok(format!("{:?}", v)),
            FqxValue::Null => Ok("".to_string()),
        }
    }
}

impl TryFrom<FqxValue> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Blob(v) => Ok(v),
            _ => bail!("convert to Vec<u8> failed"),
        }
    }
}

impl TryFrom<FqxValue> for DateTime<Local> {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Timestamp(v) => Ok(v),
            _ => bail!("convert to DateTime<Local> failed"),
        }
    }
}

impl TryFrom<FqxValue> for NaiveDateTime {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::DateTime(v) => Ok(v),
            _ => bail!("convert to NaiveDateTime failed"),
        }
    }
}

impl TryFrom<FqxValue> for NaiveDate {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Date(v) => Ok(v),
            _ => bail!("convert to NaiveDate failed"),
        }
    }
}

impl TryFrom<FqxValue> for NaiveTime {
    type Error = anyhow::Error;

    fn try_from(value: FqxValue) -> std::result::Result<Self, Self::Error> {
        match value {
            FqxValue::Time(v) => Ok(v),
            _ => bail!("convert to NaiveTime failed"),
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

        impl From<Option<$t>> for FqxValue {
            fn from(value: Option<$t>) -> Self {
                match value {
                    Some(v) => FqxValue::$v(v),
                    None => FqxValue::Null,
                }
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
impl_from_rs_for_value!(DateTime<Local>, Timestamp);
impl_from_rs_for_value!(NaiveDateTime, DateTime);
impl_from_rs_for_value!(NaiveDate, Date);
impl_from_rs_for_value!(NaiveTime, Time);
impl_from_rs_for_value!(Vec<u8>, Blob);

impl From<&str> for FqxValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

// ================================================================================================
// FqxValue <-> FqxValueType
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
            FqxValue::Timestamp(_) => FqxValueType::Timestamp,
            FqxValue::DateTime(_) => FqxValueType::DateTime,
            FqxValue::Date(_) => FqxValueType::Date,
            FqxValue::Time(_) => FqxValueType::Time,
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
            FqxValue::Timestamp(_) => FqxValueType::Timestamp,
            FqxValue::DateTime(_) => FqxValueType::DateTime,
            FqxValue::Date(_) => FqxValueType::Date,
            FqxValue::Time(_) => FqxValueType::Time,
            FqxValue::Null => FqxValueType::Null,
        }
    }
}

// ================================================================================================
// FromPyObject & IntoPy<PyObject>
// ================================================================================================

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
        } else if let Ok(v) = <PyDateTime as pyo3::PyTryFrom>::try_from(ob) {
            let d = NaiveDate::from_ymd_opt(
                v.get_year() as i32,
                v.get_month() as u32,
                v.get_day() as u32,
            )
            .ok_or::<PyErr>(anyhow!("naive_date casting failed").into())?;
            let t = NaiveTime::from_hms_micro_opt(
                v.get_hour() as u32,
                v.get_minute() as u32,
                v.get_second() as u32,
                v.get_microsecond(),
            )
            .ok_or::<PyErr>(anyhow!("naive_time casting failed").into())?;
            let dt = NaiveDateTime::new(d, t);
            Ok(FqxValue::DateTime(dt))
        } else if let Ok(v) = <PyDate as pyo3::PyTryFrom>::try_from(ob) {
            let d = NaiveDate::from_ymd_opt(
                v.get_year() as i32,
                v.get_month() as u32,
                v.get_day() as u32,
            )
            .ok_or::<PyErr>(anyhow!("naive_date casting failed").into())?;
            Ok(FqxValue::Date(d))
        } else if let Ok(v) = <PyTime as pyo3::PyTryFrom>::try_from(ob) {
            let t = NaiveTime::from_hms_micro_opt(
                v.get_hour() as u32,
                v.get_minute() as u32,
                v.get_second() as u32,
                v.get_microsecond(),
            )
            .ok_or::<PyErr>(anyhow!("naive_time casting failed").into())?;
            Ok(FqxValue::Time(t))
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
            FqxValue::Timestamp(v) => v.timestamp().into_py(py),
            FqxValue::DateTime(v) => {
                let (d, t) = (v.date(), v.time());
                PyDateTime::new(
                    py,
                    d.year(),
                    d.month() as u8,
                    d.day() as u8,
                    t.hour() as u8,
                    t.minute() as u8,
                    t.second() as u8,
                    t.nanosecond() / 1000,
                    None,
                )
                .map_or(py.None(), |pdt| pdt.into_py(py))
            }
            FqxValue::Date(v) => PyDate::new(py, v.year(), v.month() as u8, v.day() as u8)
                .map_or(py.None(), |pdt| pdt.into_py(py)),
            FqxValue::Time(v) => PyTime::new(
                py,
                v.hour() as u8,
                v.minute() as u8,
                v.second() as u8,
                v.nanosecond() / 1000,
                None,
            )
            .map_or(py.None(), |pdt| pdt.into_py(py)),
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
            FqxValue::Timestamp(v) => format!("{:?}", v),
            FqxValue::DateTime(v) => format!("{:?}", v),
            FqxValue::Date(v) => format!("{:?}", v),
            FqxValue::Time(v) => format!("{:?}", v),
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
            FqxValueType::U8 => bail!("cannot cast bool into u8"),
            FqxValueType::U16 => bail!("cannot cast bool into u16"),
            FqxValueType::U32 => bail!("cannot cast bool into u32"),
            FqxValueType::U64 => bail!("cannot cast bool into u64"),
            FqxValueType::I8 => bail!("cannot cast bool into i8"),
            FqxValueType::I16 => bail!("cannot cast bool into i16"),
            FqxValueType::I32 => bail!("cannot cast bool into i32"),
            FqxValueType::I64 => bail!("cannot cast bool into i64"),
            FqxValueType::F32 => bail!("cannot cast bool into f32"),
            FqxValueType::F64 => bail!("cannot cast bool into f64"),
            FqxValueType::String => Ok(FqxValue::String(self.to_string())),
            FqxValueType::Blob => bail!("cannot cast bool into blob"),
            FqxValueType::Timestamp => bail!("cannot cast bool into timestamp"),
            FqxValueType::DateTime => bail!("cannot cast bool into date_time"),
            FqxValueType::Date => bail!("cannot cast bool into date"),
            FqxValueType::Time => bail!("cannot cast bool into time"),
            FqxValueType::Null => Ok(FqxValue::Null),
        }
    }
}

macro_rules! impl_try_cast_for_numeric {
    ($t:ty) => {
        impl TryCast for $t {
            fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
                match typ {
                    FqxValueType::Bool => bail!("cannot cast numeric into bool"),
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
                    FqxValueType::Blob => bail!("cannot cast numeric into blob"),
                    FqxValueType::Null => Ok(FqxValue::Null),
                    _ => bail!("cannot cast numeric into time-like types"),
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
            FqxValueType::Timestamp => Ok(FqxValue::Timestamp(DateTime::<Local>::from_str(&self)?)),
            FqxValueType::DateTime => Ok(FqxValue::DateTime(NaiveDateTime::from_str(&self)?)),
            FqxValueType::Date => Ok(FqxValue::Date(NaiveDate::from_str(&self)?)),
            FqxValueType::Time => Ok(FqxValue::Time(NaiveTime::from_str(&self)?)),
            FqxValueType::Null => Ok(FqxValue::Null),
        }
    }
}

impl<'a> TryCast for &'a str {
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
            FqxValueType::String => Ok(FqxValue::String(self.to_string())),
            FqxValueType::Blob => Ok(FqxValue::Blob(self.as_bytes().to_vec())),
            FqxValueType::Timestamp => Ok(FqxValue::Timestamp(DateTime::<Local>::from_str(&self)?)),
            FqxValueType::DateTime => Ok(FqxValue::DateTime(NaiveDateTime::from_str(&self)?)),
            FqxValueType::Date => Ok(FqxValue::Date(NaiveDate::from_str(&self)?)),
            FqxValueType::Time => Ok(FqxValue::Time(NaiveTime::from_str(&self)?)),
            FqxValueType::Null => Ok(FqxValue::Null),
        }
    }
}

impl TryCast for Vec<u8> {
    fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
        match typ {
            FqxValueType::Bool => bail!("cannot cast Vec<u8> into bool"),
            FqxValueType::U8 => bail!("cannot cast Vec<u8> into u8"),
            FqxValueType::U16 => bail!("cannot cast Vec<u8> into u16"),
            FqxValueType::U32 => bail!("cannot cast Vec<u8> into u32"),
            FqxValueType::U64 => bail!("cannot cast Vec<u8> into u64"),
            FqxValueType::I8 => bail!("cannot cast Vec<u8> into i8"),
            FqxValueType::I16 => bail!("cannot cast Vec<u8> into i16"),
            FqxValueType::I32 => bail!("cannot cast Vec<u8> into i32"),
            FqxValueType::I64 => bail!("cannot cast Vec<u8> into i64"),
            FqxValueType::F32 => bail!("cannot cast Vec<u8> into f32"),
            FqxValueType::F64 => bail!("cannot cast Vec<u8> into f64"),
            FqxValueType::String => Ok(FqxValue::String(String::from_utf8(self)?)),
            FqxValueType::Blob => Ok(FqxValue::Blob(self)),
            FqxValueType::Null => Ok(FqxValue::Null),
            _ => bail!("cannot cast Vec<u8> into time-like types"),
        }
    }
}

impl TryCast for DateTime<Local> {
    fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
        match typ {
            FqxValueType::Timestamp => Ok(FqxValue::Timestamp(self)),
            FqxValueType::String => Ok(FqxValue::String(format!("{:?}", self))),
            _ => bail!("cannot cast into timestamp"),
        }
    }
}

impl TryCast for NaiveDateTime {
    fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
        match typ {
            FqxValueType::DateTime => Ok(FqxValue::DateTime(self)),
            FqxValueType::String => Ok(FqxValue::String(format!("{:?}", self))),
            _ => bail!("cannot cast into date_time"),
        }
    }
}

impl TryCast for NaiveDate {
    fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
        match typ {
            FqxValueType::Date => Ok(FqxValue::Date(self)),
            FqxValueType::String => Ok(FqxValue::String(format!("{:?}", self))),
            _ => bail!("cannot cast into date"),
        }
    }
}

impl TryCast for NaiveTime {
    fn try_cast(self, typ: &FqxValueType) -> Result<FqxValue> {
        match typ {
            FqxValueType::Time => Ok(FqxValue::Time(self)),
            FqxValueType::String => Ok(FqxValue::String(format!("{:?}", self))),
            _ => bail!("cannot cast into time"),
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
            tiberius::ColumnType::Datetime => FqxValueType::DateTime,
            tiberius::ColumnType::Money4 => unimplemented!(),
            tiberius::ColumnType::Guid => unimplemented!(),
            tiberius::ColumnType::Intn => FqxValueType::I64,
            tiberius::ColumnType::Bitn => FqxValueType::Bool,
            tiberius::ColumnType::Decimaln => unimplemented!(),
            tiberius::ColumnType::Numericn => unimplemented!(),
            tiberius::ColumnType::Floatn => FqxValueType::F64,
            tiberius::ColumnType::Datetimen => FqxValueType::DateTime,
            tiberius::ColumnType::Daten => FqxValueType::Date,
            tiberius::ColumnType::Timen => FqxValueType::Time,
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

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_cvt {
    use std::str::FromStr;

    use chrono::FixedOffset;

    use super::*;
    use crate::fqx;

    #[test]
    fn chrono_conversions() {
        let nd = NaiveDate::from_ymd_opt(2023, 11, 8).unwrap();
        let nt = NaiveTime::from_hms_opt(10, 40, 0).unwrap();
        let ndt = NaiveDateTime::new(nd.clone(), nt.clone());
        let dt = ndt
            .and_local_timezone(FixedOffset::east_opt(8 * 3600).unwrap())
            .unwrap();

        println!("{:?}", nd);
        println!("{:?}", nt);
        println!("{:?}", ndt);
        println!("{:?}", dt);

        let str_nd = format!("{:?}", &nd);
        let str_nt = format!("{:?}", &nt);
        let str_ndt = format!("{:?}", &ndt);
        let str_dt = format!("{:?}", &dt);

        println!("{:?}", str_nd); // "2023-11-08"
        println!("{:?}", str_nt); // "10:40:00"
        println!("{:?}", str_ndt); // "2023-11-08T10:40:00"
        println!("{:?}", str_dt); // "2023-11-08T18:40:00+08:00"

        let nd_ = NaiveDate::from_str(&str_nd).unwrap();
        let nt_ = NaiveTime::from_str(&str_nt).unwrap();
        let ndt_ = NaiveDateTime::from_str(&str_ndt).unwrap();
        let dt_ = DateTime::<Local>::from_str(&str_dt).unwrap();

        println!("{:?}", nd_);
        println!("{:?}", nt_);
        println!("{:?}", ndt_);
        println!("{:?}", dt_);
    }

    #[test]
    fn fqx_time_like_value_conversions_success() {
        let nd = NaiveDate::from_ymd_opt(2023, 11, 8).unwrap();
        let nt = NaiveTime::from_hms_opt(10, 40, 0).unwrap();
        let ndt = NaiveDateTime::new(nd.clone(), nt.clone());
        let dt = DateTime::<Local>::from_naive_utc_and_offset(
            ndt.clone(),
            FixedOffset::east_opt(8 * 3600).unwrap(),
        );

        let v1 = fqx!(nd);
        let v2 = fqx!(nt);
        let v3 = fqx!(ndt);
        let v4 = fqx!(dt);

        println!("{:?}", v1);
        println!("{:?}", v2);
        println!("{:?}", v3);
        println!("{:?}", v4);

        let v1_ = v1.to_string();
        let v2_ = v2.to_string();
        let v3_ = v3.to_string();
        let v4_ = v4.to_string();

        println!("{:?}", v1_.try_cast(&FqxValueType::Date));
        println!("{:?}", v2_.try_cast(&FqxValueType::Time));
        println!("{:?}", v3_.try_cast(&FqxValueType::DateTime));
        println!("{:?}", v4_.try_cast(&FqxValueType::Timestamp));
    }

    #[test]
    fn tmp() {
        let nd = NaiveDate::from_ymd_opt(2023, 11, 8).unwrap();
        let nt = NaiveTime::from_hms_opt(10, 40, 0).unwrap();
        let ndt = NaiveDateTime::new(nd.clone(), nt.clone());
        let dt = ndt
            .and_local_timezone(FixedOffset::east_opt(8 * 3600).unwrap())
            .unwrap();

        println!("{:?}", dt.to_string());
    }
}
