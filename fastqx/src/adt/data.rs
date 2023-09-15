//! file: rough.rs
//! author: Jacob Xie
//! date: 2023/09/11 08:54:05 Monday
//! brief: for both dynamic query and Pyo3

use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use pyo3::types::PyType;
use serde::{Deserialize, Serialize};

use super::value::*;
use crate::csv::*;

// ================================================================================================
// RoughData
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxData", get_all)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastqxData {
    pub(crate) columns: Vec<String>,
    pub(crate) types: Vec<FastqxValueType>,
    pub(crate) data: Vec<Vec<FastqxValue>>,
}

impl FastqxData {
    pub fn new(
        columns: Vec<String>,
        types: Vec<FastqxValueType>,
        data: Vec<Vec<FastqxValue>>,
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
                if matches!(e, FastqxValue::Null) {
                    continue;
                }
                match &types[idx] {
                    FastqxValueType::Bool => {
                        *e = FastqxValue::Bool(bool::try_from(e.clone())?);
                    }
                    FastqxValueType::U8 => {
                        *e = FastqxValue::U8(u8::try_from(e.clone())?);
                    }
                    FastqxValueType::U16 => {
                        *e = FastqxValue::U16(u16::try_from(e.clone())?);
                    }
                    FastqxValueType::U32 => {
                        *e = FastqxValue::U32(u32::try_from(e.clone())?);
                    }
                    FastqxValueType::U64 => {
                        *e = FastqxValue::U64(u64::try_from(e.clone())?);
                    }
                    FastqxValueType::I8 => {
                        *e = FastqxValue::I8(i8::try_from(e.clone())?);
                    }
                    FastqxValueType::I16 => {
                        *e = FastqxValue::I16(i16::try_from(e.clone())?);
                    }
                    FastqxValueType::I32 => {
                        *e = FastqxValue::I32(i32::try_from(e.clone())?);
                    }
                    FastqxValueType::I64 => {
                        *e = FastqxValue::I64(i64::try_from(e.clone())?);
                    }
                    FastqxValueType::F32 => {
                        *e = FastqxValue::F32(f32::try_from(e.clone())?);
                    }
                    FastqxValueType::F64 => {
                        *e = FastqxValue::F64(f64::try_from(e.clone())?);
                    }
                    FastqxValueType::String => {
                        *e = FastqxValue::String(String::try_from(e.clone())?);
                    }
                    FastqxValueType::Blob => {
                        *e = FastqxValue::Blob(Vec::<u8>::try_from(e.clone())?);
                    }
                    FastqxValueType::Null => {
                        // Do nothing
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn rough_data_from_csv_(path: String, type_hints: Vec<String>) -> Result<FastqxData> {
    let type_hints = type_hints
        .iter()
        .map(|t| match t.as_str() {
            "Bool" => FastqxValueType::Bool,
            "U8" => FastqxValueType::U8,
            "U16" => FastqxValueType::U16,
            "U32" => FastqxValueType::U32,
            "U64" => FastqxValueType::U64,
            "I8" => FastqxValueType::I8,
            "I16" => FastqxValueType::I16,
            "I32" => FastqxValueType::I32,
            "I64" => FastqxValueType::I64,
            "F32" => FastqxValueType::F32,
            "F64" => FastqxValueType::F64,
            "String" => FastqxValueType::String,
            "Blob" => FastqxValueType::Blob,
            "Null" => FastqxValueType::Null,
            _ => FastqxValueType::String,
        })
        .collect::<Vec<_>>();

    Ok(csv_read_rd(path, &type_hints)?)
}

#[pymethods]
impl FastqxData {
    #[new]
    fn py_new(
        columns: Vec<String>,
        types: Vec<FastqxValueType>,
        data: Vec<Vec<FastqxValue>>,
    ) -> PyResult<Self> {
        Ok(FastqxData::new(columns, types, data)?)
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

    #[classmethod]
    #[pyo3(name = "from_csv", text_signature = "(path, type_hints)")]
    fn py_from_csv(_cls: &PyType, path: String, type_hints: Vec<String>) -> PyResult<Self> {
        Ok(rough_data_from_csv_(path, type_hints)?)
    }

    #[pyo3(name = "to_csv", text_signature = "(path, type_hints)")]
    fn py_to_csv(&self, path: String) -> PyResult<()> {
        Ok(csv_write_rd(&self, path)?)
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
        let foo = FastqxValue::F64(123.456);
        println!("{:?}", serde_json::to_string(&foo));

        let foo = FastqxValue::Null;
        println!("{:?}", serde_json::to_string(&foo));
    }
}
