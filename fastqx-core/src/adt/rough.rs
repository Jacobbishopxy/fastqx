//! file: rough.rs
//! author: Jacob Xie
//! date: 2023/09/11 08:54:05 Monday
//! brief: for both dynamic query and Pyo3

use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use super::value::*;

// ================================================================================================
// RoughData
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxData", get_all)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoughData {
    pub(crate) columns: Vec<String>,
    pub(crate) types: Vec<RoughValueType>,
    pub(crate) data: Vec<Vec<RoughValue>>,
}

impl RoughData {
    pub fn new(
        columns: Vec<String>,
        types: Vec<RoughValueType>,
        data: Vec<Vec<RoughValue>>,
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
                if matches!(e, RoughValue::Null) {
                    continue;
                }
                match &types[idx] {
                    RoughValueType::Bool => {
                        *e = RoughValue::Bool(bool::try_from(e.clone())?);
                    }
                    RoughValueType::U8 => {
                        *e = RoughValue::U8(u8::try_from(e.clone())?);
                    }
                    RoughValueType::U16 => {
                        *e = RoughValue::U16(u16::try_from(e.clone())?);
                    }
                    RoughValueType::U32 => {
                        *e = RoughValue::U32(u32::try_from(e.clone())?);
                    }
                    RoughValueType::U64 => {
                        *e = RoughValue::U64(u64::try_from(e.clone())?);
                    }
                    RoughValueType::I8 => {
                        *e = RoughValue::I8(i8::try_from(e.clone())?);
                    }
                    RoughValueType::I16 => {
                        *e = RoughValue::I16(i16::try_from(e.clone())?);
                    }
                    RoughValueType::I32 => {
                        *e = RoughValue::I32(i32::try_from(e.clone())?);
                    }
                    RoughValueType::I64 => {
                        *e = RoughValue::I64(i64::try_from(e.clone())?);
                    }
                    RoughValueType::F32 => {
                        *e = RoughValue::F32(f32::try_from(e.clone())?);
                    }
                    RoughValueType::F64 => {
                        *e = RoughValue::F64(f64::try_from(e.clone())?);
                    }
                    RoughValueType::String => {
                        *e = RoughValue::String(String::try_from(e.clone())?);
                    }
                    RoughValueType::Blob => {
                        *e = RoughValue::Blob(Vec::<u8>::try_from(e.clone())?);
                    }
                    RoughValueType::Null => {
                        // Do nothing
                    }
                }
            }
        }

        Ok(())
    }
}

#[pymethods]
impl RoughData {
    #[new]
    fn py_new(
        columns: Vec<String>,
        types: Vec<RoughValueType>,
        data: Vec<Vec<RoughValue>>,
    ) -> PyResult<Self> {
        Ok(RoughData::new(columns, types, data)?)
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
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_rough {
    use super::*;

    #[test]
    fn rough_value_print() {
        let foo = RoughValue::F64(123.456);
        println!("{:?}", serde_json::to_string(&foo));

        let foo = RoughValue::Null;
        println!("{:?}", serde_json::to_string(&foo));
    }
}
