//! file: pyidx.rs
//! author: Jacob Xie
//! date: 2023/10/27 23:54:11 Friday
//! brief:

use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Result};
use pyo3::types::PySlice;
use pyo3::{FromPyObject, Python};

use super::utils::{slice_1d, slice_2d, slice_2d_mut, slice_col_mut, slice_hashmap_mut};
use crate::adt::{FqxData, FqxValue};

// ================================================================================================
// new type: IdxSlice
// ================================================================================================

pub(crate) struct IdxSlice<'a>(&'a PySlice);

impl<'a> FromPyObject<'a> for IdxSlice<'a> {
    fn extract(ob: &'a pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(IdxSlice(ob.downcast::<PySlice>()?))
    }
}

// ================================================================================================
// PyIdx & PyIdxD
// ================================================================================================

#[derive(FromPyObject)]
pub(crate) enum PyIdx<'a> {
    // column-wise
    S(usize),
    VS(Vec<usize>),
    VST(Vec<String>),
    // row-wise
    PS(IdxSlice<'a>),
    // row-column
    PST((IdxSlice<'a>, IdxSlice<'a>)),
    PSTR((usize, IdxSlice<'a>)),
    PSTL((IdxSlice<'a>, usize)),
}

#[derive(FromPyObject)]
pub(crate) enum PyAssign {
    // column-wise
    Col(Vec<FqxValue>),
    ColIdx(HashMap<usize, Vec<FqxValue>>),
    ColName(HashMap<String, Vec<FqxValue>>),
    // row-wise
    Row(Vec<Vec<FqxValue>>),
}

impl<'a> PyIdx<'a> {
    pub fn slice_owned(self, py: Python<'_>, d: &FqxData) -> FqxData {
        match self {
            // column-wise
            PyIdx::S(s) => _col(d, [s].as_slice()),
            PyIdx::VS(vs) => _col(d, &vs),
            PyIdx::VST(vst) => _col(d, &_positions(d, &vst)),
            // row-wise
            PyIdx::PS(ps) => FqxData {
                columns: d.columns.clone(),
                types: d.types.clone(),
                data: slice_1d(&d.data, d.height() as isize, ps.0),
            },
            // row-column
            PyIdx::PST((r, c)) => _row_col(d, r.0, c.0),
            PyIdx::PSTR((r, c)) => {
                let r = _usize2slice(r, py);
                _row_col(d, r, c.0)
            }
            PyIdx::PSTL((r, c)) => {
                let c = _usize2slice(c, py);
                _row_col(d, r.0, c)
            }
        }
    }

    pub fn slice_mut(self, py: Python<'_>, d: &mut FqxData, val: PyAssign) -> Result<()> {
        let (h, w) = d.shape();
        let (ih, iw) = (h as isize, w as isize);
        match self {
            PyIdx::S(s) => {
                if let PyAssign::Col(c) = val {
                    if c.len() != h {
                        return Err(anyhow!("length mismatch"));
                    }
                    let row_slice = _full_slice(py);
                    slice_col_mut(d, ih, row_slice, s, c);
                }
            }
            PyIdx::VS(_) => {
                if let PyAssign::ColIdx(hm) = val {
                    let rpc = hm
                        .into_iter()
                        .map(|(k, v)| {
                            if v.len() != h {
                                return Err(anyhow!("length mismatch"));
                            }
                            Ok((k, VecDeque::from(v)))
                        })
                        .collect::<Result<HashMap<_, _>>>()?;
                    let row_slice = _full_slice(py);
                    slice_hashmap_mut(d, ih, row_slice, rpc);
                }
            }
            PyIdx::VST(vst) => {
                if let PyAssign::ColName(hm) = val {
                    let vst_len = vst.len();
                    let positions = d.get_positions(&vst);
                    if vst_len != positions.len() {
                        return Err(anyhow!("name not found in column"));
                    }
                    let mut name_map = vst
                        .into_iter()
                        .zip(positions.into_iter())
                        .collect::<HashMap<_, _>>();

                    let rpc = hm
                        .into_iter()
                        .map(|(k, v)| {
                            if v.len() != h {
                                return Err(anyhow!("length mismatch"));
                            }
                            Ok((name_map.remove(&k).unwrap(), VecDeque::from(v)))
                        })
                        .collect::<Result<HashMap<_, _>>>()?;
                    let row_slice = _full_slice(py);
                    slice_hashmap_mut(d, ih, row_slice, rpc);
                }
            }
            PyIdx::PS(_) => todo!(),
            PyIdx::PST(_) => todo!(),
            PyIdx::PSTR(_) => todo!(),
            PyIdx::PSTL(_) => todo!(),
        }

        Ok(())
    }
}

// ================================================================================================
// Helpers
// ================================================================================================

fn _positions(d: &FqxData, select: &[String]) -> Vec<usize> {
    select
        .iter()
        .filter_map(|c| d.columns.iter().position(|dc| dc == c))
        .collect()
}

fn _col(d: &FqxData, select: &[usize]) -> FqxData {
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

fn _col_mut(d: &mut FqxData, replace: HashMap<usize, Vec<FqxValue>>) -> Result<()> {
    // val: vector of columns

    Ok(())
}

fn _row_col(d: &FqxData, row_slice: &PySlice, col_slice: &PySlice) -> FqxData {
    let (h, w) = d.shape();
    let (h, w) = (h as isize, w as isize);
    let data = slice_2d(&d.data, h, w, row_slice, col_slice);
    let columns = slice_1d(&d.columns, w, col_slice);
    let types = slice_1d(&d.types, w, col_slice);
    FqxData {
        columns,
        types,
        data,
    }
}

fn _row_col_mut(
    d: &mut FqxData,
    row_slice: &PySlice,
    col_slice: &PySlice,
    val: Vec<Vec<FqxValue>>,
) -> Result<()> {
    todo!()
}

fn _full_slice(py: Python<'_>) -> &PySlice {
    PySlice::full(py)
}

fn _usize2slice(i: usize, py: Python<'_>) -> &PySlice {
    let i = i as isize;
    PySlice::new(py, i, i + 1, 1)
}
