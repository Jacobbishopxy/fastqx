//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/10/11 13:31:46 Wednesday
//! brief:

use anyhow::{anyhow, Result};
use pyo3::types::PySlice;

use crate::adt::{FqxData, FqxRow, FqxValue};

// ================================================================================================
// helpers
// ================================================================================================

// decode Python slice
// len: the length of a container to be sliced
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

///////////////////////////////////////////////////////////////////////////////////////////////////

fn _slice_op<R>(len: isize, slice: &PySlice, f: impl Fn(usize) -> R) -> Vec<R> {
    let (start, stop, step, mut i) = de_slice(len, slice);
    let mut res = vec![];

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < len {
            res.push(f(i as usize));
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }

    res
}

pub(crate) fn slice_vec<I, E>(input: &I, len: isize, slice: &PySlice) -> Vec<E>
where
    I: std::ops::Index<usize, Output = E>,
    E: Clone,
{
    let f = |i| input[i].clone();
    _slice_op(len, slice, f)
}

pub(crate) fn slice_data(d: &Vec<FqxRow>, row_slice: &PySlice, col_slice: &PySlice) -> Vec<FqxRow> {
    let row_len = d.len() as isize;
    let col_len = d.get(0).map(|r| r.len()).unwrap_or(0) as isize;

    let f = |i| FqxRow(slice_vec(&d[i], col_len, col_slice));
    _slice_op(row_len, row_slice, f)
}

pub(crate) fn slice_fqx(d: &FqxData, row_slice: &PySlice, col_slice: &PySlice) -> FqxData {
    let col_len = d.width() as isize;

    let columns = slice_vec(&d.columns, col_len, col_slice);
    let types = slice_vec(&d.types, col_len, col_slice);
    let data = slice_data(&d.data, row_slice, col_slice);

    FqxData {
        columns,
        types,
        data,
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn _slice_mut_op(
    row_len: isize,
    row_slice: &PySlice,
    mut f: impl FnMut(usize, usize) -> Result<()>,
) -> Result<()> {
    let (start, stop, step, mut i) = de_slice(row_len, row_slice);
    let mut val_i = 0;

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < row_len {
            f(val_i, i as usize)?;

            val_i += 1;
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }

    Ok(())
}

pub(crate) fn slice_vec_mut<I, O>(
    input: &mut I,
    row_len: isize,
    row_slice: &PySlice,
    mut val: Vec<O>,
) -> Result<()>
where
    I: std::ops::IndexMut<usize, Output = O>,
{
    let f = |vi, i| {
        let v = val
            .get_mut(vi)
            .ok_or(anyhow!("slice vec out of boundary"))?;
        std::mem::swap(&mut input[i], v);

        Ok(())
    };
    _slice_mut_op(row_len, row_slice, f)
}

pub(crate) fn slice_fqx_mut(
    d: &mut FqxData,
    row_slice: &PySlice,
    col_slice: &PySlice,
    mut val: Vec<Vec<FqxValue>>,
) -> Result<()> {
    let (row_len, col_len) = d.shape();
    let (row_len, col_len) = (row_len as isize, col_len as isize);

    let f = |vi, i| {
        let dest = val.get_mut(vi).ok_or(anyhow!("out of boundary"))?;
        let v = std::mem::replace(dest, vec![]);
        slice_vec_mut(&mut d[i], col_len, col_slice, v)?;

        Ok(())
    };
    _slice_mut_op(row_len, row_slice, f)
}
