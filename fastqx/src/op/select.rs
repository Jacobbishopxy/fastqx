//! file: select.rs
//! author: Jacob Xie
//! date: 2023/09/25 15:16:03 Monday
//! brief:

use ref_cast::RefCast;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::op::FqxSlice;

// ================================================================================================
// OpSelect
// ================================================================================================

pub trait OpSelect<I> {
    type Ret<A>;

    fn select(self, indices: &[usize]) -> Self::Ret<I>;
}

// ================================================================================================
// FqxSelect
// ================================================================================================

#[derive(RefCast, Debug, Clone)]
#[repr(transparent)]
pub struct FqxRowSelect<A>(pub(crate) Vec<A>);

// ================================================================================================
// Impl
// ================================================================================================

impl OpSelect<FqxRowSelect<FqxValue>> for FqxRow {
    type Ret<A> = A;

    fn select(mut self, indices: &[usize]) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let s = indices
            .iter()
            .filter_map(|i| {
                let mut d = FqxValue::Null;
                self.0.get_mut(*i).map(|v| {
                    std::mem::swap(&mut d, v);
                    d
                })
            })
            .collect();
        FqxRowSelect(s)
    }
}

impl<'a> OpSelect<FqxRowSelect<&'a FqxValue>> for &'a FqxRow {
    type Ret<A> = A;

    fn select(self, indices: &[usize]) -> Self::Ret<FqxRowSelect<&'a FqxValue>> {
        let s = indices.iter().filter_map(|i| self.0.get(*i)).collect();
        FqxRowSelect(s)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl OpSelect<FqxRowSelect<FqxValue>> for FqxData {
    type Ret<A> = Vec<A>;

    fn select(self, indices: &[usize]) -> Self::Ret<FqxRowSelect<FqxValue>> {
        self.iter_owned()
            .map(|r| r.select(indices))
            .collect::<Vec<_>>()
    }
}

impl<'a> OpSelect<FqxRowSelect<&'a FqxValue>> for &'a FqxData {
    type Ret<A> = Vec<A>;

    fn select(self, indices: &[usize]) -> Self::Ret<FqxRowSelect<&'a FqxValue>> {
        self.iter().map(|r| r.select(indices)).collect::<Vec<_>>()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> OpSelect<FqxRowSelect<&'a FqxValue>> for &'a FqxSlice {
    type Ret<A> = Vec<A>;

    fn select(self, indices: &[usize]) -> Self::Ret<FqxRowSelect<&'a FqxValue>> {
        self.0.iter().map(|r| r.select(indices)).collect::<Vec<_>>()
    }
}
