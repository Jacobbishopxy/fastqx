//! file: pyvalue.rs
//! author: Jacob Xie
//! date: 2023/10/03 22:44:00 Tuesday
//! brief:

use pyo3::prelude::*;

use crate::adt::FqxValueType;

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

    #[pyo3(name = "is_numeric", text_signature = "($self)")]
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
