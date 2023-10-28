//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/10/11 13:31:46 Wednesday
//! brief:

use std::collections::{HashMap, VecDeque};

use pyo3::types::PySlice;

use crate::adt::{FqxRow, FqxValue};

// ================================================================================================
// helpers
// ================================================================================================

fn de_slice(len: isize, slice: &PySlice) -> (isize, isize, isize, isize) {
    let mut start = slice
        .getattr("start")
        .and_then(|s| s.extract::<isize>())
        .unwrap_or(0);
    if start < 0 {
        start = len + start
    }
    let mut stop = slice
        .getattr("stop")
        .and_then(|s| s.extract::<isize>())
        .unwrap_or(len);
    if stop < 0 {
        stop = len + stop;
    }
    let mut step = slice
        .getattr("step")
        .and_then(|s| s.extract::<isize>())
        .unwrap_or(1);
    if step < 0 {
        step = -step;
    }

    let i = if start < stop { start } else { stop };

    (start, stop, step, i)
}

pub(crate) fn slice_1d<I, R>(input: &I, len: isize, slice: &PySlice) -> Vec<R>
where
    I: std::ops::Index<usize, Output = R>,
    R: Clone,
{
    let (start, stop, step, mut i) = de_slice(len, slice);
    let mut res = vec![];

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < len {
            res.push(input[i as usize].clone())
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }

    res
}

pub(crate) fn slice_2d<I>(
    input: &I,
    row_len: isize,
    col_len: isize,
    row_slice: &PySlice,
    col_slice: &PySlice,
) -> Vec<FqxRow>
where
    I: std::ops::Index<usize, Output = FqxRow>,
{
    let (start, stop, step, mut i) = de_slice(row_len, row_slice);
    let mut res = vec![];

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < row_len {
            res.push(FqxRow(slice_1d(&input[i as usize], col_len, col_slice)))
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }

    res
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn slice_col_mut<'m, I>(
    input: &'m mut I,
    row_len: isize,
    row_slice: &PySlice,
    col_idx: usize,
    mut val: Vec<FqxValue>,
) where
    I: std::ops::IndexMut<usize, Output = FqxRow>,
{
    let (start, stop, step, mut i) = de_slice(row_len, row_slice);
    let mut val_i = 0;

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < row_len {
            let v = std::mem::replace(&mut val[val_i], FqxValue::Null);
            // slice_1d_mut(&mut input[i as usize], col_len, col_slice, v);
            input[i as usize].0.get_mut(col_idx).map(|e| *e = v);

            val_i += 1;
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }
}

pub(crate) fn slice_hashmap_mut<'m, I>(
    input: &'m mut I,
    row_len: isize,
    slice1: &PySlice,
    mut rpc: HashMap<usize, VecDeque<FqxValue>>,
) where
    I: std::ops::IndexMut<usize, Output = FqxRow>,
{
    let (start, stop, step, mut i) = de_slice(row_len, slice1);

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < row_len {
            let d = rpc
                .iter_mut()
                .filter_map(|(&k, v)| v.pop_front().map(|e| (k, e)))
                .collect();
            input[i as usize].select_mut(d);
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }
}

pub(crate) fn slice_1d_mut<'m, I, O>(input: &'m mut I, len: isize, slice: &PySlice, val: Vec<O>)
where
    I: std::ops::IndexMut<usize, Output = O>,
    O: Sized + Clone,
{
    let (start, stop, step, mut i) = de_slice(len, slice);
    let mut val_i = 0;

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < len {
            input[i as usize] = val[val_i].clone();
            val_i += 1;
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }
}

pub(crate) fn slice_2d_mut<'m, I>(
    input: &'m mut I,
    row_len: isize,
    col_len: isize,
    row_slice: &PySlice,
    col_slice: &PySlice,
    mut val: Vec<Vec<FqxValue>>,
) where
    I: std::ops::IndexMut<usize, Output = FqxRow>,
{
    let (start, stop, step, mut i) = de_slice(row_len, row_slice);
    let mut val_i = 0;

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < row_len {
            let v = std::mem::replace(&mut val[val_i], vec![]);
            slice_1d_mut(&mut input[i as usize], col_len, col_slice, v);
            val_i += 1;
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }
}
