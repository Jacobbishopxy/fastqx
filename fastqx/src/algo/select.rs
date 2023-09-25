//! file: select.rs
//! author: Jacob Xie
//! date: 2023/09/25 15:16:03 Monday
//! brief:

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::algo::FqxSlice;

// ================================================================================================
// AlgoSelect
// ================================================================================================

pub trait AlgoSelect<'a, I>
where
    Self: 'a,
{
    type Ret<A>;

    fn select(&'a self, indices: &[usize]) -> Self::Ret<I>;
}

// ================================================================================================
// FqxSelect
// ================================================================================================

pub struct FqxRowSelect<'a>(pub(crate) Vec<&'a FqxValue>);

// ================================================================================================
// Impl
// ================================================================================================

impl<'a> AlgoSelect<'a, FqxRowSelect<'a>> for FqxRow {
    type Ret<A> = A;

    fn select(&'a self, indices: &[usize]) -> Self::Ret<FqxRowSelect<'a>> {
        let s = indices.iter().map(|i| &self[*i]).collect();
        FqxRowSelect(s)
    }
}

impl<'a> AlgoSelect<'a, FqxRowSelect<'a>> for FqxData {
    type Ret<A> = Vec<A>;

    fn select(&'a self, indices: &[usize]) -> Self::Ret<FqxRowSelect<'a>> {
        self.iter().map(|r| r.select(indices)).collect::<Vec<_>>()
    }
}

impl<'a> AlgoSelect<'a, FqxRowSelect<'a>> for FqxSlice {
    type Ret<A> = Vec<A>;

    fn select(&'a self, indices: &[usize]) -> Self::Ret<FqxRowSelect<'a>> {
        self.0.iter().map(|r| r.select(indices)).collect::<Vec<_>>()
    }
}
