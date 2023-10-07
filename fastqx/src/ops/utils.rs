//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/09/28 18:02:11 Thursday
//! brief:

use std::cmp::Ordering;

use crate::adt::{FqxRow, FqxRowAbstract, FqxValue};
use crate::ops::OpReduce;

pub(crate) fn get_min(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Less) = a.partial_cmp(&b) {
        b
    } else {
        a
    }
}

pub(crate) fn get_max(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Greater) = a.partial_cmp(&b) {
        b
    } else {
        a
    }
}

pub(crate) fn calc_mean(row_of_sum: FqxRow, count: usize) -> FqxRow {
    let inner = row_of_sum
        .0
        .into_iter()
        .map(|e| e / FqxValue::U64(count as u64))
        .collect::<Vec<_>>();
    FqxRow(inner)
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) trait OpReduceFqxRow<T>
where
    Self: OpReduce<T>,
{
    fn reduce_fqx_row<F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(FqxValue, FqxValue) -> FqxValue;
}

impl<I, V, T, E> OpReduceFqxRow<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
    E: From<Vec<FqxValue>>,
{
    fn reduce_fqx_row<F>(self, mut f: F) -> Option<Self::Item>
    where
        F: FnMut(FqxValue, FqxValue) -> FqxValue,
    {
        let res = OpReduce::reduce(self, |pr, cr| {
            let inner = pr
                .into()
                .0
                .into_iter()
                .zip(cr.into().0.into_iter())
                .map(|(p, c)| f(p.into(), c.into()))
                .collect::<Vec<_>>();
            E::from(inner)
        });

        res
    }
}

impl<'a, I, V, T, E> OpReduceFqxRow<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: Into<FqxRowAbstract<I, V>> + Clone,
    E: From<Vec<FqxValue>>,
{
    fn reduce_fqx_row<F>(self, mut f: F) -> Option<Self::Item>
    where
        F: FnMut(FqxValue, FqxValue) -> FqxValue,
    {
        let res = OpReduce::reduce(self, |pr, cr| {
            let inner = pr
                .into()
                .0
                .into_iter()
                .zip(cr.into().0.into_iter())
                .map(|(p, c)| f(p.into(), c.into()))
                .collect::<Vec<_>>();
            E::from(inner)
        });

        res
    }
}
