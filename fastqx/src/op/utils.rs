//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/09/28 18:02:11 Thursday
//! brief:

use std::cmp::Ordering;

use crate::adt::{FqxRow, FqxValue};

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
