//! file: fold.rs
//! author: Jacob Xie
//! date: 2023/09/24 18:50:53 Sunday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxRowAbstract, FqxValue};
use crate::ops::FqxGroup;

// ================================================================================================
// OpFold
// ================================================================================================

pub trait OpFold<I> {
    type Item;
    type Ret<A>;

    fn fold<A, F>(self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> A;

    fn try_fold<A, F>(self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> Result<A>;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// Generic T

impl<I, V, T, E> OpFold<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> A,
    {
        Iterator::fold(self.into_iter(), accumulator, |acc, r| f(acc, r))
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> Result<A>,
    {
        Iterator::try_fold(&mut self.into_iter(), accumulator, |acc, r| f(acc, r))
    }
}

impl<'a, I, V, T, E> OpFold<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>> + 'a,
{
    type Item = &'a E;

    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> A,
    {
        self.into_iter().fold(accumulator, |acc, r| f(acc, r))
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> Result<A>,
    {
        self.into_iter().try_fold(accumulator, |acc, r| f(acc, r))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup

impl<I, V, T, E> OpFold<FqxRowAbstract<I, V>> for FqxGroup<T>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = HashMap<Vec<FqxValue>, A>;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = Iterator::fold(v.into_iter(), accumulator.clone(), |acc, r| f(acc, r));
            res.insert(k, a);
        }

        res
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a =
                Iterator::try_fold(&mut v.into_iter(), accumulator.clone(), |acc, r| f(acc, r))?;
            res.insert(k, a);
        }

        Ok(res)
    }
}

impl<'a, I, V, T, E> OpFold<&'a FqxRowAbstract<I, V>> for &'a FqxGroup<T>
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>> + 'a,
{
    type Item = &'a E;

    type Ret<A> = HashMap<Vec<FqxValue>, A>;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = Iterator::fold(v.into_iter(), accumulator.clone(), |acc, r| f(acc, r));
            res.insert(k.clone(), a);
        }

        res
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a =
                Iterator::try_fold(&mut v.into_iter(), accumulator.clone(), |acc, r| f(acc, r))?;
            res.insert(k.clone(), a);
        }

        Ok(res)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_fold {
    use super::*;
    use crate::fqx;
    use crate::mock::data::D2;
    use crate::ops::{OpGroup, OpSelect};

    #[test]
    fn fold_self_success() {
        let data = D2.clone();

        let foo = (&data).fold(vec![], |mut acc, r| {
            acc.push(r[1].clone());

            acc
        });
        println!("{:?}", foo);

        let foo = data.fold(fqx!(0f32), |mut acc, r| {
            acc += r[2].clone();

            acc
        });
        println!("{:?}", foo);
    }

    #[test]
    fn fold_slice_success() {
        let data = D2.clone();

        let slice = &data[1..3];

        let foo = slice.fold(vec![], |mut acc, r| {
            acc.push(r[1].clone());

            acc
        });
        println!("{:?}", foo);
    }

    #[test]
    fn fold_group_success() {
        let data = D2.clone();

        let foo = data
            .rf()
            .group_by(|r| vec![r[0].clone()])
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);

        let foo = data
            .group_by(|r| vec![r[0].clone()])
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);
    }

    #[test]
    fn fold_selected_success() {
        let data = D2.clone();

        let foo = (&data)
            .select([0, 1].as_slice())
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);

        let foo = data
            .select([0, 1].as_slice())
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);
    }

    #[test]
    fn fold_selected_group_success() {
        let data = D2.clone();

        let foo = (&data)
            .select([0, 1].as_slice())
            .group_by(|r| vec![r[0].clone()])
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);

        let foo = data
            .select([0, 1].as_slice())
            .group_by(|r| vec![r[0].clone()])
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);
    }
}
