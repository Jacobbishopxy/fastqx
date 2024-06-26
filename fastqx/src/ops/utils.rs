//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/09/28 18:02:11 Thursday
//! brief:

use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::{EitherOrBoth, Itertools};

use crate::adt::{FqxD, FqxValue, FqxValueType, RowProps, SeqAppend, SeqSlice};
use crate::ops::FqxLazyGroup;

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn _sort_bool_to_ordering(b: bool) -> Ordering {
    if b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn _get_min<'c>(a: &'c FqxValue, b: &'c FqxValue) -> &'c FqxValue {
    if let Some(Ordering::Less) = a.partial_cmp(b) {
        a
    } else {
        b
    }
}

fn _get_max<'c>(a: &'c FqxValue, b: &'c FqxValue) -> &'c FqxValue {
    if let Some(Ordering::Greater) = a.partial_cmp(b) {
        a
    } else {
        b
    }
}

pub(crate) fn _get_row_sum<R: RowProps>(r1: &R, r2: &R) -> R {
    let r = r1.iter().zip(r2.iter()).map(|(v1, v2)| v1 + v2).collect();
    R::from_values(r)
}

pub(crate) fn _get_row_min<R: RowProps>(r1: &R, r2: &R) -> R {
    let r = r1
        .iter()
        .zip(r2.iter())
        .map(|(v1, v2)| _get_min(v1, v2))
        .cloned()
        .collect();
    R::from_values(r)
}

pub(crate) fn _get_row_max<R: RowProps>(r1: &R, r2: &R) -> R {
    let r = r1
        .iter()
        .zip(r2.iter())
        .map(|(v1, v2)| _get_max(v1, v2))
        .cloned()
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

pub(crate) fn lazy_agg_ctor<'a, U>(lz: &FqxLazyGroup<'a, U>, d: Vec<U::RowT>) -> U
where
    U: FqxD,
{
    let mut new_loc = lz.selected_keys.clone();
    new_loc.extend(lz.selected_aggs.clone());

    let new_cols = lz.d.columns_().clone().takes(new_loc.clone());
    let new_typs = lz.d.types_().clone().takes(new_loc);

    U::cst(new_cols, new_typs, d)
}

fn _group<I, R>(iter: I, pos: &[usize]) -> HashMap<Vec<FqxValue>, Vec<R>>
where
    I: IntoIterator<Item = R>,
    R: RowProps,
{
    let mut gr = HashMap::new();
    Itertools::chunk_by(iter.into_iter(), |row| row.select_vals_owned(pos))
        .into_iter()
        .for_each(|(k, g)| gr.entry(k).or_insert(vec![]).extend(g.collect_vec()));
    gr
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn _join<U, N, S>(l: U, r: U, left_on: &N, right_on: &N, ignore_missing: bool) -> U
where
    U: FqxD,
    for<'a> &'a N: IntoIterator<Item = &'a S>,
    S: AsRef<str>,
{
    let l_positions = l.columns_position(left_on);
    let r_positions = r.columns_position(right_on);
    let r_empty_row = r.empty_row();

    let (l_cols, l_types, l_data) = l.dcst();
    let (r_cols, r_types, r_data) = r.dcst();

    let gr = _group(r_data, &r_positions);

    let d = Iterator::fold(l_data.into_iter(), vec![], |mut acc, mut row| {
        let keys = row.select_vals_owned(&l_positions);

        match gr.get(&keys) {
            Some(v) => {
                for r in v.into_iter() {
                    let mut new_row = row.clone();
                    new_row.extend(r.clone().iter_owned());
                    acc.push(new_row);
                }
            }
            None => {
                if !ignore_missing {
                    let empty_row = r_empty_row.clone();
                    row.extend(empty_row.iter_owned());
                    acc.push(row);
                }
            }
        }

        acc
    });

    let mut c = l_cols;
    let mut t = l_types;

    c.append(r_cols);
    t.append(r_types);

    U::cst(c, t, d)
}

fn _l_empty_extends<R: RowProps>(le: &R, r: Vec<R>) -> Vec<R> {
    r.into_iter()
        .map(|w| {
            let mut l = le.clone();
            l.extend(w.iter_owned());
            l
        })
        .collect_vec()
}

fn _r_empty_extends<R: RowProps>(re: &R, l: Vec<R>) -> Vec<R> {
    l.into_iter()
        .map(|mut w| {
            w.extend(re.clone().iter_owned());
            w
        })
        .collect_vec()
}

fn _lr_extends<R: RowProps>(l: Vec<R>, r: Vec<R>) -> Vec<R> {
    Itertools::cartesian_product(l.into_iter(), r.into_iter())
        .into_iter()
        .map(|(mut row_l, row_r)| {
            row_l.extend(row_r.iter_owned());
            row_l
        })
        .collect_vec()
}

pub(crate) fn _outer_join<U, N, S>(l: U, r: U, left_on: &N, right_on: &N) -> U
where
    U: FqxD,
    for<'a> &'a N: IntoIterator<Item = &'a S>,
    S: AsRef<str>,
{
    let l_positions = l.columns_position(left_on);
    let r_positions = r.columns_position(right_on);
    let l_empty_row = l.empty_row();
    let r_empty_row = r.empty_row();

    let (l_cols, l_types, l_data) = l.dcst();
    let (r_cols, r_types, r_data) = r.dcst();

    let gl = _group(l_data, &l_positions);
    let gr = _group(r_data, &r_positions);

    let mut d = vec![];

    Itertools::merge_join_by(
        gl.into_iter().sorted_by_key(|x| x.0.clone()),
        gr.into_iter().sorted_by_key(|x| x.0.clone()),
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

    let mut c = l_cols;
    let mut t = l_types;
    c.append(r_cols);
    t.append(r_types);

    U::cst(c, t, d)
}
