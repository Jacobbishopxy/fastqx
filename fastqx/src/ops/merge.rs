//! file: merge.rs
//! author: Jacob Xie
//! date: 2023/10/10 09:11:09 Tuesday
//! brief:

use itertools::{EitherOrBoth, Itertools};

use crate::adt::{FqxDataGenerator, FqxSchema, FqxSchemaGetter, FqxValue};
use crate::ops::utils::{merge_bool_to_ordering, sort_bool_to_ordering};

// ================================================================================================
// OpMerge
// ================================================================================================

pub trait OpMerge {
    type Item;

    fn merge_by<F>(self, other: Self, f: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool;

    fn sorted_merge_by<C, F>(self, other: Self, cmp: C, f: F) -> Self
    where
        C: Clone,
        C: FnMut(&Self::Item, &Self::Item) -> bool,
        F: FnMut(&Self::Item, &Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<T, E> OpMerge for T
where
    T: IntoIterator<Item = E>,
    T: FqxDataGenerator<Vec<E>> + FqxSchemaGetter<E>,
    E: IntoIterator<Item = FqxValue> + Extend<FqxValue> + Clone,
{
    type Item = E;

    fn merge_by<F>(self, other: Self, mut f: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        let (l_empties, r_empties, schema) = gen_empties_and_schema(&self, &other);

        let d = Itertools::merge_join_by(self.into_iter(), other.into_iter(), |l, r| {
            merge_bool_to_ordering(f(l, r))
        })
        .map(|e| merge_row(&l_empties, &r_empties, e))
        .collect::<Vec<_>>();

        T::from_d(d, schema)
    }

    fn sorted_merge_by<C, F>(self, other: Self, cmp: C, mut f: F) -> Self
    where
        C: Clone,
        C: FnMut(&Self::Item, &Self::Item) -> bool,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        let (l_empties, r_empties, schema) = gen_empties_and_schema(&self, &other);

        let sl = Itertools::sorted_by(self.into_iter(), |p, c| {
            sort_bool_to_ordering(cmp.clone()(p, c))
        });
        let sr = Itertools::sorted_by(other.into_iter(), |p, c| {
            sort_bool_to_ordering(cmp.clone()(p, c))
        });

        let d = Itertools::merge_join_by(sl, sr, |l, r| merge_bool_to_ordering(f(l, r)))
            .map(|e| merge_row(&l_empties, &r_empties, e))
            .collect::<Vec<_>>();

        T::from_d(d, schema)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// helpers

fn gen_empties_and_schema<T, E>(l: &T, r: &T) -> (E, E, FqxSchema)
where
    T: FqxSchemaGetter<E>,
    E: IntoIterator<Item = FqxValue> + Extend<FqxValue>,
{
    let mut schema = l.get_schema();
    schema.extend(r.get_schema());

    (l.gen_empty_row(), r.gen_empty_row(), schema)
}

fn merge_row<E>(le: &E, re: &E, eob: EitherOrBoth<E, E>) -> E
where
    E: IntoIterator<Item = FqxValue> + Extend<FqxValue> + Clone,
{
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

        let res = d1.sorted_merge_by(d2, |r1, r2| r1[0] < r2[0], |r1, r2| r1[0] == r2[0]);
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
    }
}
