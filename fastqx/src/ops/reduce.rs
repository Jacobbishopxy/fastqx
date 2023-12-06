//! file: reduce.rs
//! author: Jacob Xie
//! date: 2023/09/25 17:16:50 Monday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxD, FqxValue};
use crate::ops::FqxGroup;

// ================================================================================================
// OpReduce
// ================================================================================================

pub trait OpReduce<const OWNED: bool> {
    type Item;
    type Ret<A>;

    fn reduce<F>(self, f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item;

    fn try_reduce<F>(self, f: F) -> Result<Self::Ret<Self::Item>>
    where
        F: FnMut(Self::Item, Self::Item) -> Result<Self::Item>;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// Generic T

impl<U> OpReduce<true> for U
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = Option<A>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        Iterator::reduce(self.data_take().into_iter(), |p, c| f(p, c))
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<Self::Item>>
    where
        F: FnMut(Self::Item, Self::Item) -> Result<Self::Item>,
    {
        let mut iter = self.data_take().into_iter();
        iter.next()
            .map(|ini| iter.try_fold(ini, |acc, c| f(acc, c)))
            .transpose()
    }
}

impl<'a, U> OpReduce<false> for &'a U
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = Option<A>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        Iterator::reduce(self.data().into_iter().cloned(), |p, c| f(p, c))
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<Self::Item>>
    where
        F: FnMut(Self::Item, Self::Item) -> Result<Self::Item>,
    {
        // try_reduce is not stable
        let mut iter = self.data().into_iter().cloned();
        iter.next()
            .map(|ini| iter.try_fold(ini, |acc, c| f(acc, c)))
            .transpose()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<U> OpReduce<true> for FqxGroup<U>
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = HashMap<Vec<FqxValue>, Option<A>>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = Iterator::reduce(v.data_take().into_iter(), |p, c| f(p, c));
            res.insert(k, a);
        }

        res
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<Self::Item>>
    where
        F: FnMut(Self::Item, Self::Item) -> Result<Self::Item>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let mut iter = v.data_take().into_iter();
            let a = iter
                .next()
                .map(|ini| iter.try_fold(ini, |acc, c| f(acc, c)))
                .transpose()?;

            res.insert(k, a);
        }

        Ok(res)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_reduce {
    use super::*;
    use crate::ops::mock::data::D2;
    use crate::ops::{OpGroup, OpSelect};

    #[test]
    fn reduce_self_success() {
        let data = D2.clone();

        let foo = (&data).reduce(|p, c| p + c);
        println!("{:?}", foo);

        let foo = data.reduce(|p, c| p + c);
        println!("{:?}", foo);
    }

    // #[test]
    // fn reduce_slice_success() {
    //     let data = D2.clone();

    //     let slice = &data[..];

    //     let foo = slice.reduce(|p, c| p + c);

    //     println!("{:?}", foo);
    // }

    #[test]
    fn reduce_group_success() {
        let data = D2.clone();

        // let foo = data
        //     .rf()
        //     .group_by_fn(|r| vec![r[0].clone()])
        //     .reduce(|p, c| p + c);
        // println!("{:?}", foo);

        let foo = data
            .group_by_fn(|r| vec![r[0].clone()])
            .reduce(|p, c| p + c);
        println!("{:?}", foo);
    }

    #[test]
    fn reduce_selected_success() {
        // let data = D2.clone();

        // let foo = data.select([0, 1].as_slice()).reduce(|p, c| p + c);
        // println!("{:?}", foo);
    }

    #[test]
    fn reduce_selected_group_success() {
        // let data = D2.clone();

        // let foo = data
        //     .select([0, 1].as_slice())
        //     .group_by_fn(|r| vec![r[0].clone()])
        //     .reduce(|p, c| p + c);
        // println!("{:?}", foo);
    }
}
