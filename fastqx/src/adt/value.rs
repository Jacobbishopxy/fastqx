//! file: value.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:46:03 Wednesday
//! brief:

use std::cmp::Ordering;
use std::hash::Hash;

use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
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
    Timestamp,
    DateTime,
    Date,
    Time,
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    Timestamp(DateTime<Local>),
    DateTime(NaiveDateTime),
    Date(NaiveDate),
    Time(NaiveTime),
    Null,
}

// ================================================================================================
// Impl
// ================================================================================================

impl FqxValue {
    pub fn is_type(&self, t: &FqxValueType) -> bool {
        match self {
            FqxValue::Bool(_) => matches!(t, FqxValueType::Bool),
            FqxValue::U8(_) => matches!(t, FqxValueType::U8),
            FqxValue::U16(_) => matches!(t, FqxValueType::U16),
            FqxValue::U32(_) => matches!(t, FqxValueType::U32),
            FqxValue::U64(_) => matches!(t, FqxValueType::U64),
            FqxValue::I8(_) => matches!(t, FqxValueType::I8),
            FqxValue::I16(_) => matches!(t, FqxValueType::I16),
            FqxValue::I32(_) => matches!(t, FqxValueType::I32),
            FqxValue::I64(_) => matches!(t, FqxValueType::I64),
            FqxValue::F32(_) => matches!(t, FqxValueType::F32),
            FqxValue::F64(_) => matches!(t, FqxValueType::F64),
            FqxValue::String(_) => matches!(t, FqxValueType::String),
            FqxValue::Blob(_) => matches!(t, FqxValueType::Blob),
            FqxValue::Timestamp(_) => matches!(t, FqxValueType::Timestamp),
            FqxValue::DateTime(_) => matches!(t, FqxValueType::DateTime),
            FqxValue::Date(_) => matches!(t, FqxValueType::Date),
            FqxValue::Time(_) => matches!(t, FqxValueType::Time),
            FqxValue::Null => matches!(t, FqxValueType::Null),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl PartialEq for FqxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Blob(l0), Self::Blob(r0)) => l0 == r0,
            (Self::Timestamp(l0), Self::Timestamp(r0)) => l0 == r0,
            (Self::DateTime(l0), Self::DateTime(r0)) => l0 == r0,
            (Self::Date(l0), Self::Date(r0)) => l0 == r0,
            (Self::Time(l0), Self::Time(r0)) => l0 == r0,
            (Self::Null, Self::Null) => true,
            // numeric types comparing
            (Self::U8(l), Self::U8(r)) => l == r,
            (Self::U8(l), Self::U16(r)) => *l as u16 == *r,
            (Self::U8(l), Self::U32(r)) => *l as u32 == *r,
            (Self::U8(l), Self::U64(r)) => *l as u64 == *r,
            (Self::U8(l), Self::I8(r)) => *l as i8 == *r,
            (Self::U8(l), Self::I16(r)) => *l as i16 == *r,
            (Self::U8(l), Self::I32(r)) => *l as i32 == *r,
            (Self::U8(l), Self::I64(r)) => *l as i64 == *r,
            (Self::U8(l), Self::F32(r)) => *l as f32 == *r,
            (Self::U8(l), Self::F64(r)) => *l as f64 == *r,
            (Self::U16(l), Self::U8(r)) => *l == *r as u16,
            (Self::U16(l), Self::U16(r)) => l == r,
            (Self::U16(l), Self::U32(r)) => *l as u32 == *r,
            (Self::U16(l), Self::U64(r)) => *l as u64 == *r,
            (Self::U16(l), Self::I8(r)) => *l as i8 == *r,
            (Self::U16(l), Self::I16(r)) => *l as i16 == *r,
            (Self::U16(l), Self::I32(r)) => *l as i32 == *r,
            (Self::U16(l), Self::I64(r)) => *l as i64 == *r,
            (Self::U16(l), Self::F32(r)) => *l as f32 == *r,
            (Self::U16(l), Self::F64(r)) => *l as f64 == *r,
            (Self::U32(l), Self::U8(r)) => *l == *r as u32,
            (Self::U32(l), Self::U16(r)) => *l == *r as u32,
            (Self::U32(l), Self::U32(r)) => l == r,
            (Self::U32(l), Self::U64(r)) => *l as u64 == *r,
            (Self::U32(l), Self::I8(r)) => *l as i8 == *r,
            (Self::U32(l), Self::I16(r)) => *l as i16 == *r,
            (Self::U32(l), Self::I32(r)) => *l as i32 == *r,
            (Self::U32(l), Self::I64(r)) => *l as i64 == *r,
            (Self::U32(l), Self::F32(r)) => *l as f32 == *r,
            (Self::U32(l), Self::F64(r)) => *l as f64 == *r,
            (Self::U64(l), Self::U8(r)) => *l == *r as u64,
            (Self::U64(l), Self::U16(r)) => *l == *r as u64,
            (Self::U64(l), Self::U32(r)) => *l == *r as u64,
            (Self::U64(l), Self::U64(r)) => l == r,
            (Self::U64(l), Self::I8(r)) => *l as i8 == *r,
            (Self::U64(l), Self::I16(r)) => *l as i16 == *r,
            (Self::U64(l), Self::I32(r)) => *l as i32 == *r,
            (Self::U64(l), Self::I64(r)) => *l as i64 == *r,
            (Self::U64(l), Self::F32(r)) => *l as f32 == *r,
            (Self::U64(l), Self::F64(r)) => *l as f64 == *r,
            (Self::I8(l), Self::U8(r)) => *l == *r as i8,
            (Self::I8(l), Self::U16(r)) => *l == *r as i8,
            (Self::I8(l), Self::U32(r)) => *l == *r as i8,
            (Self::I8(l), Self::U64(r)) => *l == *r as i8,
            (Self::I8(l), Self::I8(r)) => l == r,
            (Self::I8(l), Self::I16(r)) => *l as i16 == *r,
            (Self::I8(l), Self::I32(r)) => *l as i32 == *r,
            (Self::I8(l), Self::I64(r)) => *l as i64 == *r,
            (Self::I8(l), Self::F32(r)) => *l as f32 == *r,
            (Self::I8(l), Self::F64(r)) => *l as f64 == *r,
            (Self::I16(l), Self::U8(r)) => *l == *r as i16,
            (Self::I16(l), Self::U16(r)) => *l == *r as i16,
            (Self::I16(l), Self::U32(r)) => *l == *r as i16,
            (Self::I16(l), Self::U64(r)) => *l == *r as i16,
            (Self::I16(l), Self::I8(r)) => *l == *r as i16,
            (Self::I16(l), Self::I16(r)) => l == r,
            (Self::I16(l), Self::I32(r)) => *l as i32 == *r,
            (Self::I16(l), Self::I64(r)) => *l as i64 == *r,
            (Self::I16(l), Self::F32(r)) => *l as f32 == *r,
            (Self::I16(l), Self::F64(r)) => *l as f64 == *r,
            (Self::I32(l), Self::U8(r)) => *l == *r as i32,
            (Self::I32(l), Self::U16(r)) => *l == *r as i32,
            (Self::I32(l), Self::U32(r)) => *l == *r as i32,
            (Self::I32(l), Self::U64(r)) => *l == *r as i32,
            (Self::I32(l), Self::I8(r)) => *l == *r as i32,
            (Self::I32(l), Self::I16(r)) => *l == *r as i32,
            (Self::I32(l), Self::I32(r)) => l == r,
            (Self::I32(l), Self::I64(r)) => *l as i64 == *r,
            (Self::I32(l), Self::F32(r)) => *l as f32 == *r,
            (Self::I32(l), Self::F64(r)) => *l as f64 == *r,
            (Self::I64(l), Self::U8(r)) => *l == *r as i64,
            (Self::I64(l), Self::U16(r)) => *l == *r as i64,
            (Self::I64(l), Self::U32(r)) => *l == *r as i64,
            (Self::I64(l), Self::U64(r)) => *l == *r as i64,
            (Self::I64(l), Self::I8(r)) => *l == *r as i64,
            (Self::I64(l), Self::I16(r)) => *l == *r as i64,
            (Self::I64(l), Self::I32(r)) => *l == *r as i64,
            (Self::I64(l), Self::I64(r)) => l == r,
            (Self::I64(l), Self::F32(r)) => *l as f32 == *r,
            (Self::I64(l), Self::F64(r)) => *l as f64 == *r,
            (Self::F32(l), Self::U8(r)) => *l == *r as f32,
            (Self::F32(l), Self::U16(r)) => *l == *r as f32,
            (Self::F32(l), Self::U32(r)) => *l == *r as f32,
            (Self::F32(l), Self::U64(r)) => *l == *r as f32,
            (Self::F32(l), Self::I8(r)) => *l == *r as f32,
            (Self::F32(l), Self::I16(r)) => *l == *r as f32,
            (Self::F32(l), Self::I32(r)) => *l == *r as f32,
            (Self::F32(l), Self::I64(r)) => *l == *r as f32,
            (Self::F32(l), Self::F32(r)) => l == r,
            (Self::F32(l), Self::F64(r)) => *l == *r as f32,
            (Self::F64(l), Self::U8(r)) => *l == *r as f64,
            (Self::F64(l), Self::U16(r)) => *l == *r as f64,
            (Self::F64(l), Self::U32(r)) => *l == *r as f64,
            (Self::F64(l), Self::U64(r)) => *l == *r as f64,
            (Self::F64(l), Self::I8(r)) => *l == *r as f64,
            (Self::F64(l), Self::I16(r)) => *l == *r as f64,
            (Self::F64(l), Self::I32(r)) => *l == *r as f64,
            (Self::F64(l), Self::I64(r)) => *l == *r as f64,
            (Self::F64(l), Self::F32(r)) => *l == *r as f64,
            (Self::F64(l), Self::F64(r)) => l == r,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for FqxValue {}

impl PartialOrd for FqxValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Bool(l), Self::Bool(r)) => Some(l.cmp(r)),
            (Self::String(l), Self::String(r)) => Some(l.cmp(r)),
            (Self::Blob(l), Self::Blob(r)) => Some(l.cmp(r)),
            (Self::Timestamp(l), Self::Timestamp(r)) => Some(l.cmp(r)),
            (Self::DateTime(l), Self::DateTime(r)) => Some(l.cmp(r)),
            (Self::Date(l), Self::Date(r)) => Some(l.cmp(r)),
            (Self::Time(l), Self::Time(r)) => Some(l.cmp(r)),
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            // numeric types ordering
            (Self::U8(l), Self::U8(r)) => Some(l.cmp(r)),
            (Self::U8(l), Self::U16(r)) => Some((*l as u16).cmp(r)),
            (Self::U8(l), Self::U32(r)) => Some((*l as u32).cmp(r)),
            (Self::U8(l), Self::U64(r)) => Some((*l as u64).cmp(r)),
            (Self::U8(l), Self::I8(r)) => Some((*l as i8).cmp(r)),
            (Self::U8(l), Self::I16(r)) => Some((*l as i16).cmp(r)),
            (Self::U8(l), Self::I32(r)) => Some((*l as i32).cmp(r)),
            (Self::U8(l), Self::I64(r)) => Some((*l as i64).cmp(r)),
            (Self::U8(l), Self::F32(r)) => (*l as f32).partial_cmp(r),
            (Self::U8(l), Self::F64(r)) => (*l as f64).partial_cmp(r),
            (Self::U16(l), Self::U8(r)) => Some(l.cmp(&(*r as u16))),
            (Self::U16(l), Self::U16(r)) => Some(l.cmp(r)),
            (Self::U16(l), Self::U32(r)) => Some((*l as u32).cmp(r)),
            (Self::U16(l), Self::U64(r)) => Some((*l as u64).cmp(r)),
            (Self::U16(l), Self::I8(r)) => Some((*l as i8).cmp(r)),
            (Self::U16(l), Self::I16(r)) => Some((*l as i16).cmp(r)),
            (Self::U16(l), Self::I32(r)) => Some((*l as i32).cmp(r)),
            (Self::U16(l), Self::I64(r)) => Some((*l as i64).cmp(r)),
            (Self::U16(l), Self::F32(r)) => (*l as f32).partial_cmp(r),
            (Self::U16(l), Self::F64(r)) => (*l as f64).partial_cmp(r),
            (Self::U32(l), Self::U8(r)) => Some(l.cmp(&(*r as u32))),
            (Self::U32(l), Self::U16(r)) => Some(l.cmp(&(*r as u32))),
            (Self::U32(l), Self::U32(r)) => Some(l.cmp(r)),
            (Self::U32(l), Self::U64(r)) => Some((*l as u64).cmp(r)),
            (Self::U32(l), Self::I8(r)) => Some((*l as i8).cmp(r)),
            (Self::U32(l), Self::I16(r)) => Some((*l as i16).cmp(r)),
            (Self::U32(l), Self::I32(r)) => Some((*l as i32).cmp(r)),
            (Self::U32(l), Self::I64(r)) => Some((*l as i64).cmp(r)),
            (Self::U32(l), Self::F32(r)) => (*l as f32).partial_cmp(r),
            (Self::U32(l), Self::F64(r)) => (*l as f64).partial_cmp(r),
            (Self::U64(l), Self::U8(r)) => Some(l.cmp(&(*r as u64))),
            (Self::U64(l), Self::U16(r)) => Some(l.cmp(&(*r as u64))),
            (Self::U64(l), Self::U32(r)) => Some(l.cmp(&(*r as u64))),
            (Self::U64(l), Self::U64(r)) => Some(l.cmp(r)),
            (Self::U64(l), Self::I8(r)) => Some((*l as i8).cmp(r)),
            (Self::U64(l), Self::I16(r)) => Some((*l as i16).cmp(r)),
            (Self::U64(l), Self::I32(r)) => Some((*l as i32).cmp(r)),
            (Self::U64(l), Self::I64(r)) => Some((*l as i64).cmp(r)),
            (Self::U64(l), Self::F32(r)) => (*l as f32).partial_cmp(r),
            (Self::U64(l), Self::F64(r)) => (*l as f64).partial_cmp(r),
            (Self::I8(l), Self::U8(r)) => Some(l.cmp(&(*r as i8))),
            (Self::I8(l), Self::U16(r)) => Some(l.cmp(&(*r as i8))),
            (Self::I8(l), Self::U32(r)) => Some(l.cmp(&(*r as i8))),
            (Self::I8(l), Self::U64(r)) => Some(l.cmp(&(*r as i8))),
            (Self::I8(l), Self::I8(r)) => Some(l.cmp(r)),
            (Self::I8(l), Self::I16(r)) => Some((*l as i16).cmp(r)),
            (Self::I8(l), Self::I32(r)) => Some((*l as i32).cmp(r)),
            (Self::I8(l), Self::I64(r)) => Some((*l as i64).cmp(r)),
            (Self::I8(l), Self::F32(r)) => (*l as f32).partial_cmp(r),
            (Self::I8(l), Self::F64(r)) => (*l as f64).partial_cmp(r),
            (Self::I16(l), Self::U8(r)) => Some(l.cmp(&(*r as i16))),
            (Self::I16(l), Self::U16(r)) => Some(l.cmp(&(*r as i16))),
            (Self::I16(l), Self::U32(r)) => Some(l.cmp(&(*r as i16))),
            (Self::I16(l), Self::U64(r)) => Some(l.cmp(&(*r as i16))),
            (Self::I16(l), Self::I8(r)) => Some(l.cmp(&(*r as i16))),
            (Self::I16(l), Self::I16(r)) => Some(l.cmp(r)),
            (Self::I16(l), Self::I32(r)) => Some((*l as i32).cmp(r)),
            (Self::I16(l), Self::I64(r)) => Some((*l as i64).cmp(r)),
            (Self::I16(l), Self::F32(r)) => (*l as f32).partial_cmp(r),
            (Self::I16(l), Self::F64(r)) => (*l as f64).partial_cmp(r),
            (Self::I32(l), Self::U8(r)) => Some(l.cmp(&(*r as i32))),
            (Self::I32(l), Self::U16(r)) => Some(l.cmp(&(*r as i32))),
            (Self::I32(l), Self::U32(r)) => Some(l.cmp(&(*r as i32))),
            (Self::I32(l), Self::U64(r)) => Some(l.cmp(&(*r as i32))),
            (Self::I32(l), Self::I8(r)) => Some(l.cmp(&(*r as i32))),
            (Self::I32(l), Self::I16(r)) => Some(l.cmp(&(*r as i32))),
            (Self::I32(l), Self::I32(r)) => Some(l.cmp(r)),
            (Self::I32(l), Self::I64(r)) => Some((*l as i64).cmp(r)),
            (Self::I32(l), Self::F32(r)) => (*l as f32).partial_cmp(r),
            (Self::I32(l), Self::F64(r)) => (*l as f64).partial_cmp(r),
            (Self::I64(l), Self::U8(r)) => Some(l.cmp(&(*r as i64))),
            (Self::I64(l), Self::U16(r)) => Some(l.cmp(&(*r as i64))),
            (Self::I64(l), Self::U32(r)) => Some(l.cmp(&(*r as i64))),
            (Self::I64(l), Self::U64(r)) => Some(l.cmp(&(*r as i64))),
            (Self::I64(l), Self::I8(r)) => Some(l.cmp(&(*r as i64))),
            (Self::I64(l), Self::I16(r)) => Some(l.cmp(&(*r as i64))),
            (Self::I64(l), Self::I32(r)) => Some(l.cmp(&(*r as i64))),
            (Self::I64(l), Self::I64(r)) => Some(l.cmp(r)),
            (Self::I64(l), Self::F32(r)) => (*l as f32).partial_cmp(r),
            (Self::I64(l), Self::F64(r)) => (*l as f64).partial_cmp(r),
            (Self::F32(l), Self::U8(r)) => l.partial_cmp(&(*r as f32)),
            (Self::F32(l), Self::U16(r)) => l.partial_cmp(&(*r as f32)),
            (Self::F32(l), Self::U32(r)) => l.partial_cmp(&(*r as f32)),
            (Self::F32(l), Self::U64(r)) => l.partial_cmp(&(*r as f32)),
            (Self::F32(l), Self::I8(r)) => l.partial_cmp(&(*r as f32)),
            (Self::F32(l), Self::I16(r)) => l.partial_cmp(&(*r as f32)),
            (Self::F32(l), Self::I32(r)) => l.partial_cmp(&(*r as f32)),
            (Self::F32(l), Self::I64(r)) => l.partial_cmp(&(*r as f32)),
            (Self::F32(l), Self::F32(r)) => l.partial_cmp(r),
            (Self::F32(l), Self::F64(r)) => l.partial_cmp(&(*r as f32)),
            (Self::F64(l), Self::U8(r)) => l.partial_cmp(&(*r as f64)),
            (Self::F64(l), Self::U16(r)) => l.partial_cmp(&(*r as f64)),
            (Self::F64(l), Self::U32(r)) => l.partial_cmp(&(*r as f64)),
            (Self::F64(l), Self::U64(r)) => l.partial_cmp(&(*r as f64)),
            (Self::F64(l), Self::I8(r)) => l.partial_cmp(&(*r as f64)),
            (Self::F64(l), Self::I16(r)) => l.partial_cmp(&(*r as f64)),
            (Self::F64(l), Self::I32(r)) => l.partial_cmp(&(*r as f64)),
            (Self::F64(l), Self::I64(r)) => l.partial_cmp(&(*r as f64)),
            (Self::F64(l), Self::F32(r)) => l.partial_cmp(&(*r as f64)),
            (Self::F64(l), Self::F64(r)) => l.partial_cmp(r),
            _ => None,
        }
    }
}

