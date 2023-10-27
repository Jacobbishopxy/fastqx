//! file: pyrow.rs
//! author: Jacob Xie
//! date: 2023/10/03 22:42:49 Tuesday
//! brief:

use anyhow::anyhow;
use pyo3::prelude::*;

use crate::adt::{FqxRow, FqxValue};

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

    fn __set__(&mut self, _instance: PyObject, value: Vec<FqxValue>) {
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

    fn __add__(&self, rhs: Self) -> Self {
        self.clone() + rhs
    }

    fn __sub__(&self, rhs: Self) -> Self {
        self.clone() - rhs
    }

    fn __mul__(&self, rhs: Self) -> Self {
        self.clone() * rhs
    }

    fn __truediv__(&self, rhs: Self) -> Self {
        self.clone() / rhs
    }

    fn __mod__(&self, rhs: Self) -> Self {
        self.clone() % rhs
    }

    #[pyo3(name = "to_json", text_signature = "($self)")]
    fn py_to_json(&self) -> PyResult<String> {
        Ok(serde_json::to_string(&self).map_err(|e| anyhow!(e))?)
    }
}
