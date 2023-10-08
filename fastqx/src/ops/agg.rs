//! file: agg.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:21:51 Sunday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxRowAbstract, FqxValue};
use crate::ops::utils::*;
use crate::ops::{FqxGroup, OpReduce};

// ================================================================================================
// OpAgg
// ================================================================================================

pub trait OpAgg<T> {
    type Item;
    type Ret<A>;

    fn sum(self) -> Self::Ret<Self::Item>;

    fn min(self) -> Self::Ret<Self::Item>;

    fn max(self) -> Self::Ret<Self::Item>;

    fn mean(self) -> Self::Ret<Self::Item>;
}

pub trait OpAggGroup<T> {
    type Item;
    type Ret<A>;

    fn sum(self) -> Self::Ret<Self::Item>;

    fn min(self) -> Self::Ret<Self::Item>;

    fn max(self) -> Self::Ret<Self::Item>;

    fn mean(self) -> Self::Ret<Self::Item>;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// Generic T

impl<I, V, T, E> OpAgg<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: std::ops::Add<Output = E>,
    E: From<Vec<FqxValue>>,
{
    type Item = E;

    type Ret<A> = Option<Self::Item>;

    fn sum(self) -> Self::Ret<Self::Item> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Self::Ret<Self::Item> {
        self.reduce_fqx_row(get_min)
    }

    fn max(self) -> Self::Ret<Self::Item> {
        self.reduce_fqx_row(get_max)
    }

    fn mean(self) -> Self::Ret<Self::Item> {
        let len = (&self).into_iter().count();
        self.sum().map(|r| calc_mean(r, len))
    }
}

impl<'a, I, V, T, E> OpAgg<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>> + Clone,
    E: Into<FqxRowAbstract<I, V>>,
    E: std::ops::Add<Output = E>,
    E: From<Vec<FqxValue>>,
{
    type Item = E;

    type Ret<A> = Option<Self::Item>;

    fn sum(self) -> Self::Ret<Self::Item> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Self::Ret<Self::Item> {
        self.reduce_fqx_row(get_min)
    }

    fn max(self) -> Self::Ret<Self::Item> {
        self.reduce_fqx_row(get_max)
    }

    fn mean(self) -> Self::Ret<Self::Item> {
        let len = (&self).into_iter().count();
        self.sum().map(|r| calc_mean(r.clone(), len))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<I, V, T, E> OpAggGroup<FqxRowAbstract<I, V>> for FqxGroup<T>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>, // `E` can bt a reference
    E: Into<FqxRowAbstract<I, V>>,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: std::ops::Add<Output = E>,
    E: From<Vec<FqxValue>>,
{
    type Item = E;

    type Ret<A> = HashMap<Vec<FqxValue>, Option<Self::Item>>;

    fn sum(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce(|p, c| p + c);
            res.insert(k, a);
        }

        res
    }

    fn min(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_min);
            res.insert(k, a);
        }

        res
    }

    fn max(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_max);
            res.insert(k, a);
        }

        res
    }

    fn mean(self) -> Self::Ret<Self::Item> {
        let lens = self
            .0
            .iter()
            .map(|(_, v)| v.into_iter().count())
            .collect::<Vec<_>>();
        self.sum()
            .into_iter()
            .zip(lens.into_iter())
            .map(|((k, v), len)| (k, v.map(|r| calc_mean(r, len))))
            .collect::<HashMap<_, _>>()
    }
}

impl<'a, I, V, T, E> OpAggGroup<&'a FqxRowAbstract<I, V>> for &'a FqxGroup<T>
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>> + Clone,
    E: Into<FqxRowAbstract<I, V>>,
    E: std::ops::Add<Output = E>,
    E: From<Vec<FqxValue>>,
{
    type Item = E;

    type Ret<A> = HashMap<Vec<FqxValue>, Option<Self::Item>>;

    fn sum(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = OpReduce::reduce(v.into_iter().cloned(), |p, c| p + c);
            res.insert(k.clone(), a);
        }

        res
    }

    fn min(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = OpReduceFqxRow::reduce_fqx_row(v.into_iter().cloned(), get_min);
            res.insert(k.clone(), a);
        }

        res
    }

    fn max(self) -> Self::Ret<Self::Item> {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = OpReduceFqxRow::reduce_fqx_row(v.into_iter().cloned(), get_max);
            res.insert(k.clone(), a);
        }

        res
    }

    fn mean(self) -> Self::Ret<Self::Item> {
        let lens = (&self.0)
            .iter()
            .map(|(_, v)| v.into_iter().count())
            .collect::<Vec<_>>();
        self.sum()
            .into_iter()
            .zip(lens.into_iter())
            .map(|((k, v), len)| (k, v.map(|r| calc_mean(r, len))))
            .collect::<HashMap<_, _>>()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_agg {
    use once_cell::sync::Lazy;

    use super::*;
    use crate::adt::*;
    use crate::ops::{OpGroup, OpSelect};

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
    fn agg_self_success() {
        let data = DATA.clone();

        let a1 = (&data).sum();
        let a2 = (&data).max();
        let a3 = (&data).min();
        let a4 = (&data).mean();
        println!("{:?}", a1);
        println!("{:?}", a2);
        println!("{:?}", a3);
        println!("{:?}", a4);

        let a1 = data.clone().sum();
        let a2 = data.clone().max();
        let a3 = data.clone().min();
        let a4 = data.mean();
        println!("{:?}", a1);
        println!("{:?}", a2);
        println!("{:?}", a3);
        println!("{:?}", a4);
    }

    #[test]
    fn agg_slice_success() {
        let data = DATA.clone();

        let slice = &data[..];

        let a1 = slice.sum();
        let a2 = slice.max();
        let a3 = slice.min();
        let a4 = slice.mean();
        println!("{:?}", a1);
        println!("{:?}", a2);
        println!("{:?}", a3);
        println!("{:?}", a4);
    }

    #[test]
    fn agg_selected_success() {
        let data = DATA.clone();

        let selected = (&data)
            .select(&[0, 2])
            .group_by(|r| vec![r[0].clone()])
            .cloned()
            .mean();
        println!("{:?}", selected);

        let selected = data.select(&[0, 2]).group_by(|r| vec![r[0].clone()]).mean();
        println!("{:?}", selected);
    }

    #[test]
    fn agg_group_success() {
        let data = DATA.clone();

        let grp = (&data).group_by(|r| vec![r[0].clone()]);
        let grp = grp.cloned().mean();
        println!("{:?}", grp);

        let grp = data.group_by(|r| vec![r[0].clone()]);
        let grp = grp.mean();
        println!("{:?}", grp);
    }

    #[test]
    fn agg_selected_group_success() {
        let data = DATA.clone();

        let selected = (&data).select(&[0, 2]).group_by(|r| vec![r[0].clone()]);
        let selected = selected.cloned().mean();
        println!("{:?}", selected);

        let selected = data.select(&[0, 2]).group_by(|r| vec![r[0].clone()]).mean();
        println!("{:?}", selected);
    }
}
