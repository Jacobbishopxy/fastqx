//! file: value.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:46:03 Wednesday
//! brief:

use std::hash::Hash;

use anyhow::Result;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use super::ab::cvt::TryCast;

// ================================================================================================
// FqxValueType & FqxValue
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
    #[pyo3(name = "is_float", text_signature = "($self)")]
    pub fn is_float(&self) -> bool {
        match self {
            FqxValueType::F32 => true,
            FqxValueType::F64 => true,
            _ => false,
        }
    }

    fn __repr__(&self) -> &'static str {
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

// TODO: impl PartialEq, PartialOrd !!! type variants
#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
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

// IMPORTANT! Doesn't work on F32 & F64!
impl Eq for FqxValue {}

// IMPORTANT! Doesn't work on F32 & F64!
impl Hash for FqxValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Default for FqxValue {
    fn default() -> Self {
        FqxValue::Null
    }
}

impl<'a> From<&'a FqxValue> for FqxValue {
    fn from(value: &'a FqxValue) -> Self {
        value.clone()
    }
}

impl<'a> AsRef<FqxValue> for FqxValue {
    fn as_ref(&self) -> &FqxValue {
        &self
    }
}

impl<'a> AsMut<FqxValue> for FqxValue {
    fn as_mut(&mut self) -> &mut FqxValue {
        self
    }
}

impl FqxValue {
    pub fn is_float(&self) -> bool {
        match self {
            FqxValue::F32(_) => true,
            FqxValue::F64(_) => true,
            _ => false,
        }
    }

    pub fn try_cast(self, typ: &FqxValueType) -> Result<Self> {
        match self {
            FqxValue::Bool(v) => v.try_cast(typ),
            FqxValue::U8(v) => v.try_cast(typ),
            FqxValue::U16(v) => v.try_cast(typ),
            FqxValue::U32(v) => v.try_cast(typ),
            FqxValue::U64(v) => v.try_cast(typ),
            FqxValue::I8(v) => v.try_cast(typ),
            FqxValue::I16(v) => v.try_cast(typ),
            FqxValue::I32(v) => v.try_cast(typ),
            FqxValue::I64(v) => v.try_cast(typ),
            FqxValue::F32(v) => v.try_cast(typ),
            FqxValue::F64(v) => v.try_cast(typ),
            FqxValue::String(v) => v.try_cast(typ),
            FqxValue::Blob(v) => v.try_cast(typ),
            FqxValue::Null => Ok(FqxValue::Null),
        }
    }

    pub fn try_cast_mut(&mut self, typ: &FqxValueType) -> Result<()> {
        match self {
            FqxValue::Bool(v) => *self = v.try_cast(typ)?,
            FqxValue::U8(v) => *self = v.try_cast(typ)?,
            FqxValue::U16(v) => *self = v.try_cast(typ)?,
            FqxValue::U32(v) => *self = v.try_cast(typ)?,
            FqxValue::U64(v) => *self = v.try_cast(typ)?,
            FqxValue::I8(v) => *self = v.try_cast(typ)?,
            FqxValue::I16(v) => *self = v.try_cast(typ)?,
            FqxValue::I32(v) => *self = v.try_cast(typ)?,
            FqxValue::I64(v) => *self = v.try_cast(typ)?,
            FqxValue::F32(v) => *self = v.try_cast(typ)?,
            FqxValue::F64(v) => *self = v.try_cast(typ)?,
            FqxValue::String(v) => *self = v.clone().try_cast(typ)?,
            FqxValue::Blob(v) => *self = v.clone().try_cast(typ)?,
            FqxValue::Null => *self = FqxValue::Null,
        };

        Ok(())
    }
}
