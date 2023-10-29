//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/10/11 13:31:46 Wednesday
//! brief:

use std::collections::{HashMap, VecDeque};

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

pub(crate) fn slice_vec<I, R>(input: &I, len: isize, slice: &PySlice) -> Vec<R>
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

pub(crate) fn slice_col_fqx(d: &FqxData, select: &[usize]) -> FqxData {
    let len = d.width();
    let mut columns = vec![];
    let mut types = vec![];

    for &p in select.iter() {
        if p < len {
            columns.push(d.columns[p].clone());
            types.push(d.types[p].clone());
        }
    }

    let data = d
        .iter()
        .map(|r| {
            select
                .iter()
                .filter_map(|&p| if p < len { Some(r[p].clone()) } else { None })
                .collect()
        })
        .collect();

    FqxData {
        columns,
        types,
        data,
    }
}

pub(crate) fn slice_fqx(d: &FqxData, row_slice: &PySlice, col_slice: &PySlice) -> FqxData {
    let (row_len, col_len) = d.shape();
    let (row_len, col_len) = (row_len as isize, col_len as isize);
    let (start, stop, step, mut i) = de_slice(row_len, row_slice);
    let mut data = vec![];

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < row_len {
            data.push(FqxRow(slice_vec(&d[i as usize], col_len, col_slice)))
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }

    let columns = slice_vec(&d.columns, row_len, col_slice);
    let types = slice_vec(&d.types, row_len, col_slice);

    FqxData {
        columns,
        types,
        data,
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn slice_col_mut<'m, I>(
    input: &'m mut I,
    row_len: isize,
    row_slice: &PySlice,
    col_idx: usize,
    val: Vec<FqxValue>,
) -> Result<()>
where
    I: std::ops::IndexMut<usize, Output = FqxRow>,
{
    let (start, stop, step, mut i) = de_slice(row_len, row_slice);
    let mut val_i = 0;

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < row_len {
            let v = val.get(val_i).ok_or(anyhow!("slice vec out of boundary"))?;
            input[i as usize].0.get_mut(col_idx).map(|e| *e = v.clone());

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

pub(crate) fn slice_hashmap_mut<'m, I>(
    input: &'m mut I,
    row_len: isize,
    slice1: &PySlice,
    mut rpc: HashMap<usize, VecDeque<FqxValue>>,
) -> Result<()>
where
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

    Ok(())
}

pub(crate) fn slice_vec_mut<'m, I, O>(
    input: &'m mut I,
    len: isize,
    slice: &PySlice,
    val: Vec<O>,
) -> Result<()>
where
    I: std::ops::IndexMut<usize, Output = O>,
    O: Sized + Clone,
{
    let (start, stop, step, mut i) = de_slice(len, slice);
    let mut val_i = 0;

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < len {
            let v = val.get(val_i).ok_or(anyhow!("slice vec out of boundary"))?;
            input[i as usize] = v.clone();
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

pub(crate) fn slice_fqx_mut(
    d: &mut FqxData,
    row_slice: &PySlice,
    col_slice: &PySlice,
    mut val: Vec<Vec<FqxValue>>,
) -> Result<()> {
    let (row_len, col_len) = d.shape();
    let (row_len, col_len) = (row_len as isize, col_len as isize);

    let (start, stop, step, mut i) = de_slice(row_len, row_slice);
    let mut val_i = 0;

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < row_len {
            let dest = val.get_mut(val_i).ok_or(anyhow!("out of boundary"))?;
            let v = std::mem::replace(dest, vec![]);
            slice_vec_mut(&mut d[i as usize], col_len, col_slice, v)?;
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
