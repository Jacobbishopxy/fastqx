//! file: sort.rs
//! author: Jacob Xie
//! date: 2023/10/09 19:45:55 Monday
//! brief:

use std::collections::HashMap;

use itertools::Itertools;

use crate::adt::FqxD;
use crate::ops::utils::_sort_bool_to_ordering;
use crate::ops::FqxGroup;

// ================================================================================================
// OpSort
// ================================================================================================

pub trait OpSort
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

impl<U> OpSort for U
where
    Self: Sized,
    U: FqxD,
{
    type Item = U::RowT;

    fn sorted_by<F>(self, mut cmp: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        let (c, t, d) = self.dcst();

        let d =
            Itertools::sorted_by(d.into_iter(), |p, c| _sort_bool_to_ordering(cmp(p, c))).collect();

        U::cst(c, t, d)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<U> OpSort for FqxGroup<U>
where
    Self: Sized,
    U: FqxD,
{
    type Item = U::RowT;

    fn sorted_by<F>(self, mut cmp: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let (c, t, d) = v.dcst();

            let d = Itertools::sorted_by(d.into_iter(), |p, c| _sort_bool_to_ordering(cmp(p, c)))
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
    use super::*;
    use crate::ops::mock::data::D1;
    use crate::ops::{OpGroup, OpOwned, OpSelect};

    #[test]
    fn sort_self_success() {
        let data = D1.clone();

        let foo = data.rf().sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);

        let foo = data.sorted_by(|p, c| p[0] < c[0]);
        println!("{:?}", foo);
    }

    #[test]
    fn sort_group_success() {
        let data = D1.clone();

        let foo = data
            .rf()
            .group_by_fn_(|r| vec![r[0].clone()])
            .to_owned()
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);

        let foo = data
            .group_by_fn_(|r| vec![r[0].clone()])
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);
    }

    #[test]
    fn sort_selected_success() {
        let data = D1.clone();

        let foo = data.select([0, 1].as_slice()).sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);
    }

    #[test]
    fn sort_selected_group_success() {
        let data = D1.clone();

        let foo = data
            .select([0, 1].as_slice())
            .group_by_fn_(|r| vec![r[0].clone()])
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);
    }
}
