//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/10/11 13:31:46 Wednesday
//! brief:

use pyo3::types::PySlice;

use crate::adt::FqxRow;

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
    len1: isize,
    len2: isize,
    slice1: &PySlice,
    slice2: &PySlice,
) -> Vec<FqxRow>
where
    I: std::ops::Index<usize, Output = FqxRow>,
{
    let (start, stop, step, mut i) = de_slice(len1, slice1);
    let mut res = vec![];

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < len1 {
            res.push(FqxRow(slice_1d(&input[i as usize], len2, slice2)))
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }

    res
}

pub(crate) fn slice_data_mut<'m, I, O>(input: &'m mut I, len: isize, slice: &PySlice, val: Vec<O>)
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
