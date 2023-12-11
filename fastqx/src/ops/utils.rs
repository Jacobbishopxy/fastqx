//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/09/28 18:02:11 Thursday
//! brief:

use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::{EitherOrBoth, Itertools};

use crate::adt::{FqxData, FqxRow, FqxValue, FqxValueType, RowProps};

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn _sort_bool_to_ordering(b: bool) -> Ordering {
    if b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn _get_min(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Less) = a.partial_cmp(&b) {
        a
    } else {
        b
    }
}

fn _get_max(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Greater) = a.partial_cmp(&b) {
        a
    } else {
        b
    }
}

pub(crate) fn _get_row_min<R: RowProps>(r1: R, r2: R) -> R {
    let r = r1
        .iter_owned()
        .zip(r2.iter_owned())
        .map(|(v1, v2)| _get_min(v1, v2))
        .collect();
    R::from_values(r)
}

pub(crate) fn _get_row_max<R: RowProps>(r1: R, r2: R) -> R {
    let r = r1
        .iter_owned()
        .zip(r2.iter_owned())
        .map(|(v1, v2)| _get_max(v1, v2))
        .collect();
    R::from_values(r)
}

pub(crate) fn _calc_mean<R: RowProps>(row_of_sum: R, count: usize) -> R {
    let inner = row_of_sum
        .iter_owned()
        .map(|e| {
            let nmr = e.try_cast(&FqxValueType::F64).unwrap_or(FqxValue::Null);
            let dnm = FqxValue::F64(count as f64);

            nmr / dnm
        })
        .collect::<Vec<_>>();

    R::from_values(inner)
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

fn _l_empty_extends(le: &FqxRow, r: Vec<FqxRow>) -> Vec<FqxRow> {
    r.into_iter()
        .map(|w| {
            let mut l = le.clone();
            l.extend(w);
            l
        })
        .collect_vec()
}

fn _r_empty_extends(re: &FqxRow, l: Vec<FqxRow>) -> Vec<FqxRow> {
    l.into_iter()
        .map(|mut w| {
            w.extend(re.clone());
            w
        })
        .collect_vec()
}

fn _lr_extends(l: Vec<FqxRow>, r: Vec<FqxRow>) -> Vec<FqxRow> {
    Itertools::cartesian_product(l.into_iter(), r.into_iter())
        .into_iter()
        .map(|(mut row_l, row_r)| {
            row_l.extend(row_r);
            row_l
        })
        .collect_vec()
}

pub(crate) fn _outer_join<N, S>(l: FqxData, r: FqxData, left_on: N, right_on: N) -> FqxData
where
    N: IntoIterator<Item = S>,
    S: ToString,
{
    let l_positions = l.get_positions(left_on);
    let r_positions = r.get_positions(right_on);
    let l_empty_row = l.empty_row();
    let r_empty_row = r.empty_row();

    let gl = _group(l.data, &l_positions);
    let gr = _group(r.data, &r_positions);

    let mut d = vec![];

    Itertools::merge_join_by(
        gl.into_iter().sorted(),
        gr.into_iter().sorted(),
        |(i, _), (j, _)| i.cmp(j),
    )
    .into_iter()
    .for_each(|e| match e {
        EitherOrBoth::Both((_, l), (_, r)) => {
            let nw = _lr_extends(l, r);
            d.extend(nw);
        }
        EitherOrBoth::Left((_, l)) => {
            let l = _r_empty_extends(&r_empty_row, l);
            d.extend(l);
        }
        EitherOrBoth::Right((_, r)) => {
            let r = _l_empty_extends(&l_empty_row, r);
            d.extend(r);
        }
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
