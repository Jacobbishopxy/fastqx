//! file: agg.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:21:51 Sunday
//! brief:

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Add;

use crate::adt::{FqxD, FqxValue, RowProps};
use crate::fqx;
use crate::ops::utils::*;
use crate::ops::{FqxGroup, FqxLazyGroup};

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
            .map(|ini| iter.fold(ini.clone(), |acc, cr| _get_row_sum(&acc, cr)))
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
                _get_row_sum(&acc, cr)
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

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxLazyGroup<T>

impl<'a, U> OpAgg for FqxLazyGroup<'a, U>
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = U;

    fn sum(&self) -> Self::Ret<Self::Item> {
        lazy_agg(&self, |acc, cr| acc.add(cr))
    }

    fn min(&self) -> Self::Ret<Self::Item> {
        lazy_agg(&self, |acc, cr| acc.min(&cr))
    }

    fn max(&self) -> Self::Ret<Self::Item> {
        lazy_agg(&self, |acc, cr| acc.max(&cr))
    }

    fn mean(&self) -> Self::Ret<Self::Item> {
        lazy_agg_with_count(
            &self,
            |acc, cr| acc.add(cr),
            |row, count| {
                U::RowT::from_values(row.iter_owned().map(|v| v / fqx!(count as f32)).collect())
            },
        )
    }
}

fn lazy_agg<'a, U, F>(lz: &FqxLazyGroup<'a, U>, f: F) -> U
where
    U: FqxD,
    F: Fn(U::RowT, U::RowT) -> U::RowT,
{
    let mut buf: HashMap<Vec<&FqxValue>, U::RowT> = HashMap::new();
    lz.to_group().into_iter().for_each(|(k, g)| {
        let mut iter = g.into_iter();
        let ini = iter.next().unwrap().select(&lz.selected_aggs);
        let accum = iter.fold(ini, |acc, cr| f(acc, cr.select(&lz.selected_aggs)));
        match buf.entry(k) {
            Entry::Occupied(mut o) => {
                let old = std::mem::take(o.get_mut());
                *o.get_mut() = f(accum, old);
            }
            Entry::Vacant(v) => {
                v.insert(accum);
            }
        }
    });

    let new_data = buf
        .into_iter()
        .map(|(k, v)| {
            let mut ks = U::RowT::from_values(k.into_iter().cloned().collect());
            ks.extend(v.to_values());
            ks
        })
        .collect();

    lazy_agg_ctor(lz, new_data)
}

fn lazy_agg_with_count<'a, U, F1, F2>(lz: &FqxLazyGroup<'a, U>, f_acc: F1, f_rc: F2) -> U
where
    U: FqxD,
    F1: Fn(U::RowT, U::RowT) -> U::RowT,
    F2: Fn(U::RowT, usize) -> U::RowT,
{
    let mut buf: HashMap<Vec<&FqxValue>, (U::RowT, usize)> = HashMap::new();
    lz.to_group().into_iter().for_each(|(k, g)| {
        let mut iter = g.into_iter();
        let ini = iter.next().unwrap().select(&lz.selected_aggs);
        let mut count = 1;
        let accum = iter.fold(ini, |acc, cr| {
            count += 1;
            f_acc(acc, cr.select(&lz.selected_aggs))
        });
        match buf.entry(k) {
            Entry::Occupied(mut o) => {
                let (pa, pc) = std::mem::take(o.get_mut());
                count += pc;
                *o.get_mut() = (f_acc(accum, pa), count);
            }
            Entry::Vacant(v) => {
                v.insert((accum, count));
            }
        }
    });

    let new_data = buf
        .into_iter()
        .map(|(k, (v, c))| {
            let mut ks = U::RowT::from_values(k.into_iter().cloned().collect());
            ks.extend(f_rc(v, c).to_values());
            ks
        })
        .collect();

    lazy_agg_ctor(lz, new_data)
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_agg {
    use super::*;
    use crate::ops::{OpGroup, OpLazyGroup, OpOwned, OpSelect};

    use crate::ops::mock::data::{D2, D5};

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

        let grp = data.rf().group_by_fn_(|r| vec![r[0].clone()]);
        let grp = grp.to_owned().mean();
        println!("{:?}", grp);

        let grp = data.group_by_fn_(|r| vec![r[0].clone()]);
        let grp = grp.mean();
        println!("{:?}", grp);
    }

    #[test]
    fn agg_selected_group_success() {
        let data = D2.clone();

        let selected = (&data)
            .select([0, 2].as_slice())
            .group_by_fn_(|r| vec![r[0].clone()])
            .mean();
        println!("{:?}", selected);

        let selected = data
            .select([0, 2].as_slice())
            .group_by_fn_(|r| vec![r[0].clone()])
            .mean();
        println!("{:?}", selected);
    }

    #[test]
    fn agg_lazy_group_success() {
        let data = D5.clone();

        let grp = data.group_by(&["col_0"]).select(&["col_2"]);

        println!("{:?}", grp.sum());
        println!("{:?}", grp.min());
        println!("{:?}", grp.max());
        println!("{:?}", grp.mean());
    }
}
