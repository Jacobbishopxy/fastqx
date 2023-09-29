//! file: agg.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:21:51 Sunday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::op::utils::*;
use crate::op::{FqxGroup, FqxRowSelect, FqxSlice, OpFoldFqxRow, OpReduce, OpReduceFqxRow};

// ================================================================================================
// OpAgg
// ================================================================================================

pub trait OpAgg<O> {
    type Ret<A>;

    fn sum(self) -> Self::Ret<O>;

    fn min(self) -> Self::Ret<O>;

    fn max(self) -> Self::Ret<O>;

    fn mean(self) -> Self::Ret<O>;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxData

impl OpAgg<FqxRow> for FqxData {
    type Ret<A> = Option<A>;

    fn sum(self) -> Self::Ret<FqxRow> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Self::Ret<FqxRow> {
        self.reduce_fqx_row(get_min)
    }

    fn max(self) -> Self::Ret<FqxRow> {
        self.reduce_fqx_row(get_max)
    }

    fn mean(self) -> Self::Ret<FqxRow> {
        let len = self.height();
        self.sum().map(|r| calc_mean(r, len))
    }
}

impl<'a> OpAgg<FqxRow> for &'a FqxData {
    type Ret<A> = Option<A>;

    fn sum(self) -> Self::Ret<FqxRow> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Self::Ret<FqxRow> {
        self.reduce_fqx_row(get_min)
    }

    fn max(self) -> Self::Ret<FqxRow> {
        self.reduce_fqx_row(get_max)
    }

    fn mean(self) -> Self::Ret<FqxRow> {
        let len = self.height();
        self.sum().map(|r| calc_mean(r, len))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSlice

impl<'a> OpAgg<FqxRow> for &'a FqxSlice {
    type Ret<A> = Option<A>;

    fn sum(self) -> Self::Ret<FqxRow> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Self::Ret<FqxRow> {
        self.reduce_fqx_row(get_min)
    }

    fn max(self) -> Self::Ret<FqxRow> {
        self.reduce_fqx_row(get_max)
    }

    fn mean(self) -> Self::Ret<FqxRow> {
        let len = self.0.len();
        self.sum().map(|r| calc_mean(r, len))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup

impl OpAgg<FqxRow> for FqxGroup<Vec<FqxRow>> {
    type Ret<A> = HashMap<FqxValue, Option<A>>;

    fn sum(self) -> Self::Ret<FqxRow> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce(|p, c| p + c);
            res.insert(k, a);
        }

        res
    }

    fn min(self) -> Self::Ret<FqxRow> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_min);
            res.insert(k, a);
        }

        res
    }

    fn max(self) -> Self::Ret<FqxRow> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_max);
            res.insert(k, a);
        }

        res
    }

    fn mean(self) -> Self::Ret<FqxRow> {
        let len = self.0.len();
        self.sum()
            .into_iter()
            .map(|(k, v)| (k, v.map(|r| calc_mean(r, len))))
            .collect::<HashMap<_, _>>()
    }
}

impl<'a> OpAgg<FqxRow> for FqxGroup<Vec<&'a FqxRow>> {
    type Ret<A> = HashMap<FqxValue, Option<A>>;

    fn sum(self) -> Self::Ret<FqxRow> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce(|p, c| p + c);
            res.insert(k, a);
        }

        res
    }

    fn min(self) -> Self::Ret<FqxRow> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_min);
            res.insert(k, a);
        }

        res
    }

    fn max(self) -> Self::Ret<FqxRow> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_max);
            res.insert(k, a);
        }

        res
    }

    fn mean(self) -> Self::Ret<FqxRow> {
        let len = self.0.len();
        self.sum()
            .into_iter()
            .map(|(k, v)| (k, v.map(|r| calc_mean(r, len))))
            .collect::<HashMap<_, _>>()
    }
}

