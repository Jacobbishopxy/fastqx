//! file: row.rs
//! author: Jacob Xie
//! date: 2023/09/20 19:26:51 Wednesday
//! brief:

use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, RangeFrom, RangeFull,
    RangeInclusive, RangeTo, RangeToInclusive, Rem, RemAssign, Sub, SubAssign,
};

use anyhow::{bail, Result};
use itertools::{EitherOrBoth, Itertools};
use pyo3::prelude::*;
use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use super::util::{slice_vec, takes_vec};
use crate::adt::{FqxValue, FqxValueType, FromTo, RowProps, SeqSlice};

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

impl SeqSlice for FqxRow {
    fn empty() -> Self {
        FqxRow::default()
    }

    fn sliced<I>(self, range: I) -> Self
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
// impl RowProps
// ================================================================================================

impl RowProps for FqxRow {
    fn get_nth(&self, idx: usize) -> Option<&FqxValue> {
        self.0.get(idx)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn types(&self) -> Vec<FqxValueType> {
        self.0.iter().map(FqxValueType::from).collect()
    }

    fn to_values(self) -> Vec<FqxValue> {
        self.0
    }

    fn iter_owned(self) -> std::vec::IntoIter<FqxValue> {
        self.into_iter()
    }

    fn iter(&self) -> std::slice::Iter<'_, FqxValue> {
        self.into_iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, FqxValue> {
        self.into_iter()
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

impl<'a> From<Cow<'a, [FqxValue]>> for FqxRow {
    fn from(value: Cow<'a, [FqxValue]>) -> Self {
        match value {
            Cow::Borrowed(b) => FqxRow(b.to_vec()),
            Cow::Owned(o) => FqxRow(o),
        }
    }
}

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

impl<'a> IntoIterator for &'a mut FqxRow {
    type Item = &'a mut FqxValue;

    type IntoIter = std::slice::IterMut<'a, FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
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
// Arithmetic: FqxRow
// ================================================================================================

macro_rules! impl_arith_for_row {
    ($t:ident, $tf:tt, $ta:ident, $taf:tt, $op:tt, $opa:tt) => {
        impl $t for FqxRow {
            type Output = FqxRow;

            fn $tf(self, rhs: Self) -> Self::Output {
                let inner = self
                    .into_iter()
                    .zip_longest(rhs.into_iter())
                    .map(|pair| match pair {
                        EitherOrBoth::Both(l, r) => l $op r,
                        _ => FqxValue::Null,
                    })
                    .collect();

                FqxRow(inner)
            }
        }

        impl $ta for FqxRow {
            fn $taf(&mut self, rhs: Self) {
                self
                    .into_iter()
                    .zip_longest(rhs.into_iter())
                    .for_each(|pair| match pair {
                        EitherOrBoth::Both(l, r) => *l $opa r,
                        _ => {}
                    })
            }
        }
    };
}

impl_arith_for_row!(Add, add, AddAssign, add_assign, +, +=);
impl_arith_for_row!(Sub, sub, SubAssign, sub_assign, -, -=);
impl_arith_for_row!(Mul, mul, MulAssign, mul_assign, *, *=);
impl_arith_for_row!(Div, div, DivAssign, div_assign, /, /=);
impl_arith_for_row!(Rem, rem, RemAssign, rem_assign, %, %=);

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
