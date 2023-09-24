//! file: iter.rs
//! author: Jacob Xie
//! date: 2023/09/22 19:05:36 Friday
//! brief:

use ref_cast::RefCast;

use crate::adt::{FqxData, FqxRow, FqxValue};

// ================================================================================================
// Iterate
// ================================================================================================

pub struct FqxII {
    inner: std::vec::IntoIter<Vec<FqxValue>>,
}

impl Iterator for FqxII {
    type Item = FqxRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(FqxRow)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl IntoIterator for FqxData {
    type Item = FqxRow;

    type IntoIter = FqxII;

    fn into_iter(self) -> Self::IntoIter {
        FqxII {
            inner: self.data.into_iter(),
        }
    }
}

// ================================================================================================
// IterRef
// ================================================================================================

pub struct FqxRefII<'a> {
    inner: std::slice::Iter<'a, Vec<FqxValue>>,
}

impl<'a> Iterator for FqxRefII<'a> {
    type Item = &'a FqxRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(FqxRow::ref_cast)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> IntoIterator for &'a FqxData {
    type Item = &'a FqxRow;

    type IntoIter = FqxRefII<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRefII {
            inner: self.data.iter(),
        }
    }
}

// ================================================================================================
// IterMutRef
// ================================================================================================

pub struct FqxMutRefII<'a> {
    inner: std::slice::IterMut<'a, Vec<FqxValue>>,
}

impl<'a> Iterator for FqxMutRefII<'a> {
    type Item = &'a mut FqxRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(FqxRow::ref_cast_mut)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> IntoIterator for &'a mut FqxData {
    type Item = &'a mut FqxRow;

    type IntoIter = FqxMutRefII<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FqxMutRefII {
            inner: self.data.iter_mut(),
        }
    }
}

// ================================================================================================
// impl FqxData
// ================================================================================================

impl FqxData {
    pub fn iter_owned(self) -> FqxII {
        self.into_iter()
    }

    pub fn iter(&self) -> FqxRefII {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> FqxMutRefII {
        self.into_iter()
    }
}
