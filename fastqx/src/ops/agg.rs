//! file: agg.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:21:51 Sunday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxD, FqxValue, RowProps};
use crate::ops::utils::*;
use crate::ops::FqxGroup;

// ================================================================================================
// OpAgg
// ================================================================================================

pub trait OpAgg {
    type Item;
    type Ret<A>;

    fn sum(&self) -> Self::Ret<Self::Item>;

    fn min(&self) -> Self::Ret<Self::Item>;

    fn max(&self) -> Self::Ret<Self::Item>;

    fn mean(&self) -> Self::Ret<Self::Item>;

    // fn median(&self) -> Self::Ret<Self::Item>;

    // fn mode(&self) -> Self::Ret<Self::Item>;

    // fn count(&self) -> Self::Ret<Self::Item>;

    // fn std(&self) -> Self::Ret<Self::Item>;

    // fn var(&self) -> Self::Ret<Self::Item>;

    // fn skew(&self) -> Self::Ret<Self::Item>;

    // fn kurt(&self) -> Self::Ret<Self::Item>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<U> OpAgg for U
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = Option<Self::Item>;

    fn sum(&self) -> Self::Ret<Self::Item> {
        let mut iter = self.data().into_iter();
        iter.next()
            .map(|ini| iter.fold(ini.clone(), |acc, cr| acc.add(&cr)))
    }

    fn min(&self) -> Self::Ret<Self::Item> {
        let mut iter = self.data().into_iter();
        iter.next()
            .map(|ini| iter.fold(ini.clone(), |acc, cr| _get_row_min(&acc, &cr)))
    }

    fn max(&self) -> Self::Ret<Self::Item> {
        let mut iter = self.data().into_iter();
        iter.next()
            .map(|ini| iter.fold(ini.clone(), |acc, cr| _get_row_max(&acc, &cr)))
    }

    fn mean(&self) -> Self::Ret<Self::Item> {
        let mut count = 0;
        let mut iter = self.data().into_iter();
        let sum = iter.next().map(|ini| {
            iter.fold(ini.clone(), |acc, cr| {
                count += 1;
                acc.add(&cr)
            })
        });

        sum.map(|r| _calc_mean(r, count))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<U> OpAgg for FqxGroup<U>
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = HashMap<Vec<FqxValue>, Option<A>>;

    fn sum(&self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.sum();
            res.insert(k.clone(), a);
        }

        res
    }

    fn min(&self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.min();
            res.insert(k.clone(), a);
        }

        res
    }

    fn max(&self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.max();
            res.insert(k.clone(), a);
        }

        res
    }

    fn mean(&self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.mean();
            res.insert(k.clone(), a);
        }

        res
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_agg {
    use super::*;
    use crate::ops::{OpGroup, OpOwned, OpSelect};

    use crate::ops::mock::data::D2;

    #[test]
    fn agg_self_success() {
        let data = D2.clone();

        let a1 = (&data).sum();
        let a2 = (&data).max();
        let a3 = (&data).min();
        let a4 = (&data).mean();
        println!("{:?}", a1);
        println!("{:?}", a2);
        println!("{:?}", a3);
        println!("{:?}", a4);

        let a1 = D2.clone().sum();
        let a2 = D2.clone().max();
        let a3 = D2.clone().min();
        let a4 = data.mean();
        println!("{:?}", a1);
        println!("{:?}", a2);
        println!("{:?}", a3);
        println!("{:?}", a4);
    }

    #[test]
    fn agg_selected_success() {
        let data = D2.clone();

        let selected = (&data).select([0, 2].as_slice()).sum();
        println!("{:?}", selected);
        let selected = data.select([0, 2].as_slice()).sum();
        println!("{:?}", selected);
    }

    #[test]
    fn agg_group_success() {
        let data = D2.clone();

        let grp = data.rf().group_by_fn(|r| vec![r[0].clone()]);
        let grp = grp.to_owned().mean();
        println!("{:?}", grp);

        let grp = data.group_by_fn(|r| vec![r[0].clone()]);
        let grp = grp.mean();
        println!("{:?}", grp);
    }

    #[test]
    fn agg_selected_group_success() {
        let data = D2.clone();

        let selected = (&data)
            .select([0, 2].as_slice())
            .group_by_fn(|r| vec![r[0].clone()])
            .mean();
        println!("{:?}", selected);

        let selected = data
            .select([0, 2].as_slice())
            .group_by_fn(|r| vec![r[0].clone()])
            .mean();
        println!("{:?}", selected);
    }
}
