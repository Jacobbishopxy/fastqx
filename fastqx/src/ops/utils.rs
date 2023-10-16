//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/09/28 18:02:11 Thursday
//! brief:

use std::cmp::Ordering;

use crate::adt::{FqxRowAbstract, FqxValue, FqxValueType};

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn merge_bool_to_ordering(b: bool) -> Ordering {
    if b {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

pub(crate) fn sort_bool_to_ordering(b: bool) -> Ordering {
    if b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn get_min(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Less) = a.partial_cmp(&b) {
        a
    } else {
        b
    }
}

pub(crate) fn get_max(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Greater) = a.partial_cmp(&b) {
        a
    } else {
        b
    }
}

pub(crate) fn calc_mean<I, V, E>(row_of_sum: E, count: usize) -> E
where
    E: Into<FqxRowAbstract<I, V>>,
    E: From<Vec<FqxValue>>,
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
{
    let inner = row_of_sum
        .into()
        .0
        .into_iter()
        .map(|e| {
            let numer = e
                .into()
                .try_cast(&FqxValueType::F64)
                .unwrap_or(FqxValue::Null);
            let denom = FqxValue::F64(count as f64);

            numer / denom
        })
        .collect::<Vec<_>>();

    E::from(inner)
}
