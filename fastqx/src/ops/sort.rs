//! file: sort.rs
//! author: Jacob Xie
//! date: 2023/10/09 19:45:55 Monday
//! brief:

use std::collections::HashMap;

use itertools::Itertools;

use crate::adt::{FqxD, PhantomU};
use crate::ops::utils::sort_bool_to_ordering;
use crate::ops::FqxGroup;

// ================================================================================================
// OpSort
// ================================================================================================

pub trait OpSort<T>
where
    Self: Sized,
{
    type Item;

    fn sorted_by<F>(self, cmp: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// Generic T

impl<U, C, T, I, E> OpSort<PhantomU<C, T, I, E>> for U
where
    Self: Sized,
    U: FqxD<C, T, I, E>,
    C: Clone,
    T: Clone,
    I: Default + Clone,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    type Item = I;

    fn sorted_by<F>(self, mut cmp: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        let (c, t, d) = self.dcst();

        let d =
            Itertools::sorted_by(d.into_iter(), |p, c| sort_bool_to_ordering(cmp(p, c))).collect();

        U::cst(c, t, d)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<U, C, T, I, E> OpSort<FqxGroup<PhantomU<C, T, I, E>>> for FqxGroup<U>
where
    Self: Sized,
    U: FqxD<C, T, I, E>,
    C: Clone,
    T: Clone,
    I: Default + Clone,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    type Item = I;

    fn sorted_by<F>(self, mut cmp: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let (c, t, d) = v.dcst();

            let d = Itertools::sorted_by(d.into_iter(), |p, c| sort_bool_to_ordering(cmp(p, c)))
                .collect();
            res.insert(k, U::cst(c, t, d));
        }

        FqxGroup(res)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use super::*;
    use crate::adt::*;
    use crate::ops::{OpCloned, OpGroup, OpSelect};

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
    fn sort_self_success() {
        let data = DATA.clone();

        let foo = data.rf().sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);

        let foo = data.sorted_by(|p, c| p[0] < c[0]);
        println!("{:?}", foo);
    }

    #[test]
    fn sort_group_success() {
        let data = DATA.clone();

        let foo = data
            .rf()
            .group_by(|r| vec![r[0].clone()])
            .cloned()
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);

        let foo = data
            .group_by(|r| vec![r[0].clone()])
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);
    }

    #[test]
    fn sort_selected_success() {
        let data = DATA.clone();

        let foo = data.select([0, 1].as_slice()).sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);
    }

    #[test]
    fn sort_selected_group_success() {
        let data = DATA.clone();

        let foo = data
            .select([0, 1].as_slice())
            .group_by(|r| vec![r[0].clone()])
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);
    }
}