impl Ord for FqxValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

// IMPORTANT! Doesn't work on F32 & F64!
impl Hash for FqxValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl PartialEq<FqxValueType> for FqxValue {
    fn eq(&self, other: &FqxValueType) -> bool {
        match self {
            FqxValue::Bool(_) => matches!(other, &FqxValueType::Bool),
            FqxValue::U8(_) => matches!(other, &FqxValueType::U8),
            FqxValue::U16(_) => matches!(other, &FqxValueType::U16),
            FqxValue::U32(_) => matches!(other, &FqxValueType::U32),
            FqxValue::U64(_) => matches!(other, &FqxValueType::U64),
            FqxValue::I8(_) => matches!(other, &FqxValueType::I8),
            FqxValue::I16(_) => matches!(other, &FqxValueType::I16),
            FqxValue::I32(_) => matches!(other, &FqxValueType::I32),
            FqxValue::I64(_) => matches!(other, &FqxValueType::I64),
            FqxValue::F32(_) => matches!(other, &FqxValueType::F32),
            FqxValue::F64(_) => matches!(other, &FqxValueType::F64),
            FqxValue::String(_) => matches!(other, &FqxValueType::String),
            FqxValue::Blob(_) => matches!(other, &FqxValueType::Blob),
            FqxValue::Timestamp(_) => matches!(other, &FqxValueType::Timestamp),
            FqxValue::DateTime(_) => matches!(other, &FqxValueType::DateTime),
            FqxValue::Date(_) => matches!(other, &FqxValueType::Date),
            FqxValue::Time(_) => matches!(other, &FqxValueType::Time),
            FqxValue::Null => true, // null value can be any type
        }
    }
}

