//! file: row.rs
//! author: Jacob Xie
//! date: 2023/09/20 19:26:51 Wednesday
//! brief:

use std::ops::{Index, IndexMut};

use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use super::{FqxData, FqxValue, FqxValueType};

// ================================================================================================
// FqxRow
// ================================================================================================

macro_rules! guard {
    ($s:expr, $i:expr) => {
        if $i >= $s.len() {
            return Err(anyhow!(format!("idx: {} out of boundary {}", $i, $s.len())));
        }
    };
}

#[pyclass]
#[derive(RefCast, Debug, Clone, Serialize, Deserialize)]
#[repr(transparent)]
pub struct FqxRow(pub(crate) Vec<FqxValue>);

impl FqxRow {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn uncheck_cast(&mut self, idx: usize, typ: &FqxValueType) -> Result<()> {
        self[idx].try_cast_mut(typ)?;
        Ok(())
    }

    pub fn cast(&mut self, idx: usize, typ: &FqxValueType) -> Result<()> {
        guard!(self, idx);

        self.uncheck_cast(idx, typ)
    }

    pub fn uncheck_apply(
        &mut self,
        idx: usize,
        apply_fn: &dyn Fn(&mut FqxValue) -> Result<()>,
    ) -> Result<()> {
        apply_fn(&mut self[idx])?;

        Ok(())
    }

    pub fn apply(
        &mut self,
        idx: usize,
        apply_fn: &dyn Fn(&mut FqxValue) -> Result<()>,
    ) -> Result<()> {
        guard!(self, idx);

        self.uncheck_apply(idx, apply_fn)
    }
}

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

    #[pyo3(name = "to_json", text_signature = "($self)")]
    fn py_to_json(&self) -> PyResult<String> {
        Ok(serde_json::to_string(&self).map_err(|e| anyhow!(e))?)
    }
}
