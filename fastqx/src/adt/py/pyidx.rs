//! file: pyidx.rs
//! author: Jacob Xie
//! date: 2023/10/27 23:54:11 Friday
//! brief:

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
    R(isize),                          // a single row
    RS(IdxSlice<'a>),                  // row slice
    V((isize, isize)),                 // a single value
    RSS((IdxSlice<'a>, IdxSlice<'a>)), // row-col slice
    RIS((isize, IdxSlice<'a>)),        // a single row, slice of value
    RSI((IdxSlice<'a>, isize)),        // a slice of row, single value
}

// IMPORTANT: The order of the variants effects deserialization!
#[derive(Debug, FromPyObject)]
pub(crate) enum PyAssign {
    D2(Vec<Vec<FqxValue>>),
    D1(Vec<FqxValue>),
    S(FqxValue),
}

impl<'a> PyIdx<'a> {
    pub fn slice_owned(self, py: Python<'_>, d: &FqxData) -> FqxData {
        match self {
            PyIdx::R(r) => FqxData {
                columns: d.columns.clone(),
                types: d.types.clone(),
                data: slice_vec(&d.data, d.height() as isize, _isize2slice(r, py)),
            },
            PyIdx::RS(ps) => FqxData {
                columns: d.columns.clone(),
                types: d.types.clone(),
                data: slice_vec(&d.data, d.height() as isize, ps.0),
            },
            PyIdx::V((r, c)) => slice_fqx(d, _isize2slice(r, py), _isize2slice(c, py)),
            PyIdx::RSS((r, c)) => slice_fqx(d, r.0, c.0),
            PyIdx::RIS((r, c)) => slice_fqx(d, _isize2slice(r, py), c.0),
            PyIdx::RSI((r, c)) => slice_fqx(d, r.0, _isize2slice(c, py)),
        }
    }

    pub fn slice_mut(self, py: Python<'_>, d: &mut FqxData, val: PyAssign) -> Result<()> {
        match self {
            PyIdx::R(r) => {
                if let PyAssign::D1(d1) = val {
                    let row_slice = _isize2slice(r, py);
                    let col_slice = _full_slice(py);
                    slice_fqx_mut(d, row_slice, col_slice, vec![d1])?;
                }
            }
            PyIdx::RS(rs) => {
                if let PyAssign::D2(d2) = val {
                    let col_slice = _full_slice(py);
                    slice_fqx_mut(d, rs.0, col_slice, d2)?;
                }
            }
            PyIdx::V((r, c)) => {
                if let PyAssign::S(v) = val {
                    let row_slice = _isize2slice(r, py);
                    let col_slice = _isize2slice(c, py);
                    slice_fqx_mut(d, row_slice, col_slice, vec![vec![v]])?;
                }
            }
            PyIdx::RSS((rs, cs)) => {
                if let PyAssign::D2(d2) = val {
                    slice_fqx_mut(d, rs.0, cs.0, d2)?;
                }
            }
            PyIdx::RIS((r, cs)) => {
                if let PyAssign::D1(d1) = val {
                    let row_slice = _isize2slice(r, py);
                    slice_fqx_mut(d, row_slice, cs.0, vec![d1])?;
                }
            }
            PyIdx::RSI((rs, c)) => {
                if let PyAssign::D1(d1) = val {
                    let col_slice = _isize2slice(c, py);
                    slice_fqx_mut(
                        d,
                        rs.0,
                        col_slice,
                        d1.into_iter().map(|e| vec![e]).collect(),
                    )?;
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

fn _isize2slice(i: isize, py: Python<'_>) -> &PySlice {
    PySlice::new(py, i, i + 1, 1)
}