impl Default for FqxValue {
    fn default() -> Self {
        FqxValue::Null
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////////////////////////////

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
            FqxValue::Timestamp(v) => v.try_cast(typ),
            FqxValue::DateTime(v) => v.try_cast(typ),
            FqxValue::Date(v) => v.try_cast(typ),
            FqxValue::Time(v) => v.try_cast(typ),
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
            FqxValue::Timestamp(v) => *self = v.clone().try_cast(typ)?,
            FqxValue::DateTime(v) => *self = v.clone().try_cast(typ)?,
            FqxValue::Date(v) => *self = v.clone().try_cast(typ)?,
            FqxValue::Time(v) => *self = v.clone().try_cast(typ)?,
            FqxValue::Null => *self = FqxValue::Null,
        };

        Ok(())
    }
}

// ================================================================================================
// Py
// ================================================================================================

#[pymethods]
impl FqxValueType {
    pub fn is_float(&self) -> bool {
        match self {
            FqxValueType::F32 => true,
            FqxValueType::F64 => true,
            _ => false,
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            FqxValueType::Bool => false,
            FqxValueType::String => false,
            FqxValueType::Blob => false,
            FqxValueType::Null => false,
            _ => true,
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
            FqxValueType::Timestamp => "FqxValueType::Timestamp",
            FqxValueType::DateTime => "FqxValueType::DateTime",
            FqxValueType::Date => "FqxValueType::Date",
            FqxValueType::Time => "FqxValueType::Time",
            FqxValueType::Null => "FqxValueType::Null",
        }
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_value {
    use super::*;

    #[test]
    fn partial_eq_success() {
        let a1 = FqxValue::I16(1) > FqxValue::F64(0.0);

        println!("{:?}", a1);
    }
}
