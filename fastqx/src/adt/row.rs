//! file: row.rs
//! author: Jacob Xie
//! date: 2023/09/20 19:26:51 Wednesday
//! brief:

use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use super::{FqxValue, FqxValueType};

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

    pub fn uncheck_apply<F>(&mut self, idx: usize, f: F) -> Result<()>
    where
        F: Fn(&mut FqxValue) -> Result<()>,
    {
        f(&mut self[idx])?;

        Ok(())
    }

    pub fn apply<F>(&mut self, idx: usize, f: F) -> Result<()>
    where
        F: Fn(&mut FqxValue) -> Result<()>,
    {
        guard!(self, idx);

        self.uncheck_apply(idx, f)
    }
}

// ================================================================================================
// AsRef & AsMut
// ================================================================================================

impl AsRef<Vec<FqxValue>> for FqxRow {
    fn as_ref(&self) -> &Vec<FqxValue> {
        &self.0
    }
}

impl AsRef<FqxRow> for Vec<FqxValue> {
    fn as_ref(&self) -> &FqxRow {
        FqxRow::ref_cast(self)
    }
}

impl AsMut<Vec<FqxValue>> for FqxRow {
    fn as_mut(&mut self) -> &mut Vec<FqxValue> {
        &mut self.0
    }
}

impl AsMut<FqxRow> for Vec<FqxValue> {
    fn as_mut(&mut self) -> &mut FqxRow {
        FqxRow::ref_cast_mut(self)
    }
}

// ================================================================================================
// IntoIterator & FromIterator
// ================================================================================================

impl IntoIterator for FqxRow {
    type Item = FqxValue;

    type IntoIter = std::vec::IntoIter<FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<FqxValue> for FqxRow {
    fn from_iter<T: IntoIterator<Item = FqxValue>>(iter: T) -> Self {
        FqxRow(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a FqxRow {
    type Item = &'a FqxValue;

    type IntoIter = std::slice::Iter<'a, FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// ================================================================================================
// FqxRowSlice
// ================================================================================================

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct FqxRowSlice(pub(crate) [FqxValue]);

// ================================================================================================
// Index<usize>
// No boundary check!
// ================================================================================================

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

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_index_range {
    () => {
        impl Index<RangeFull> for FqxRow {
            type Output = FqxRowSlice;

            fn index(&self, index: RangeFull) -> &Self::Output {
                FqxRowSlice::ref_cast(&self.0[index])
            }
        }

        impl IndexMut<RangeFull> for FqxRow {
            fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
                FqxRowSlice::ref_cast_mut(&mut self.0[index])
            }
        }
    };
    ($t:ident) => {
        impl Index<$t<usize>> for FqxRow {
            type Output = FqxRowSlice;

            fn index(&self, index: $t<usize>) -> &Self::Output {
                FqxRowSlice::ref_cast(&self.0[index])
            }
        }

        impl IndexMut<$t<usize>> for FqxRow {
            fn index_mut(&mut self, index: $t<usize>) -> &mut Self::Output {
                FqxRowSlice::ref_cast_mut(&mut self.0[index])
            }
        }
    };
}

impl_index_range!();
impl_index_range!(Range);
impl_index_range!(RangeFrom);
impl_index_range!(RangeTo);
impl_index_range!(RangeToInclusive);
impl_index_range!(RangeInclusive);

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
