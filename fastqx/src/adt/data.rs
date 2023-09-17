//! file: data.rs
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
// FastqxData
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxData", get_all)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FqxData {
    pub(crate) columns: Vec<String>,
    pub(crate) types: Vec<FqxValueType>,
    pub(crate) data: Vec<Vec<FqxValue>>,
}

impl FqxData {
    pub fn new(
        columns: Vec<String>,
        types: Vec<FqxValueType>,
        data: Vec<Vec<FqxValue>>,
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
                if matches!(e, FqxValue::Null) {
                    continue;
                }
                match &types[idx] {
                    FqxValueType::Bool => {
                        *e = FqxValue::Bool(bool::try_from(e.clone())?);
                    }
                    FqxValueType::U8 => {
                        *e = FqxValue::U8(u8::try_from(e.clone())?);
                    }
                    FqxValueType::U16 => {
                        *e = FqxValue::U16(u16::try_from(e.clone())?);
                    }
                    FqxValueType::U32 => {
                        *e = FqxValue::U32(u32::try_from(e.clone())?);
                    }
                    FqxValueType::U64 => {
                        *e = FqxValue::U64(u64::try_from(e.clone())?);
                    }
                    FqxValueType::I8 => {
                        *e = FqxValue::I8(i8::try_from(e.clone())?);
                    }
                    FqxValueType::I16 => {
                        *e = FqxValue::I16(i16::try_from(e.clone())?);
                    }
                    FqxValueType::I32 => {
                        *e = FqxValue::I32(i32::try_from(e.clone())?);
                    }
                    FqxValueType::I64 => {
                        *e = FqxValue::I64(i64::try_from(e.clone())?);
                    }
                    FqxValueType::F32 => {
                        *e = FqxValue::F32(f32::try_from(e.clone())?);
                    }
                    FqxValueType::F64 => {
                        *e = FqxValue::F64(f64::try_from(e.clone())?);
                    }
                    FqxValueType::String => {
                        *e = FqxValue::String(String::try_from(e.clone())?);
                    }
                    FqxValueType::Blob => {
                        *e = FqxValue::Blob(Vec::<u8>::try_from(e.clone())?);
                    }
                    FqxValueType::Null => {
                        // Do nothing
                    }
                }
            }
        }

        Ok(())
    }
}

#[pymethods]
impl FqxData {
    #[new]
    fn py_new(
        columns: Vec<String>,
        types: Vec<FqxValueType>,
        data: Vec<Vec<FqxValue>>,
    ) -> PyResult<Self> {
        Ok(FqxData::new(columns, types, data)?)
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
    fn py_from_csv(_cls: &PyType, path: String, type_hints: Vec<FqxValueType>) -> PyResult<Self> {
        Ok(csv_read_rd(path, &type_hints)?)
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
mod test_data {
    use super::*;

    #[test]
    fn fqxvalue_print() {
        let foo = FqxValue::F64(123.456);
        println!("{:?}", serde_json::to_string(&foo));

        let foo = FqxValue::Null;
        println!("{:?}", serde_json::to_string(&foo));
    }
}
