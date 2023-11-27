//! file: row.rs
//! author: Jacob Xie
//! date: 2023/09/20 19:26:51 Wednesday
//! brief:

use std::collections::HashMap;
use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use anyhow::{bail, Result};
use itertools::Itertools;
use pyo3::prelude::*;
use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use super::util::{slice_vec, takes_vec};
use crate::adt::{FqxRowAbstract, FqxRowLike, FqxSlice, FqxValue, FqxValueType, FromTo};

// ================================================================================================
// FqxRow
// ================================================================================================

macro_rules! guard {
    ($s:expr, $i:expr) => {
        if $i >= $s.len() {
            bail!(format!("idx: {} out of boundary {}", $i, $s.len()));
        }
    };
}

#[pyclass]
#[derive(
    RefCast, Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord,
)]
#[repr(transparent)]
pub struct FqxRow(pub(crate) Vec<FqxValue>);

impl FqxRow {
    pub fn new(d: Vec<FqxValue>) -> Self {
        FqxRow(d)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn types(&self) -> Vec<FqxValueType> {
        self.0.iter().map(FqxValueType::from).collect()
    }

    pub fn to_values(self) -> Vec<FqxValue> {
        self.0
    }

    pub fn data(&self) -> &Vec<FqxValue> {
        &self.0
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

    pub fn select(&self, idx: &[usize]) -> Vec<&FqxValue> {
        idx.into_iter().fold(vec![], |mut acc, i| {
            if let Some(e) = self.0.get(*i) {
                acc.push(e);
            }
            acc
        })
    }

    pub fn select_owned(&self, idx: &[usize]) -> Vec<FqxValue> {
        idx.into_iter().fold(vec![], |mut acc, i| {
            if let Some(e) = self.0.get(*i) {
                acc.push(e.clone());
            }
            acc
        })
    }

    pub fn select_mut(&mut self, d: HashMap<usize, FqxValue>) {
        let len = self.len();
        let typs = self.types();
        for (k, v) in d.into_iter() {
            if k <= len && typs[k] == FqxValueType::from(&v) {
                self.0.get_mut(k).map(|e| *e = v);
            }
        }
    }
}

// ================================================================================================
// impl SliceRow
// ================================================================================================

impl FqxSlice for FqxRow {
    fn slice<I>(self, range: I) -> Self
    where
        I: FromTo,
    {
        FqxRow(slice_vec(self.0, range))
    }

    fn takes<I>(self, indices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        FqxRow(takes_vec(self.0, indices))
    }
}

// ================================================================================================
// FqxRow -> FqxRowLike
// ================================================================================================

impl AsRef<FqxRowAbstract<Vec<FqxValue>, FqxValue>> for FqxRow {
    fn as_ref(&self) -> &FqxRowAbstract<Vec<FqxValue>, FqxValue> {
        FqxRowAbstract::ref_cast(&self.0)
    }
}

impl AsMut<FqxRowAbstract<Vec<FqxValue>, FqxValue>> for FqxRow {
    fn as_mut(&mut self) -> &mut FqxRowAbstract<Vec<FqxValue>, FqxValue> {
        FqxRowAbstract::ref_cast_mut(&mut self.0)
    }
}

impl Into<FqxRowAbstract<Vec<FqxValue>, FqxValue>> for FqxRow {
    fn into(self) -> FqxRowAbstract<Vec<FqxValue>, FqxValue> {
        FqxRowAbstract(self.0)
    }
}

impl From<FqxRowAbstract<Vec<FqxValue>, FqxValue>> for FqxRow {
    fn from(value: FqxRowAbstract<Vec<FqxValue>, FqxValue>) -> Self {
        FqxRow(value.0)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl FqxRowLike<Vec<FqxValue>, FqxValue> for FqxRow {
    fn from_abstract(a: FqxRowAbstract<Vec<FqxValue>, FqxValue>) -> Self {
        FqxRow(a.0)
    }

    fn to_abstract(self) -> FqxRowAbstract<Vec<FqxValue>, FqxValue> {
        FqxRowAbstract(self.0)
    }

    fn as_abstract_ref(&self) -> &FqxRowAbstract<Vec<FqxValue>, FqxValue> {
        self.as_ref()
    }

    fn as_abstract_mut(&mut self) -> &mut FqxRowAbstract<Vec<FqxValue>, FqxValue> {
        self.as_mut()
    }
}

// ================================================================================================
// FqxRow: AsRef & AsMut & From
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

impl From<Vec<FqxValue>> for FqxRow {
    fn from(value: Vec<FqxValue>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a FqxRow> for FqxRow {
    fn from(value: &'a FqxRow) -> Self {
        value.clone()
    }
}

// impl ToOwned<>

// ================================================================================================
// FqxRow: IntoIterator & FromIterator, Extend
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

///////////////////////////////////////////////////////////////////////////////////////////////////

impl Extend<FqxValue> for FqxRow {
    fn extend<T: IntoIterator<Item = FqxValue>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter())
    }
}

// ================================================================================================
// FqxRow: Index<usize>
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

macro_rules! impl_index_range {
    () => {
        impl Index<RangeFull> for FqxRow {
            type Output = [FqxValue];

            fn index(&self, index: RangeFull) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<RangeFull> for FqxRow {
            fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
    ($t:ident) => {
        impl Index<$t<usize>> for FqxRow {
            type Output = [FqxValue];

            fn index(&self, index: $t<usize>) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<$t<usize>> for FqxRow {
            fn index_mut(&mut self, index: $t<usize>) -> &mut Self::Output {
                &mut self.0[index]
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
// Py
// ================================================================================================

#[pymethods]
impl FqxRow {
    #[new]
    fn __new__(row: Vec<FqxValue>) -> Self {
        Self(row)
    }

    fn __get__(&self, _instance: PyObject, _owner: PyObject) -> Vec<FqxValue> {
        self.0.clone()
    }

    fn __set__(&mut self, _instance: PyObject, value: Vec<FqxValue>) {
        self.0 = value
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(serde_json::to_string_pretty(&self).map_err(anyhow::Error::msg)?)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(serde_json::to_string(&self).map_err(anyhow::Error::msg)?)
    }

    fn __getitem__(&self, idx: isize) -> FqxValue {
        self[idx as usize].clone()
    }

    fn __setitem__(&mut self, idx: isize, val: FqxValue) {
        self[idx as usize] = val;
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

    fn __len__(&self) -> usize {
        self.len()
    }

    fn __contains__(&self, object: FqxValue) -> bool {
        self.0.iter().contains(&object)
    }

    fn __concat__(&self, other: FqxRow) -> FqxRow {
        let mut res = self.clone();
        res.extend(other);
        res
    }

    fn __inplace_concat__(&self, other: FqxRow) -> FqxRow {
        let mut res = self.clone();
        res.extend(other);
        res
    }

    #[pyo3(name = "to_str", text_signature = "($self)")]
    fn py_to_str(&self) -> PyResult<String> {
        self.__str__()
    }

    #[pyo3(name = "cast")]
    fn py_cast(&mut self, idx: usize, typ: String) -> PyResult<()> {
        let typ = &FqxValueType::try_from(typ)?;
        Ok(self.cast(idx, typ)?)
    }

    #[pyo3(name = "types")]
    fn py_types(&self) -> Vec<FqxValueType> {
        self.types()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_row {
    use super::*;
    use crate::fqx;

    #[test]
    fn as_abstract_success() {
        let foo = FqxRow(vec![
            FqxValue::Null,
            FqxValue::I16(0),
            FqxValue::String("ha".to_string()),
        ]);

        let bar = foo.as_abstract_ref();

        println!("{:?}", bar[0]);
    }

    #[test]
    fn as_abstract_arith_success() {
        let mut a1 = FqxRow(vec![
            FqxValue::F32(0.1),
            FqxValue::I16(0),
            FqxValue::String("ha".to_string()),
        ]);

        let a2 = FqxRow(vec![
            FqxValue::F32(0.2),
            FqxValue::I16(0),
            FqxValue::String("!".to_string()),
        ]);

        *a1.as_abstract_mut() += a2.to_abstract();

        println!("{:?}", a1);
    }

    #[test]
    fn select_mut_success() {
        let mut foo = fqx!(1, 0, "a", 2.1, 21);

        let rpc: HashMap<usize, FqxValue> = HashMap::from_iter([(0, fqx!(2)), (2, fqx!("c"))]);
        foo.select_mut(rpc);
        println!("{:?}", foo);
    }

    #[test]
    fn index_success() {
        let foo = fqx!(1, 0, "a", 2.1, 21);

        println!("{:?}", foo[0]);
        println!("{:?}", foo[1]);
        println!("{:?}", foo[2]);
        println!("{:?}", foo[3]);
        println!("{:?}", foo[4]);
    }
}
