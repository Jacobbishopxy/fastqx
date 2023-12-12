//! file: cumagg.rs
//! author: Jacob Xie
//! date: 2023/10/21 21:48:15 Saturday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxD, FqxValue, RowProps};
use crate::ops::utils::*;
use crate::ops::FqxGroup;

// ================================================================================================
// OpCumAgg
// ================================================================================================

pub trait OpCumAgg
where
    Self: Sized,
{
    type Item;
    type Ret<A>;

    fn cum_sum(&self) -> Self::Ret<Self::Item>;

    fn cum_min(&self) -> Self::Ret<Self::Item>;

    fn cum_max(&self) -> Self::Ret<Self::Item>;

    fn cum_mean(&self) -> Self::Ret<Self::Item>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<U> OpCumAgg for U
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = Vec<Self::Item>;

    fn cum_sum(&self) -> Self::Ret<Self::Item> {
        let mut iter = self.data().into_iter();
        iter.next()
            .map(|fst| {
                iter.fold(vec![fst.clone()], |mut acc, r| {
                    let cum = acc.last().unwrap().add(&r);
                    acc.push(cum);
                    acc
                })
            })
            .unwrap_or(vec![])
    }

    fn cum_min(&self) -> Self::Ret<Self::Item> {
        let mut iter = self.data().into_iter();
        iter.next()
            .map(|fst| {
                iter.fold(vec![fst.clone()], |mut acc, r| {
                    let l = acc.last().unwrap();
                    let cum = _get_row_min(l, &r);
                    acc.push(cum);
                    acc
                })
            })
            .unwrap_or(vec![])
    }

    fn cum_max(&self) -> Self::Ret<Self::Item> {
        let mut iter = self.data().into_iter();
        iter.next()
            .map(|fst| {
                iter.fold(vec![fst.clone()], |mut acc, r| {
                    let l = acc.last().unwrap();
                    let cum = _get_row_max(l, &r);
                    acc.push(cum);
                    acc
                })
            })
            .unwrap_or(vec![])
    }

    fn cum_mean(&self) -> Self::Ret<Self::Item> {
        let sum = self.cum_sum();

        sum.into_iter()
            .enumerate()
            .map(|(idx, r)| _calc_mean(r, idx + 1))
            .collect()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<U> OpCumAgg for FqxGroup<U>
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = HashMap<Vec<FqxValue>, Vec<Self::Item>>;

    fn cum_sum(&self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.cum_sum();
            res.insert(k.clone(), a);
        }

        res
    }

    fn cum_min(&self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.cum_min();
            res.insert(k.clone(), a);
        }

        res
    }

    fn cum_max(&self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.cum_max();
            res.insert(k.clone(), a);
        }

        res
    }

    fn cum_mean(&self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.cum_mean();
            res.insert(k.clone(), a);
        }

        res
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_cumagg {
    use super::*;

    use crate::ops::mock::data::D2;
    use crate::ops::{OpGroup, OpOwned, OpSelect};

    #[test]
    fn cumagg_self_success() {
        let data = D2.clone();

        let a1 = (&data).cum_sum();
        let a2 = (&data).cum_max();
        let a3 = (&data).cum_min();
        let a4 = (&data).cum_mean();
        println!("{:?}", a1);
        println!("{:?}", a2);
        println!("{:?}", a3);
        println!("{:?}", a4);

        let a1 = D2.clone().cum_sum();
        let a2 = D2.clone().cum_max();
        let a3 = D2.clone().cum_min();
        let a4 = data.cum_mean();
        println!("{:?}", a1);
        println!("{:?}", a2);
        println!("{:?}", a3);
        println!("{:?}", a4);
    }

    #[test]
    fn cum_agg_selected_success() {
        let data = D2.clone();

        let selected = (&data).select([0, 2].as_slice()).cum_sum();
        println!("{:?}", selected);
        let selected = data.select([0, 2].as_slice()).cum_sum();
        println!("{:?}", selected);
    }

    #[test]
    fn cum_agg_group_success() {
        let data = D2.clone();

        let grp = data.rf().group_by_fn(|r| vec![r[0].clone()]);
        let grp = grp.to_owned().cum_mean();
        println!("{:?}", grp);

        let grp = data.group_by_fn(|r| vec![r[0].clone()]);
        let grp = grp.cum_mean();
        println!("{:?}", grp);
    }

    #[test]
    fn cum_agg_selected_group_success() {
        let data = D2.clone();

        let selected = (&data)
            .select([0, 2].as_slice())
            .group_by_fn(|r| vec![r[0].clone()])
            .cum_mean();
        println!("{:?}", selected);

        let selected = data
            .select([0, 2].as_slice())
            .group_by_fn(|r| vec![r[0].clone()])
            .cum_mean();
        println!("{:?}", selected);
    }
}
