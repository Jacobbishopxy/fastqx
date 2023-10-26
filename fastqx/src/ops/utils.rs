//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/09/28 18:02:11 Thursday
//! brief:

use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::adt::{FqxData, FqxRow, FqxRowAbstract, FqxValue, FqxValueType};

///////////////////////////////////////////////////////////////////////////////////////////////////

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

fn _group<I>(iter: I, pos: &[usize]) -> HashMap<Vec<FqxValue>, Vec<FqxRow>>
where
    I: IntoIterator<Item = FqxRow>,
{
    let mut gr = HashMap::new();
    Itertools::group_by(iter.into_iter(), |row| row.select_owned(pos))
        .into_iter()
        .for_each(|(k, g)| gr.entry(k).or_insert(vec![]).extend(g.collect_vec()));
    gr
}

pub(crate) fn _join<N, S>(
    l: FqxData,
    r: FqxData,
    left_on: N,
    right_on: N,
    ignore_missing: bool,
) -> FqxData
where
    N: IntoIterator<Item = S>,
    S: ToString,
{
    let l_positions = l.get_positions(left_on);
    let r_positions = r.get_positions(right_on);
    let r_empty_row = r.empty_row();

    let gr = _group(r.data, &r_positions);

    let d = Iterator::fold(l.data.into_iter(), vec![], |mut acc, mut row| {
        let keys = row.select_owned(&l_positions);

        match gr.get(&keys) {
            Some(v) => {
                for r in v.into_iter() {
                    let mut new_row = row.clone();
                    new_row.extend(r.clone());
                    acc.push(new_row);
                }
            }
            None => {
                if !ignore_missing {
                    let empty_row = r_empty_row.clone();
                    row.extend(empty_row);
                    acc.push(row);
                }
            }
        }

        acc
    });

    let mut c = l.columns;
    let mut t = l.types;
    c.extend(r.columns);
    t.extend(r.types);

    FqxData {
        columns: c,
        types: t,
        data: d,
    }
}
