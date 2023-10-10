//! file: merge.rs
//! author: Jacob Xie
//! date: 2023/10/10 09:11:09 Tuesday
//! brief:

use itertools::Itertools;

use crate::adt::{FqxDataGenerator, FqxSchemaGetter, FqxValue};
use crate::ops::utils::merge_bool_to_ordering;

// ================================================================================================
// OpMerge
// ================================================================================================

pub trait OpMerge {
    type Item;

    fn merge_by<F>(self, other: Self, f: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

// TODO: `merge_by` is based on orders. SQL-style join is required

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
        let l_empties = self.gen_empty_row();
        let r_empties = other.gen_empty_row();
        let mut schema = self.get_schema();
        schema.extend(other.get_schema());

        let d = Itertools::merge_join_by(self.into_iter(), other.into_iter(), |l, r| {
            merge_bool_to_ordering(f(l, r))
        })
        .map(|e| match e {
            itertools::EitherOrBoth::Both(mut l, r) => {
                l.extend(r);
                l
            }
            itertools::EitherOrBoth::Left(mut l) => {
                l.extend(r_empties.clone());
                l
            }
            itertools::EitherOrBoth::Right(r) => {
                let mut l = l_empties.clone();
                l.extend(r);
                l
            }
        })
        .collect::<Vec<_>>();

        T::from_d(d, schema)
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
    // use crate::ops::{OpCloned, OpGroup, OpSelect};

    static DATA1: Lazy<FqxData> = Lazy::new(|| {
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

    static DATA2: Lazy<FqxData> = Lazy::new(|| {
        FqxData::new(
            vec![String::from("c1"), String::from("c4")],
            vec![FqxValueType::I32, FqxValueType::String],
            vec![
                vec![FqxValue::I32(1), FqxValue::String(String::from("A"))],
                vec![FqxValue::I32(1), FqxValue::String(String::from("C"))],
                vec![FqxValue::I32(2), FqxValue::String(String::from("B"))],
                vec![FqxValue::I32(2), FqxValue::String(String::from("B+"))],
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
}
