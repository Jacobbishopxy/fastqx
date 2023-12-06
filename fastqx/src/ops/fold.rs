//! file: fold.rs
//! author: Jacob Xie
//! date: 2023/09/24 18:50:53 Sunday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxD, FqxValue};
use crate::ops::FqxGroup;

// ================================================================================================
// OpFold
// ================================================================================================

pub trait OpFold<const OWNED: bool> {
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

impl<U> OpFold<true> for U
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> A,
    {
        Iterator::fold(self.data().into_iter(), accumulator, |acc, r| {
            f(acc, r.clone())
        })
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> Result<A>,
    {
        Iterator::try_fold(&mut self.data().into_iter(), accumulator, |acc, r| {
            f(acc, r.clone())
        })
    }
}

impl<'a, U> OpFold<false> for &'a U
where
    U: FqxD,
{
    type Item = &'a U::RowT;

    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> A,
    {
        self.data()
            .into_iter()
            .fold(accumulator, |acc, r| f(acc, r))
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> Result<A>,
    {
        self.data()
            .into_iter()
            .try_fold(accumulator, |acc, r| f(acc, r))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup

impl<U> OpFold<true> for FqxGroup<U>
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = HashMap<Vec<FqxValue>, A>;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, Self::Item) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = Iterator::fold(v.data().into_iter(), accumulator.clone(), |acc, r| {
                f(acc, r.clone())
            });
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
                Iterator::try_fold(&mut v.data().into_iter(), accumulator.clone(), |acc, r| {
                    f(acc, r.clone())
                })?;
            res.insert(k, a);
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
    fn fold_group_success() {
        let data = D2.clone();

        let foo =
            data.rf()
                .group_by_fn(|r| vec![r[0].clone()])
                .fold(String::new(), |mut acc, r| {
                    acc.push_str(&r[1].to_string());

                    acc
                });
        println!("{:?}", foo);

        let foo = data
            .group_by_fn(|r| vec![r[0].clone()])
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
            .group_by_fn(|r| vec![r[0].clone()])
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);

        let foo = data
            .select([0, 1].as_slice())
            .group_by_fn(|r| vec![r[0].clone()])
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);
    }
}
