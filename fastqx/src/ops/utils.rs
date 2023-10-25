//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/09/28 18:02:11 Thursday
//! brief:

use std::cmp::Ordering;

use itertools::Itertools;

use crate::adt::{FqxData, FqxRow, FqxRowAbstract, FqxValue, FqxValueType};
use crate::ops::OpGroup;
use crate::prelude::OpFold;

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

fn get_min(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Less) = a.partial_cmp(&b) {
        a
    } else {
        b
    }
}

fn get_max(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Greater) = a.partial_cmp(&b) {
        a
    } else {
        b
    }
}

pub(crate) fn get_row_min(r1: FqxRow, r2: FqxRow) -> FqxRow {
    let r = r1
        .into_iter()
        .zip(r2.into_iter())
        .map(|(v1, v2)| get_min(v1, v2))
        .collect();
    FqxRow(r)
}

pub(crate) fn get_row_max(r1: FqxRow, r2: FqxRow) -> FqxRow {
    let r = r1
        .into_iter()
        .zip(r2.into_iter())
        .map(|(v1, v2)| get_max(v1, v2))
        .collect();
    FqxRow(r)
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

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn fqx_data_left_join<N, S>(l: FqxData, r: FqxData, left_on: N, right_on: N) -> FqxData
where
    N: IntoIterator<Item = S>,
    S: ToString,
{
    let left_on = left_on
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>();
    let l_positions = r
        .columns()
        .into_iter()
        .enumerate()
        .fold(vec![], |mut acc, (i, e)| {
            if left_on.contains(e) {
                acc.push(i);
            }
            acc
        });

    let right_on = right_on
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>();
    let r_positions = r
        .columns()
        .into_iter()
        .enumerate()
        .fold(vec![], |mut acc, (i, e)| {
            if right_on.contains(e) {
                acc.push(i);
            }
            acc
        });

    let mut gr = r.group_by(|r| r.select_owned(&r_positions)).0;

    let d = Iterator::fold(l.data.into_iter(), vec![], |mut acc, row| {
        let keys = row.select_owned(&l_positions);

        if let Some(v) = gr.get(&keys) {
            // TODO: iter v.data, then row.extend each of them, then acc.extend
        }

        // TODO

        acc.push(row);
        acc
    });

    todo!()
}