impl OpAgg<FqxRowSelect<FqxValue>> for FqxGroup<Vec<FqxRowSelect<FqxValue>>> {
    type Ret<A> = HashMap<FqxValue, Option<A>>;

    fn sum(self) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v
                .into_iter()
                .map(FqxRow::from)
                .reduce(|p, c| p + c)
                .map(FqxRowSelect::from);
            res.insert(k, a);
        }

        res
    }

    fn min(self) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_min);
            res.insert(k, a);
        }

        res
    }

    fn max(self) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_max);
            res.insert(k, a);
        }

        res
    }

    fn mean(self) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let len = self.0.len();
        self.sum()
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.map(|r| FqxRowSelect::from(calc_mean(FqxRow::from(r), len))),
                )
            })
            .collect::<HashMap<_, _>>()
    }
}

impl<'a> OpAgg<FqxRowSelect<FqxValue>> for FqxGroup<Vec<FqxRowSelect<&'a FqxValue>>> {
    type Ret<A> = HashMap<FqxValue, Option<A>>;

    fn sum(self) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v
                .iter()
                .cloned()
                .map(FqxRow::from)
                .reduce(|p, c| p + c)
                .map(FqxRowSelect::from);
            res.insert(k, a);
        }

        res
    }

    fn min(self) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_min);
            res.insert(k, a);
        }

        res
    }

    fn max(self) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.reduce_fqx_row(get_max);
            res.insert(k, a);
        }

        res
    }

    fn mean(self) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let len = self.0.len();
        self.sum()
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.map(|r| FqxRowSelect::from(calc_mean(FqxRow::from(r), len))),
                )
            })
            .collect::<HashMap<_, _>>()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSelect

impl OpAgg<FqxRow> for Vec<FqxRowSelect<FqxValue>> {
    type Ret<A> = Option<A>;

    fn sum(self) -> Self::Ret<FqxRow> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|ini| iter.fold(FqxRow::from(ini), |acc, c| acc + FqxRow::from(c)))
    }

    fn min(mut self) -> Self::Ret<FqxRow> {
        self.pop()
            .map(|fst| self.fold_fqx_row(FqxRow::from(fst), get_min))
    }

    fn max(mut self) -> Self::Ret<FqxRow> {
        self.pop()
            .map(|fst| self.fold_fqx_row(FqxRow::from(fst), get_max))
    }

    fn mean(self) -> Self::Ret<FqxRow> {
        let len = self.len();
        self.sum().map(|r| calc_mean(r, len))
    }
}

impl<'a> OpAgg<FqxRow> for Vec<FqxRowSelect<&'a FqxValue>> {
    type Ret<A> = Option<A>;

    fn sum(self) -> Self::Ret<FqxRow> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|ini| iter.fold(FqxRow::from(ini), |acc, c| acc + FqxRow::from(c)))
    }

    fn min(mut self) -> Self::Ret<FqxRow> {
        self.pop()
            .map(|fst| self.fold_fqx_row(FqxRow::from(fst), get_min))
    }

    fn max(mut self) -> Self::Ret<FqxRow> {
        self.pop()
            .map(|fst| self.fold_fqx_row(FqxRow::from(fst), get_max))
    }

    fn mean(self) -> Self::Ret<FqxRow> {
        let len = self.len();
        self.sum().map(|r| calc_mean(r, len))
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
    use crate::op::{OpGroup, OpSelect};

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
    fn agg_group_success() {
        let data = DATA.clone();

        let grp = (&data).group_by(|r| r[0].clone()).mean();
        println!("{:?}", grp);

        let grp = data.group_by(|r| r[0].clone()).mean();
        println!("{:?}", grp);
    }

    #[test]
    fn agg_select_group_success() {
        let data = DATA.clone();

        // TODO: index of FqxRowSelect?
        let selected = data.select(&[0, 2]).group_by(|r| r.0[0].clone()).mean();
        println!("{:?}", selected);
    }
}
