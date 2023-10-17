//! file: merge.rs
//! author: Jacob Xie
//! date: 2023/10/10 09:11:09 Tuesday
//! brief:

use itertools::{EitherOrBoth, Itertools};

use crate::adt::{FqxD, FqxData, FqxRow, PhantomU};
use crate::ops::utils::{merge_bool_to_ordering, sort_bool_to_ordering};
use crate::ops::OpCloned;

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
        O: OpCloned<Self::Ret, Ret = Self::Ret>;

    fn sorted_merge_by<P, F, O>(self, other: O, cmp: P, f: F) -> Self::Ret
    where
        P: Clone,
        P: FnMut(&Self::Item, &Self::Item) -> bool,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
        O: OpCloned<Self::Ret, Ret = Self::Ret>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<U, C, T, I, E> OpMerge<PhantomU<C, T, I, E>> for U
where
    Self: Sized,
    U: FqxD<C, T, I, E> + OpCloned<FqxData, Ret = FqxData>,
    C: Clone,
    T: Clone,
    I: Default + Clone + Extend<E>,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    type Item = FqxRow;
    type Ret = FqxData;

    fn merge_by<F, O>(self, other: O, mut f: F) -> Self::Ret
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
        O: OpCloned<Self::Ret, Ret = Self::Ret>,
    {
        let (l, r): (FqxData, FqxData) = (self.cloned(), other.cloned());
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
        O: OpCloned<Self::Ret, Ret = Self::Ret>,
    {
        let (l, r) = (self.cloned(), other.cloned());
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
    use once_cell::sync::Lazy;

    use super::*;
    use crate::adt::*;
    use crate::ops::OpSelect;

    static DATA1: Lazy<FqxData> = Lazy::new(|| {
        FqxData::new(
            vec![String::from("c1"), String::from("c2"), String::from("c3")],
            vec![FqxValueType::I32, FqxValueType::String, FqxValueType::F32],
            vec![
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("lA")),
                    FqxValue::F32(1.1),
                ],
                vec![
                    FqxValue::I32(2),
                    FqxValue::String(String::from("lB")),
                    FqxValue::F32(2.2),
                ],
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("lC")),
                    FqxValue::F32(3.3),
                ],
            ],
        )
        .unwrap()
    });

    static DATA2: Lazy<FqxData> = Lazy::new(|| {
        FqxData::new(
            vec![String::from("c1"), String::from("c4")],
            vec![FqxValueType::I32, FqxValueType::String],
            vec![
                vec![FqxValue::I32(1), FqxValue::String(String::from("rA"))],
                vec![FqxValue::I32(2), FqxValue::String(String::from("rB"))],
                vec![FqxValue::I32(1), FqxValue::String(String::from("rC"))],
                vec![FqxValue::I32(2), FqxValue::String(String::from("rD"))],
            ],
        )
        .unwrap()
    });

    #[test]
    fn merge_self_success() {
        let d1 = DATA1.clone();
        let d2 = DATA2.clone();

        let res = d1.merge_by(d2, |r1, r2| r1[0] == r2[0]);
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
    }

    #[test]
    fn sorted_merge_self_success() {
        let d1 = DATA1.clone();
        let d2 = DATA2.clone();

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
}
