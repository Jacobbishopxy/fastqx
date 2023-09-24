//! file: fold.rs
//! author: Jacob Xie
//! date: 2023/09/24 18:50:53 Sunday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::algo::{FqxGroup, FqxGroupMut, FqxSlice};

// ================================================================================================
// AlgoFold
// ================================================================================================

pub trait AlgoFold<'a>
where
    Self: 'a,
{
    type IterItem;
    type Ret<A>;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> A;

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> Result<A>;
}

pub trait AlgoFoldMut<'a>
where
    Self: 'a,
{
    type IterItem;
    type Ret<A>;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> A;

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> Result<A>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<'a> AlgoFold<'a> for FqxData {
    type IterItem = &'a FqxRow;
    type Ret<A> = A;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, Self::IterItem) -> A,
    {
        self.iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        self.iter().try_fold(accumulator, f)
    }
}

impl<'a> AlgoFoldMut<'a> for FqxData {
    type IterItem = &'a mut FqxRow;
    type Ret<A> = A;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, Self::IterItem) -> A,
    {
        self.iter_mut().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        self.iter_mut().try_fold(accumulator, f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> AlgoFold<'a> for FqxSlice {
    type IterItem = &'a FqxRow;
    type Ret<A> = A;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, Self::IterItem) -> A,
    {
        self.0.iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        self.0.iter().try_fold(accumulator, f)
    }
}

impl<'a> AlgoFoldMut<'a> for FqxSlice {
    type IterItem = &'a mut FqxRow;
    type Ret<A> = A;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, Self::IterItem) -> A,
    {
        self.0.iter_mut().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        self.0.iter_mut().try_fold(accumulator, f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> AlgoFold<'a> for FqxGroup<'a> {
    type IterItem = &'a FqxRow;
    type Ret<A> = HashMap<FqxValue, A>;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let acc = v.iter().fold(accumulator.clone(), |acc, r| f(acc, *r));

            res.insert(k.clone(), acc);
        }

        res
    }

    fn try_fold<A, F>(&self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let acc = v
                .iter()
                .try_fold(accumulator.clone(), |acc, r| f(acc, *r))?;

            res.insert(k.clone(), acc);
        }

        Ok(res)
    }
}

impl<'a> AlgoFoldMut<'a> for FqxGroupMut<'a> {
    type IterItem = &'a mut FqxRow;

    type Ret<A> = HashMap<FqxValue, A>;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter_mut() {
            let acc = v.iter_mut().fold(accumulator.clone(), |acc, r| f(acc, *r));

            res.insert(k.clone(), acc);
        }

        res
    }

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter_mut() {
            let acc = v
                .iter_mut()
                .try_fold(accumulator.clone(), |acc, r| f(acc, *r))?;

            res.insert(k.clone(), acc);
        }

        Ok(res)
    }
}
