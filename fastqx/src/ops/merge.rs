//! file: merge.rs
//! author: Jacob Xie
//! date: 2023/10/10 09:11:09 Tuesday
//! brief:

use itertools::Itertools;

use crate::adt::{FqxDataGenenartor, FqxRow, FqxSchemaGetter};

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

impl<T, E> OpMerge for T
where
    T: IntoIterator<Item = E>,
    FqxRow: From<E>,
    T: FqxDataGenenartor<Vec<E>> + FqxSchemaGetter,
{
    type Item = E;

    fn merge_by<F>(self, other: Self, f: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        let l_schema = self.get_schema();
        // let r_schema = other.get_schema();

        let d = Itertools::merge_by(self.into_iter(), other.into_iter(), f).collect::<Vec<E>>();
        T::from_d(d, l_schema)
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
    fn merge_self_success() {
        let d1 = DATA.clone();
        let d2 = DATA.clone();

        let res = d1.merge_by(d2, |r1, r2| r1[0] == r2[0]);
        println!("{:?}", res);
    }
}
