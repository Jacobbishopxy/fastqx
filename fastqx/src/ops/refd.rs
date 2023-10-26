//! file: refd.rs
//! author: Jacob Xie
//! date: 2023/10/12 22:50:44 Thursday
//! brief:

use crate::adt::{FqxAffiliate, FqxD, FqxValue, FqxValueType};
use crate::ops::FqxRowRef;

// ================================================================================================
// FqxDataRef
// ================================================================================================

#[derive(Debug)]
pub struct FqxDataRef<'a> {
    pub columns: Vec<&'a String>,
    pub types: Vec<&'a FqxValueType>,
    pub data: Vec<FqxRowRef<'a>>,
}

impl<'a> FqxDataRef<'a> {
    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.columns.len()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> FqxD<&'a String, &'a FqxValueType, FqxRowRef<'a>, &'a FqxValue> for FqxDataRef<'a> {
    fn columns(&self) -> &[&'a String] {
        &self.columns
    }

    fn types(&self) -> &[&'a FqxValueType] {
        &self.types
    }

    fn data(&self) -> &[FqxRowRef<'a>] {
        &self.data
    }

    fn dcst(self) -> (Vec<&'a String>, Vec<&'a FqxValueType>, Vec<FqxRowRef<'a>>) {
        let FqxDataRef {
            columns,
            types,
            data,
        } = self;
        (columns, types, data)
    }

    fn cst(
        columns: Vec<&'a String>,
        types: Vec<&'a FqxValueType>,
        data: Vec<FqxRowRef<'a>>,
    ) -> Self {
        Self {
            columns,
            types,
            data,
        }
    }
}

impl<'a> FqxAffiliate<&'a String, &'a FqxValueType, FqxRowRef<'a>, &'a FqxValue>
    for FqxDataRef<'a>
{
}

///////////////////////////////////////////////////////////////////////////////////////////////////

// owned
pub struct FqxRII<'a> {
    inner: std::vec::IntoIter<FqxRowRef<'a>>,
}

impl<'a> Iterator for FqxRII<'a> {
    type Item = FqxRowRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for FqxDataRef<'a> {
    type Item = FqxRowRef<'a>;

    type IntoIter = FqxRII<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRII {
            inner: self.data.into_iter(),
        }
    }
}

// ref
pub struct FqxRRefII<'a, 'b>
where
    'a: 'b,
{
    inner: std::slice::Iter<'b, FqxRowRef<'a>>,
}

impl<'a, 'b> Iterator for FqxRRefII<'a, 'b>
where
    'a: 'b,
{
    type Item = &'b FqxRowRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, 'b> IntoIterator for &'b FqxDataRef<'a> {
    type Item = &'b FqxRowRef<'a>;

    type IntoIter = FqxRRefII<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRRefII {
            inner: self.data.iter(),
        }
    }
}

// ref mut
pub struct FqxRMutRefII<'a, 'b>
where
    'a: 'b,
{
    inner: std::slice::IterMut<'b, FqxRowRef<'a>>,
}

impl<'a, 'b> Iterator for FqxRMutRefII<'a, 'b>
where
    'a: 'b,
{
    type Item = &'b mut FqxRowRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, 'b> IntoIterator for &'b mut FqxDataRef<'a>
where
    'a: 'b,
{
    type Item = &'b mut FqxRowRef<'a>;

    type IntoIter = FqxRMutRefII<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRMutRefII {
            inner: self.data.iter_mut(),
        }
    }
}
