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

pub trait OpReduce<I> {
    type In;

    fn reduce<F>(self, f: F) -> Option<Self::In>
    where
        F: FnMut(Self::In, Self::In) -> Self::In;

    fn try_reduce<F>(self, f: F) -> Result<Option<Self::In>>
    where
        F: FnMut(Self::In, Self::In) -> Result<Self::In>;
}

pub trait OpReduceGroup<I> {
    type In;
    type Ret<A>;

    fn reduce<F>(self, f: F) -> Self::Ret<Self::In>
    where
        F: FnMut(Self::In, Self::In) -> Self::In;

    fn try_reduce<F>(self, f: F) -> Result<Self::Ret<Self::In>>
    where
        F: FnMut(Self::In, Self::In) -> Result<Self::In>;
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
    E: Into<FqxRowAbstract<I, V>> + From<FqxRowAbstract<I, V>>,
{
    type In = FqxRowAbstract<I, V>;

    fn reduce<F>(self, mut f: F) -> Option<FqxRowAbstract<I, V>>
    where
        F: FnMut(Self::In, Self::In) -> Self::In,
    {
        Iterator::reduce(self.into_iter(), |p, c| f(p.into(), c.into()).into()).map(E::into)
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Option<FqxRowAbstract<I, V>>>
    where
        F: FnMut(Self::In, Self::In) -> Result<Self::In>,
    {
        // try_reduce is not stable
        let mut iter = self.into_iter();
        iter.next()
            .map(|ini| iter.try_fold(ini, |acc, c| f(acc.into(), c.into()).map(E::from)))
            .transpose()
            .map(|o| o.map(E::into))
    }
}

impl<'a, I, V, T, E> OpReduce<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: Into<FqxRowAbstract<I, V>> + From<FqxRowAbstract<I, V>> + Clone,
{
    type In = FqxRowAbstract<I, V>;

    fn reduce<F>(self, mut f: F) -> Option<FqxRowAbstract<I, V>>
    where
        F: FnMut(FqxRowAbstract<I, V>, FqxRowAbstract<I, V>) -> FqxRowAbstract<I, V>,
    {
        Iterator::reduce(self.into_iter().cloned(), |p, c| {
            f(p.into(), c.into()).into()
        })
        .map(E::into)
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Option<FqxRowAbstract<I, V>>>
    where
        F: FnMut(FqxRowAbstract<I, V>, FqxRowAbstract<I, V>) -> Result<FqxRowAbstract<I, V>>,
    {
        // try_reduce is not stable
        let mut iter = self.into_iter().cloned();
        iter.next()
            .map(|ini| iter.try_fold(ini, |acc, c| f(acc.into(), c.into()).map(E::from)))
            .transpose()
            .map(|o| o.map(E::into))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup

impl<I, V, T, E> OpReduceGroup<FqxRowAbstract<I, V>> for FqxGroup<T>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>> + From<FqxRowAbstract<I, V>>,
{
    type In = FqxRowAbstract<I, V>;

    type Ret<A> = HashMap<Vec<FqxValue>, Option<A>>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<Self::In>
    where
        F: FnMut(Self::In, Self::In) -> Self::In,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a =
                Iterator::reduce(v.into_iter(), |p, c| f(p.into(), c.into()).into()).map(E::into);
            res.insert(k, a);
        }

        res
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<Self::In>>
    where
        F: FnMut(Self::In, Self::In) -> Result<Self::In>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let mut iter = v.into_iter();
            let a = iter
                .next()
                .map(|ini| iter.try_fold(ini, |acc, c| f(acc.into(), c.into()).map(E::from)))
                .transpose()
                .map(|o| o.map(E::into))?;

            res.insert(k, a);
        }

        Ok(res)
    }
}

impl<'a, I, V, T, E> OpReduceGroup<&'a FqxRowAbstract<I, V>> for &'a FqxGroup<T>
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: Into<FqxRowAbstract<I, V>> + From<FqxRowAbstract<I, V>> + Clone,
{
    type In = FqxRowAbstract<I, V>;

    type Ret<A> = HashMap<Vec<FqxValue>, Option<A>>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<Self::In>
    where
        F: FnMut(Self::In, Self::In) -> Self::In,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = Iterator::reduce(v.into_iter().cloned(), |p, c| f(p.into(), c.into()).into())
                .map(E::into);
            res.insert(k.clone(), a);
        }

        res
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<Self::In>>
    where
        F: FnMut(Self::In, Self::In) -> Result<Self::In>,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let mut iter = v.into_iter().cloned();
            let a = iter
                .next()
                .map(|ini| iter.try_fold(ini, |acc, c| f(acc.into(), c.into()).map(E::from)))
                .transpose()
                .map(|o| o.map(E::into))?;

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
    use once_cell::sync::Lazy;

    use super::*;
    use crate::{adt::*, prelude::OpGroup};

    static DATA: Lazy<FqxData> = Lazy::new(|| {
        FqxData::new(
            vec![String::from("c1"), String::from("c2"), String::from("c3")],
            vec![FqxValueType::I32, FqxValueType::String, FqxValueType::F32],
            vec![
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("A")),
                    FqxValue::F32(2.1),
                ],
                vec![
                    FqxValue::I32(2),
                    FqxValue::String(String::from("B")),
                    FqxValue::F32(1.3),
                ],
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("C")),
                    FqxValue::F32(3.2),
                ],
            ],
        )
        .unwrap()
    });

    #[test]
    fn reduce_self_success() {
        let data = DATA.clone();

        let foo = (&data).reduce(|p, c| p + c);
        println!("{:?}", foo);

        let foo = data.reduce(|p, c| p + c);
        println!("{:?}", foo);
    }

    #[test]
    fn reduce_slice_success() {
        let data = DATA.clone();

        let slice = &data[..];

        let foo = slice.reduce(|p, c| p + c);

        println!("{:?}", foo);
    }

    #[test]
    fn reduce_group_success() {
        let data = DATA.clone();

        let foo = data.group_by(|r| vec![r[0].clone()]).reduce(|p, c| p + c);

        println!("{:?}", foo);
    }
}
