//! file: filter.rs
//! author: Jacob Xie
//! date: 2023/10/08 20:51:52 Sunday
//! brief:

use std::collections::HashMap;

use crate::adt::FqxD;
use crate::ops::FqxGroup;

// ================================================================================================
// OpFilter
// ================================================================================================

pub trait OpFilter
where
    Self: Sized,
{
    type Item;

    fn filter<F>(self, f: F) -> Self
    where
        F: FnMut(&Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// Generic T

impl<U> OpFilter for U
where
    Self: Sized,
    U: FqxD,
{
    type Item = U::RowT;

    fn filter<F>(self, f: F) -> Self
    where
        F: FnMut(&Self::Item) -> bool,
    {
        let (c, t, d) = self.dcst();

        let d = Iterator::filter(d.into_iter(), f).collect();

        U::cst(c, t, d)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<U> OpFilter for FqxGroup<U>
where
    Self: Sized,
    U: FqxD,
{
    type Item = U::RowT;

    fn filter<F>(self, mut f: F) -> Self
    where
        F: FnMut(&Self::Item) -> bool,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let (c, t, d) = v.dcst();
            let d = Iterator::filter(d.into_iter(), &mut f).collect();
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
    use crate::fqx;
    use crate::ops::mock::data::D4;
    use crate::ops::{OpGroup, OpSelect};

    #[test]
    fn filter_self_success() {
        let data = D4.clone();

        let foo = data.rf().filter(|r| r[0] == fqx!(2));
        println!("{:?}", foo);

        let foo = data.filter(|r| r[0] == fqx!(2));
        println!("{:?}", foo);
    }

    #[test]
    fn filter_group_success() {
        let data = D4.clone();

        let foo = data.rf().group_by_fn_(|r| vec![r[0].clone()]);
        let foo = foo.filter(|r| r[0] == fqx!(2));
        println!("{:?}", foo);

        let foo = data
            .group_by_fn_(|r| vec![r[0].clone()])
            .filter(|r| r[0] == fqx!(2));
        println!("{:?}", foo);
    }

    #[test]
    fn filter_selected_success() {
        let data = D4.clone();

        let foo = (&data)
            .select([0, 1].as_slice())
            .filter(|r| r[0] == fqx!(2));
        println!("{:?}", foo);

        let foo = data.select([0, 1].as_slice()).filter(|r| r[0] == fqx!(2));
        println!("{:?}", foo);
    }

    #[test]
    fn filter_selected_group_success() {
        let data = D4.clone();

        let foo = (&data)
            .select([0, 1].as_slice())
            .group_by_fn_(|r| vec![r[0].clone()]);
        let foo = foo.filter(|r| r[0] == fqx!(2));
        println!("{:?}", foo);

        let foo = data
            .select([0, 1].as_slice())
            .group_by_fn_(|r| vec![r[0].clone()])
            .filter(|r| r[0] == fqx!(2));
        println!("{:?}", foo);
    }
}
