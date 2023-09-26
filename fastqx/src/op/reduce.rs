//! file: reduce.rs
//! author: Jacob Xie
//! date: 2023/09/25 17:16:50 Monday
//! brief:

use anyhow::{anyhow, Result};

use crate::adt::{FqxData, FqxRow};
use crate::op::FqxSlice;

// ================================================================================================
// OpReduce
// ================================================================================================

pub trait OpReduce<'a, II>
where
    Self: 'a,
{
    type Ret<A>;

    fn reduce<F>(&'a self, f: F) -> Self::Ret<II>
    where
        F: Fn(II, II) -> II;

    fn try_reduce<F>(&'a self, f: F) -> Result<Self::Ret<II>>
    where
        F: Fn(II, II) -> Result<II>;
}

pub trait OpReduceMut<'a, II>
where
    Self: 'a,
{
    type Ret<A>;

    fn reduce<F>(&'a mut self, f: F) -> Self::Ret<II>
    where
        F: FnMut(II, II) -> II;

    fn try_reduce<F>(&'a mut self, f: F) -> Result<Self::Ret<II>>
    where
        F: FnMut(II, II) -> Result<II>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<'a> OpReduce<'a, &'a FqxRow> for FqxData {
    type Ret<A> = Option<A>;

    fn reduce<F>(&'a self, f: F) -> Self::Ret<&'a FqxRow>
    where
        F: Fn(&'a FqxRow, &'a FqxRow) -> &'a FqxRow,
    {
        self.iter().reduce(f)
    }

    fn try_reduce<F>(&'a self, f: F) -> Result<Self::Ret<&'a FqxRow>>
    where
        F: Fn(&'a FqxRow, &'a FqxRow) -> Result<&'a FqxRow>,
    {
        // try_reduce is not stable
        let mut iter = self.iter();
        let first = iter.next().ok_or(anyhow!("data is empty"))?;
        let res = iter.try_fold(first, |acc, r| f(acc, r).ok());
        Ok(res)
    }
}

impl<'a> OpReduceMut<'a, &'a mut FqxRow> for FqxData {
    type Ret<A> = Option<A>;

    fn reduce<F>(&'a mut self, f: F) -> Self::Ret<&'a mut FqxRow>
    where
        F: FnMut(&'a mut FqxRow, &'a mut FqxRow) -> &'a mut FqxRow,
    {
        self.iter_mut().reduce(f)
    }

    fn try_reduce<F>(&'a mut self, mut f: F) -> Result<Self::Ret<&'a mut FqxRow>>
    where
        F: FnMut(&'a mut FqxRow, &'a mut FqxRow) -> Result<&'a mut FqxRow>,
    {
        let mut iter = self.iter_mut();
        let first = iter.next().ok_or(anyhow!("data is empty"))?;
        let res = iter.try_fold(first, |acc, r| f(acc, r).ok());
        Ok(res)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> OpReduce<'a, &'a FqxRow> for FqxSlice {
    type Ret<A> = Option<A>;

    fn reduce<F>(&'a self, f: F) -> Self::Ret<&'a FqxRow>
    where
        F: Fn(&'a FqxRow, &'a FqxRow) -> &'a FqxRow,
    {
        self.0.iter().reduce(f)
    }

    fn try_reduce<F>(&'a self, f: F) -> Result<Self::Ret<&'a FqxRow>>
    where
        F: Fn(&'a FqxRow, &'a FqxRow) -> Result<&'a FqxRow>,
    {
        let mut iter = self.0.iter();
        let first = iter.next().ok_or(anyhow!("data is empty"))?;
        let res = iter.try_fold(first, |acc, r| f(acc, r).ok());
        Ok(res)
    }
}

impl<'a> OpReduceMut<'a, &'a mut FqxRow> for FqxSlice {
    type Ret<A> = Option<A>;

    fn reduce<F>(&'a mut self, f: F) -> Self::Ret<&'a mut FqxRow>
    where
        F: FnMut(&'a mut FqxRow, &'a mut FqxRow) -> &'a mut FqxRow,
    {
        self.0.iter_mut().reduce(f)
    }

    fn try_reduce<F>(&'a mut self, mut f: F) -> Result<Self::Ret<&'a mut FqxRow>>
    where
        F: FnMut(&'a mut FqxRow, &'a mut FqxRow) -> Result<&'a mut FqxRow>,
    {
        let mut iter = self.0.iter_mut();
        let first = iter.next().ok_or(anyhow!("data is empty"))?;
        let res = iter.try_fold(first, |acc, r| f(acc, r).ok());
        Ok(res)
    }
}
