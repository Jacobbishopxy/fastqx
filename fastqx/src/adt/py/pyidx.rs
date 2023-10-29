//! file: pyidx.rs
//! author: Jacob Xie
//! date: 2023/10/27 23:54:11 Friday
//! brief:

use std::collections::HashMap;

use anyhow::Result;
use pyo3::types::PySlice;
use pyo3::{FromPyObject, Python};

use super::utils::*;
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
    // single dir
    Vec(Vec<FqxValue>),
    // column-wise
    ColIdx(HashMap<usize, Vec<FqxValue>>),
    ColName(HashMap<String, Vec<FqxValue>>),
    // row-wise
    Row(Vec<Vec<FqxValue>>),
}

impl<'a> PyIdx<'a> {
    pub fn slice_owned(self, py: Python<'_>, d: &FqxData) -> FqxData {
        match self {
            // column-wise
            PyIdx::S(s) => slice_col_fqx(d, [s].as_slice()),
            PyIdx::VS(vs) => slice_col_fqx(d, &vs),
            PyIdx::VST(vst) => slice_col_fqx(d, &d.get_positions(&vst)),
            // row-wise
            PyIdx::PS(ps) => FqxData {
                columns: d.columns.clone(),
                types: d.types.clone(),
                data: slice_vec(&d.data, d.height() as isize, ps.0),
            },
            // row-column
            PyIdx::PST((r, c)) => slice_fqx(d, r.0, c.0),
            PyIdx::PSTR((r, c)) => slice_fqx(d, _usize2slice(r, py), c.0),
            PyIdx::PSTL((r, c)) => slice_fqx(d, r.0, _usize2slice(c, py)),
        }
    }

    pub fn slice_mut(self, py: Python<'_>, d: &mut FqxData, val: PyAssign) -> Result<()> {
        let h = d.height();
        let ih = h as isize;
        match self {
            PyIdx::S(s) => {
                if let PyAssign::Vec(v) = val {
                    let row_slice = _full_slice(py);
                    slice_col_mut(d, ih, row_slice, s, v)?;
                }
            }
            PyIdx::VS(_) => {
                if let PyAssign::ColIdx(hm) = val {
                    let row_slice = _full_slice(py);
                    let rpc = _gen_rpc1(h, hm);
                    slice_hashmap_mut(d, ih, row_slice, rpc)?;
                }
            }
            PyIdx::VST(vst) => {
                if let PyAssign::ColName(hm) = val {
                    let pos = d.get_positions(&vst);
                    let rpc = _gen_rpc2(h, pos, vst, hm);
                    let row_slice = _full_slice(py);
                    slice_hashmap_mut(d, ih, row_slice, rpc)?;
                }
            }
            PyIdx::PS(ps) => {
                if let PyAssign::Row(rows) = val {
                    let col_slice = _full_slice(py);
                    slice_fqx_mut(d, ps.0, col_slice, rows)?;
                }
            }
            PyIdx::PST((rs, cs)) => {
                if let PyAssign::Row(rows) = val {
                    slice_fqx_mut(d, rs.0, cs.0, rows)?;
                }
            }
            PyIdx::PSTR((r, cs)) => {
                if let PyAssign::Vec(v) = val {
                    let row_slice = _usize2slice(r, py);
                    slice_fqx_mut(d, row_slice, cs.0, vec![v])?;
                }
            }
            PyIdx::PSTL((rs, c)) => {
                if let PyAssign::Vec(v) = val {
                    slice_col_mut(d, ih, rs.0, c, v)?;
                }
            }
        }

        Ok(())
    }
}

// ================================================================================================
// Helpers
// ================================================================================================

fn _full_slice(py: Python<'_>) -> &PySlice {
    PySlice::full(py)
}

fn _usize2slice(i: usize, py: Python<'_>) -> &PySlice {
    let i = i as isize;
    PySlice::new(py, i, i + 1, 1)
}
