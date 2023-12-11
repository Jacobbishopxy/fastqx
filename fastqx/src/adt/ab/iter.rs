//! file: iter.rs
//! author: Jacob Xie
//! date: 2023/09/22 19:05:36 Friday
//! brief:

use crate::adt::{FqxData, FqxDataCow, FqxRow, FqxRowCow};

// ================================================================================================
// Iterate
// ================================================================================================

pub struct FqxII<E> {
    inner: std::vec::IntoIter<E>,
}

impl<E> Iterator for FqxII<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl IntoIterator for FqxData {
    type Item = FqxRow;

    type IntoIter = FqxII<FqxRow>;

    fn into_iter(self) -> Self::IntoIter {
        FqxII {
            inner: self.data.into_iter(),
        }
    }
}

impl<'a> IntoIterator for FqxDataCow<'a> {
    type Item = FqxRowCow<'a>;

    type IntoIter = FqxII<FqxRowCow<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        FqxII {
            inner: self.data.into_iter(),
        }
    }
}

// ================================================================================================
// IterRef
// ================================================================================================

pub struct FqxRefII<'a, E> {
    inner: std::slice::Iter<'a, E>,
}

impl<'a, E> Iterator for FqxRefII<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> IntoIterator for &'a FqxData {
    type Item = &'a FqxRow;

    type IntoIter = FqxRefII<'a, FqxRow>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRefII {
            inner: self.data.iter(),
        }
    }
}

impl<'a> IntoIterator for &'a FqxDataCow<'a> {
    type Item = &'a FqxRowCow<'a>;

    type IntoIter = FqxRefII<'a, FqxRowCow<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRefII {
            inner: self.data.iter(),
        }
    }
}

// ================================================================================================
// IterMutRef
// ================================================================================================

pub struct FqxMutRefII<'a, E> {
    inner: std::slice::IterMut<'a, E>,
}

impl<'a, E> Iterator for FqxMutRefII<'a, E> {
    type Item = &'a mut E;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> IntoIterator for &'a mut FqxData {
    type Item = &'a mut FqxRow;

    type IntoIter = FqxMutRefII<'a, FqxRow>;

    fn into_iter(self) -> Self::IntoIter {
        FqxMutRefII {
            inner: self.data.iter_mut(),
        }
    }
}

impl<'a> IntoIterator for &'a mut FqxDataCow<'a> {
    type Item = &'a mut FqxRowCow<'a>;

    type IntoIter = FqxMutRefII<'a, FqxRowCow<'a>>;

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
    pub fn iter_owned(self) -> FqxII<FqxRow> {
        self.into_iter()
    }

    pub fn iter(&self) -> FqxRefII<FqxRow> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> FqxMutRefII<FqxRow> {
        self.into_iter()
    }
}

impl<'a> FqxDataCow<'a> {
    pub fn iter_owned(self) -> FqxII<FqxRowCow<'a>> {
        self.into_iter()
    }

    pub fn iter(&'a self) -> FqxRefII<FqxRowCow<'a>> {
        self.into_iter()
    }

    pub fn iter_mut(&'a mut self) -> FqxMutRefII<FqxRowCow<'a>> {
        self.into_iter()
    }
}
