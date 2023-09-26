//! file: util.rs
//! author: Jacob Xie
//! date: 2023/09/23 22:28:21 Saturday
//! brief:

use anyhow::Result;

use crate::adt::{FqxValue, FqxValueType};

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
