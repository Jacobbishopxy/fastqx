//! file: refd.rs
//! author: Jacob Xie
//! date: 2023/10/12 22:50:44 Thursday
//! brief:

use crate::adt::ab::d::FqxD;
use crate::adt::{FqxValue, FqxValueType};
use crate::ops::FqxRowSelect;

// ================================================================================================
// FqxDataRef
// ================================================================================================

#[derive(Debug)]
pub struct FqxDataRef<'a> {
    pub columns: Vec<&'a String>,
    pub types: Vec<&'a FqxValueType>,
    pub data: Vec<FqxRowSelect<&'a FqxValue>>,
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

impl<'a> FqxD<&'a String, &'a FqxValueType, FqxRowSelect<&'a FqxValue>, &'a FqxValue>
    for FqxDataRef<'a>
{
    fn columns(&self) -> &[&'a String] {
        &self.columns
    }

    fn types(&self) -> &[&'a FqxValueType] {
        &self.types
    }

    fn data(&self) -> &[FqxRowSelect<&'a FqxValue>] {
        &self.data
    }

    fn dcst(
        self,
    ) -> (
        Vec<&'a String>,
        Vec<&'a FqxValueType>,
        Vec<FqxRowSelect<&'a FqxValue>>,
    ) {
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
        data: Vec<FqxRowSelect<&'a FqxValue>>,
    ) -> Self {
        Self {
            columns,
            types,
            data,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

// owned
pub struct FqxRII<'a> {
    inner: std::vec::IntoIter<FqxRowSelect<&'a FqxValue>>,
}

impl<'a> Iterator for FqxRII<'a> {
    type Item = FqxRowSelect<&'a FqxValue>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for FqxDataRef<'a> {
    type Item = FqxRowSelect<&'a FqxValue>;

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
    inner: std::slice::Iter<'b, FqxRowSelect<&'a FqxValue>>,
}

impl<'a, 'b> Iterator for FqxRRefII<'a, 'b>
where
    'a: 'b,
{
    type Item = &'b FqxRowSelect<&'a FqxValue>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, 'b> IntoIterator for &'b FqxDataRef<'a> {
    type Item = &'b FqxRowSelect<&'a FqxValue>;

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
    inner: std::slice::IterMut<'b, FqxRowSelect<&'a FqxValue>>,
}

impl<'a, 'b> Iterator for FqxRMutRefII<'a, 'b>
where
    'a: 'b,
{
    type Item = &'b mut FqxRowSelect<&'a FqxValue>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, 'b> IntoIterator for &'b mut FqxDataRef<'a>
where
    'a: 'b,
{
    type Item = &'b mut FqxRowSelect<&'a FqxValue>;

    type IntoIter = FqxRMutRefII<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRMutRefII {
            inner: self.data.iter_mut(),
        }
    }
}
