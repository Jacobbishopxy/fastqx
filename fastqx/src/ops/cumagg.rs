//! file: cumagg.rs
//! author: Jacob Xie
//! date: 2023/10/21 21:48:15 Saturday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxRow, FqxValue};
use crate::ops::utils::*;
use crate::ops::FqxGroup;

// ================================================================================================
// OpCumAgg
// ================================================================================================

pub trait OpCumAgg<T>
where
    Self: Sized,
{
    type Item;
    type Ret<A>;

    fn cum_sum(self) -> Self::Ret<Self::Item>;

    fn cum_min(self) -> Self::Ret<Self::Item>;

    fn cum_max(self) -> Self::Ret<Self::Item>;

    fn cum_mean(self) -> Self::Ret<Self::Item>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<T, E> OpCumAgg<FqxRow> for T
where
    T: IntoIterator<Item = E>,
    E: Into<FqxRow>,
{
    type Item = FqxRow;

    type Ret<A> = Vec<A>;

    fn cum_sum(self) -> Self::Ret<Self::Item> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|fst| {
                iter.fold(vec![fst.into()], |mut acc, r| {
                    let cum = acc.last().unwrap().clone() + r.into();
                    acc.push(cum);
                    acc
                })
            })
            .unwrap_or(vec![])
    }

    fn cum_min(self) -> Self::Ret<Self::Item> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|fst| {
                iter.fold(vec![fst.into()], |mut acc, r| {
                    let r1 = acc.last().unwrap().clone().into();
                    let r = _get_row_min(r1, r.into());
                    acc.push(r);
                    acc
                })
            })
            .unwrap_or(vec![])
    }

    fn cum_max(self) -> Self::Ret<Self::Item> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|fst| {
                iter.fold(vec![fst.into()], |mut acc, r| {
                    let r1 = acc.last().unwrap().clone().into();
                    let r = _get_row_max(r1, r.into());
                    acc.push(r);
                    acc
                })
            })
            .unwrap_or(vec![])
    }

    fn cum_mean(self) -> Self::Ret<Self::Item> {
        let sum = self.cum_sum();

        sum.into_iter()
            .enumerate()
            .map(|(idx, r)| _calc_mean(r, idx + 1))
            .collect()
    }
}

impl<'a, T, E> OpCumAgg<&'a FqxRow> for &'a T
where
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRow>,
{
    type Item = FqxRow;

    type Ret<A> = Vec<A>;

    fn cum_sum(self) -> Self::Ret<Self::Item> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|fst| {
                iter.fold(vec![fst.as_ref().into()], |mut acc: Vec<FqxRow>, r| {
                    let cum = acc.last().unwrap().clone() + r.as_ref().clone();
                    acc.push(cum);
                    acc
                })
            })
            .unwrap_or(vec![])
    }

    fn cum_min(self) -> Self::Ret<Self::Item> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|fst| {
                iter.fold(vec![fst.as_ref().into()], |mut acc: Vec<FqxRow>, r| {
                    let r1 = acc.last().unwrap().clone();
                    let r = _get_row_min(r1, r.as_ref().clone());
                    acc.push(r);
                    acc
                })
            })
            .unwrap_or(vec![])
    }

    fn cum_max(self) -> Self::Ret<Self::Item> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|fst| {
                iter.fold(vec![fst.as_ref().into()], |mut acc: Vec<FqxRow>, r| {
                    let r1 = acc.last().unwrap().clone();
                    let r = _get_row_max(r1, r.as_ref().clone());
                    acc.push(r);
                    acc
                })
            })
            .unwrap_or(vec![])
    }

    fn cum_mean(self) -> Self::Ret<Self::Item> {
        let sum = self.cum_sum();

        sum.into_iter()
            .enumerate()
            .map(|(idx, r)| _calc_mean(r, idx + 1))
            .collect()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<T, E> OpCumAgg<FqxRow> for FqxGroup<T>
where
    T: IntoIterator<Item = E>,
    E: Into<FqxRow>,
{
    type Item = FqxRow;

    type Ret<A> = HashMap<Vec<FqxValue>, Vec<A>>;

    fn cum_sum(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.cum_sum();
            res.insert(k, a);
        }

        res
    }

    fn cum_min(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.cum_min();
            res.insert(k, a);
        }

        res
    }

    fn cum_max(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.cum_max();
            res.insert(k, a);
        }

        res
    }

    fn cum_mean(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.cum_mean();
            res.insert(k, a);
        }

        res
    }
}

impl<'a, T, E> OpCumAgg<&'a FqxRow> for &'a FqxGroup<T>
where
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRow>,
{
    type Item = FqxRow;

    type Ret<A> = HashMap<Vec<FqxValue>, Vec<A>>;

    fn cum_sum(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = v.cum_sum();
            res.insert(k.clone(), a);
        }

        res
    }

    fn cum_min(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = v.cum_min();
            res.insert(k.clone(), a);
        }

        res
    }

    fn cum_max(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = v.cum_max();
            res.insert(k.clone(), a);
        }

        res
    }

    fn cum_mean(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
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

    use crate::mock::data::D2;
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
