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

use crate::adt::{FqxValue, FqxValueType};

// ================================================================================================
// FqxRowAbstract & FqxRowLike
// ================================================================================================

#[derive(RefCast, Debug, Clone, Serialize, Deserialize)]
#[repr(transparent)]
pub struct FqxRowAbstract<I, V>(pub(crate) I)
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>;

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a, I, V> AsRef<FqxRowAbstract<I, V>> for &'a FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
{
    fn as_ref(&self) -> &FqxRowAbstract<I, V> {
        &self
    }
}

impl<I, V, E> FromIterator<E> for FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    I: FromIterator<E>,
    V: Into<FqxValue>,
    E: Into<FqxRowAbstract<I, V>>,
{
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        FqxRowAbstract(iter.into_iter().collect())
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait FqxRowLike<I, V>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
{
    fn from_abstract(a: FqxRowAbstract<I, V>) -> Self;

    fn to_abstract(self) -> FqxRowAbstract<I, V>;

    fn as_abstract_ref(&self) -> &FqxRowAbstract<I, V>;

    fn as_abstract_mut(&mut self) -> &mut FqxRowAbstract<I, V>;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Index

impl<I, V> Index<usize> for FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    I: Index<usize, Output = V>,
    V: Into<FqxValue>,
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<I, V> IndexMut<usize> for FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    I: IndexMut<usize, Output = V>,
    V: Into<FqxValue>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! impl_index_range_for_abs {
    () => {
        impl<I, V> Index<RangeFull> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V>,
            I: Index<usize, Output = V> + Index<RangeFull, Output = [V]>,
            V: Into<FqxValue>,
        {
            type Output = [V];

            fn index(&self, index: RangeFull) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<I, V> IndexMut<RangeFull> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V>,
            I: IndexMut<usize, Output = V> + IndexMut<RangeFull, Output = [V]>,
            V: Into<FqxValue>,
        {
            fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
    ($t:ident) => {
        impl<I, V> Index<$t<usize>> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V>,
            I: Index<usize, Output = V> + Index<$t<usize>, Output = [V]>,
            V: Into<FqxValue>,
        {
            type Output = [V];

            fn index(&self, index: $t<usize>) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<I, V> IndexMut<$t<usize>> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V>,
            I: IndexMut<usize, Output = V> + IndexMut<$t<usize>, Output = [V]>,
            V: Into<FqxValue>,
        {
            fn index_mut(&mut self, index: $t<usize>) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
}

impl_index_range_for_abs!();
impl_index_range_for_abs!(Range);
impl_index_range_for_abs!(RangeFrom);
impl_index_range_for_abs!(RangeTo);
impl_index_range_for_abs!(RangeToInclusive);
impl_index_range_for_abs!(RangeInclusive);

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
// Test
// ================================================================================================

#[cfg(test)]
mod test_row {
    use super::*;

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
}
