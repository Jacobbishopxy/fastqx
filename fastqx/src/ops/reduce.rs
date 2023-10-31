//! file: reduce.rs
//! author: Jacob Xie
//! date: 2023/09/25 17:16:50 Monday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxRowAbstract, FqxValue};
use crate::ops::FqxGroup;

// ================================================================================================
// OpReduce
// ================================================================================================

pub trait OpReduce<T> {
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

impl<I, V, T, E> OpReduce<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = Option<A>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        Iterator::reduce(self.into_iter(), |p, c| f(p, c))
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<Self::Item>>
    where
        F: FnMut(Self::Item, Self::Item) -> Result<Self::Item>,
    {
        let mut iter = self.into_iter();
        iter.next()
            .map(|ini| iter.try_fold(ini, |acc, c| f(acc, c)))
            .transpose()
    }
}

impl<'a, I, V, T, E> OpReduce<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>> + Clone,
{
    type Item = E;

    type Ret<A> = Option<A>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        Iterator::reduce(self.into_iter().cloned(), |p, c| f(p, c))
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<Self::Item>>
    where
        F: FnMut(Self::Item, Self::Item) -> Result<Self::Item>,
    {
        // try_reduce is not stable
        let mut iter = self.into_iter().cloned();
        iter.next()
            .map(|ini| iter.try_fold(ini, |acc, c| f(acc, c)))
            .transpose()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<I, V, T, E> OpReduce<FqxRowAbstract<I, V>> for FqxGroup<T>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>> + From<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = HashMap<Vec<FqxValue>, Option<A>>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = Iterator::reduce(v.into_iter(), |p, c| f(p, c));
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
            let mut iter = v.into_iter();
            let a = iter
                .next()
                .map(|ini| iter.try_fold(ini, |acc, c| f(acc, c)))
                .transpose()?;

            res.insert(k, a);
        }

        Ok(res)
    }
}

impl<'a, I, V, T, E> OpReduce<&'a FqxRowAbstract<I, V>> for &'a FqxGroup<T>
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: Into<FqxRowAbstract<I, V>> + From<FqxRowAbstract<I, V>> + Clone,
{
    type Item = E;

    type Ret<A> = HashMap<Vec<FqxValue>, Option<A>>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = Iterator::reduce(v.into_iter().cloned(), |p, c| f(p, c));
            res.insert(k.clone(), a);
        }

        res
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<Self::Item>>
    where
        F: FnMut(Self::Item, Self::Item) -> Result<Self::Item>,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let mut iter = v.into_iter().cloned();
            let a = iter
                .next()
                .map(|ini| iter.try_fold(ini, |acc, c| f(acc, c)))
                .transpose()?;

            res.insert(k.clone(), a);
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
    use crate::mock::data::D2;
    use crate::ops::{OpGroup, OpOwned, OpSelect};

    #[test]
    fn reduce_self_success() {
        let data = D2.clone();

        let foo = (&data).reduce(|p, c| p + c);
        println!("{:?}", foo);

        let foo = data.reduce(|p, c| p + c);
        println!("{:?}", foo);
    }

    #[test]
    fn reduce_slice_success() {
        let data = D2.clone();

        let slice = &data[..];

        let foo = slice.reduce(|p, c| p + c);

        println!("{:?}", foo);
    }

    #[test]
    fn reduce_group_success() {
        let data = D2.clone();

        let foo = data
            .rf()
            .group_by_fn(|r| vec![r[0].clone()])
            .to_owned()
            .reduce(|p, c| p + c);
        println!("{:?}", foo);

        let foo = data
            .group_by_fn(|r| vec![r[0].clone()])
            .reduce(|p, c| p + c);
        println!("{:?}", foo);
    }

    #[test]
    fn reduce_selected_success() {
        let data = D2.clone();

        let foo = data
            .select([0, 1].as_slice())
            .to_owned()
            .reduce(|p, c| p + c);
        println!("{:?}", foo);
    }

    #[test]
    fn reduce_selected_group_success() {
        let data = D2.clone();

        let foo = data
            .select([0, 1].as_slice())
            .to_owned()
            .group_by_fn(|r| vec![r[0].clone()])
            .reduce(|p, c| p + c);
        println!("{:?}", foo);
    }
}
