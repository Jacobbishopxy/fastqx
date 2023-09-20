//! file: row.rs
//! author: Jacob Xie
//! date: 2023/09/20 19:26:51 Wednesday
//! brief:

use std::ops::{Index, IndexMut};

use anyhow::anyhow;
use pyo3::prelude::*;
use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use super::{FqxData, FqxValue};

// ================================================================================================
// FqxRow
// ================================================================================================

#[pyclass]
#[derive(RefCast, Debug, Clone, Serialize, Deserialize)]
#[repr(transparent)]
pub struct FqxRow(pub(crate) Vec<FqxValue>);

// ================================================================================================
// Index
// No boundary check!
// ================================================================================================

impl Index<usize> for FqxData {
    type Output = FqxRow;

    fn index(&self, index: usize) -> &Self::Output {
        FqxRow::ref_cast(&self.data[index])
    }
}

impl IndexMut<usize> for FqxData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        FqxRow::ref_cast_mut(&mut self.data[index])
    }
}

impl Index<usize> for FqxRow {
    type Output = FqxValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for FqxRow {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.get_mut(index).unwrap()
    }
}

// ================================================================================================
// FqxRow py methods
// ================================================================================================

#[pymethods]
impl FqxRow {
    #[new]
    fn py_new(row: Vec<FqxValue>) -> Self {
        Self(row)
    }

    fn __get__(&self, _instance: PyObject, _owner: PyObject) -> Vec<FqxValue> {
        self.0.clone()
    }

    fn __set_(&mut self, _instance: PyObject, value: Vec<FqxValue>) {
        self.0 = value
    }

    fn __repr__(&self) -> PyResult<String> {
        self.py_to_json()
    }

    fn __getitem__(&self, idx: usize) -> FqxValue {
        self[idx].clone()
    }

    fn __setitem__(&mut self, idx: usize, val: FqxValue) {
        self[idx] = val;
    }

    #[pyo3(name = "to_json", text_signature = "($self)")]
    fn py_to_json(&self) -> PyResult<String> {
        Ok(serde_json::to_string(&self).map_err(|e| anyhow!(e))?)
    }
}
