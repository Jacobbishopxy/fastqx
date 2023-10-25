//! file: merge.rs
//! author: Jacob Xie
//! date: 2023/10/10 09:11:09 Tuesday
//! brief:

use itertools::{EitherOrBoth, Itertools};

use crate::adt::{FqxD, FqxData, FqxRow, PhantomU};
use crate::ops::utils::{fqx_data_left_join, merge_bool_to_ordering, sort_bool_to_ordering};
use crate::ops::OpOwned;

// ================================================================================================
// FqxMergeType
// ================================================================================================

pub enum FqxMergeType {
    Left,
    Right,
    Outer,
    Inner,
    Cross,
}

impl Default for FqxMergeType {
    fn default() -> Self {
        Self::Left
    }
}

// ================================================================================================
// OpMerge
// ================================================================================================

pub trait OpMerge0<T> {
    type Ret;

    fn merge<O, N, S>(self, other: O, left_on: N, right_on: N, how: FqxMergeType) -> Self::Ret
    where
        O: OpOwned<Self::Ret, Ret = Self::Ret>,
        N: IntoIterator<Item = S>,
        S: AsRef<str>;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<U, C, T, I, E> OpMerge0<PhantomU<C, T, I, E>> for U
where
    Self: Sized,
    U: FqxD<C, T, I, E> + OpOwned<FqxData, Ret = FqxData>,
    I: Default + Clone + Extend<E>,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    type Ret = FqxData;

    fn merge<O, N, S>(self, other: O, left_on: N, right_on: N, how: FqxMergeType) -> Self::Ret
    where
        O: OpOwned<Self::Ret, Ret = Self::Ret>,
        N: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let (l, r): (FqxData, FqxData) = (self.to_owned(), other.to_owned());
        match how {
            FqxMergeType::Left => fqx_data_left_join(l, r, left_on, right_on),
            FqxMergeType::Right => fqx_data_left_join(r, l, right_on, left_on),
            FqxMergeType::Outer => todo!(),
            FqxMergeType::Inner => todo!(),
            FqxMergeType::Cross => todo!(),
        }
    }
}

// ================================================================================================
// OpMerge
// ================================================================================================

pub trait OpMerge<T>
where
    Self: Sized,
{
    type Item;
    type Ret;

    fn merge_by<F, O>(self, other: O, f: F) -> Self::Ret
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
        O: OpOwned<Self::Ret, Ret = Self::Ret>;

    fn sorted_merge_by<P, F, O>(self, other: O, cmp: P, f: F) -> Self::Ret
    where
        P: Clone,
        P: FnMut(&Self::Item, &Self::Item) -> bool,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
        O: OpOwned<Self::Ret, Ret = Self::Ret>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<U, C, T, I, E> OpMerge<PhantomU<C, T, I, E>> for U
where
    Self: Sized,
    U: FqxD<C, T, I, E> + OpOwned<FqxData, Ret = FqxData>,
    I: Default + Clone + Extend<E>,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    type Item = FqxRow;
    type Ret = FqxData;

    fn merge_by<F, O>(self, other: O, mut f: F) -> Self::Ret
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
        O: OpOwned<Self::Ret, Ret = Self::Ret>,
    {
        let (l, r): (FqxData, FqxData) = (self.to_owned(), other.to_owned());
        let l_empties = l.empty_row();
        let r_empties = r.empty_row();
        let (mut lc, mut lt, ld) = l.dcst();
        let (rc, rt, rd) = r.dcst();

        let d = Itertools::merge_join_by(ld.into_iter(), rd.into_iter(), |l, r| {
            merge_bool_to_ordering(f(l, r))
        })
        .map(|e| merge_row(&l_empties, &r_empties, e))
        .collect::<Vec<_>>();

        lc.extend(rc);
        lt.extend(rt);

        FqxData {
            columns: lc,
            types: lt,
            data: d,
        }
    }

    fn sorted_merge_by<P, F, O>(self, other: O, cmp: P, mut f: F) -> Self::Ret
    where
        P: Clone,
        P: FnMut(&Self::Item, &Self::Item) -> bool,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
        O: OpOwned<Self::Ret, Ret = Self::Ret>,
    {
        let (l, r) = (self.to_owned(), other.to_owned());
        let l_empties = l.empty_row();
        let r_empties = r.empty_row();
        let (mut lc, mut lt, ld) = l.dcst();
        let (rc, rt, rd) = r.dcst();

        let sl = Itertools::sorted_by(ld.into_iter(), |p, c| {
            sort_bool_to_ordering(cmp.clone()(p, c))
        });
        let sr = Itertools::sorted_by(rd.into_iter(), |p, c| {
            sort_bool_to_ordering(cmp.clone()(p, c))
        });

        let d = Itertools::merge_join_by(sl, sr, |l, r| merge_bool_to_ordering(f(l, r)))
            .map(|e| merge_row(&l_empties, &r_empties, e))
            .collect::<Vec<_>>();

        lc.extend(rc);
        lt.extend(rt);

        FqxData {
            columns: lc,
            types: lt,
            data: d,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// helpers

fn merge_row(le: &FqxRow, re: &FqxRow, eob: EitherOrBoth<FqxRow, FqxRow>) -> FqxRow {
    match eob {
        EitherOrBoth::Both(mut l, r) => {
            l.extend(r);
            l
        }
        EitherOrBoth::Left(mut l) => {
            l.extend(re.clone());
            l
        }
        EitherOrBoth::Right(r) => {
            let mut l = le.clone();
            l.extend(r);
            l
        }
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_merge {
    use super::*;
    use crate::mock::data::{D6, D7, D8, D9};
    use crate::ops::OpSelect;

    #[test]
    fn merge_self_success() {
        let d1 = D6.clone();
        let d2 = D7.clone();

        let res = d1.merge_by(d2, |r1, r2| r1[0] == r2[0]);
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
    }

    #[test]
    fn sorted_merge_self_success() {
        let d1 = D6.clone();
        let d2 = D7.clone();

        let res = d1.sorted_merge_by(
            d2.select(1..),
            |r1, r2| r1[0] < r2[0],
            |r1, r2| r1[0] == r2[0],
        );
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
    }

    #[test]
    fn merge_self_success_new() {
        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // merge left

        let d1 = D8.clone();
        let d2 = D9.clone();

        let res = d1.merge(d2, &["Fruit"], &["Name"], FqxMergeType::Left);
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }

        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // merge right

        let d1 = D8.clone();
        let d2 = D9.clone();

        let res = d1.merge(d2, &["Fruit"], &["Name"], FqxMergeType::Right);
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
    }
}
