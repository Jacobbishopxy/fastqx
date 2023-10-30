//! file: pyidx.rs
//! author: Jacob Xie
//! date: 2023/10/27 23:54:11 Friday
//! brief:

use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use pyo3::types::PySlice;

use super::utils::*;
use crate::adt::{FqxData, FqxRow, FqxValue};

// ================================================================================================
// new type: IdxSlice
// ================================================================================================

#[derive(Debug)]
pub(crate) struct IdxSlice<'a>(&'a PySlice);

impl<'a> FromPyObject<'a> for IdxSlice<'a> {
    fn extract(ob: &'a pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(IdxSlice(ob.downcast::<PySlice>()?))
    }
}

// ================================================================================================
// PyIdx & PyIdxD
// ================================================================================================

#[derive(Debug, FromPyObject)]
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
            PyIdx::RS(rs) => FqxData {
                columns: d.columns.clone(),
                types: d.types.clone(),
                data: slice_vec(&d.data, d.height() as isize, rs.0),
            },
            PyIdx::V((r, c)) => slice_fqx(d, _isize2slice(r, py), _isize2slice(c, py)),
            PyIdx::RSS((r, c)) => slice_fqx(d, r.0, c.0),
            PyIdx::RIS((r, c)) => slice_fqx(d, _isize2slice(r, py), c.0),
            PyIdx::RSI((r, c)) => slice_fqx(d, r.0, _isize2slice(c, py)),
        }
    }

    pub fn slice_d2(self, py: Python<'_>, d: &PyX) -> Vec<FqxRow> {
        match self {
            PyIdx::R(r) => slice_data(&d.0, _isize2slice(r, py), _full_slice(py)),
            PyIdx::RS(rs) => slice_data(&d.0, rs.0, _full_slice(py)),
            PyIdx::V((r, c)) => slice_data(&d.0, _isize2slice(r, py), _isize2slice(c, py)),
            PyIdx::RSS((r, c)) => slice_data(&d.0, r.0, c.0),
            PyIdx::RIS((r, c)) => slice_data(&d.0, _isize2slice(r, py), c.0),
            PyIdx::RSI((r, c)) => slice_data(&d.0, r.0, _isize2slice(c, py)),
        }
    }

    pub fn slice_mut(self, py: Python<'_>, d: &mut FqxData, asn: PyAssign) -> Result<()> {
        let (row_slice, col_slice, val) = match (self, asn) {
            (PyIdx::R(r), PyAssign::D1(d1)) => {
                let row_slice = _isize2slice(r, py);
                let col_slice = _full_slice(py);
                let val = vec![d1];
                (row_slice, col_slice, val)
            }
            (PyIdx::RS(rs), PyAssign::D2(d2)) => {
                let row_slice = rs.0;
                let col_slice = _full_slice(py);
                let val = d2;
                (row_slice, col_slice, val)
            }
            (PyIdx::V((r, c)), PyAssign::S(v)) => {
                let row_slice = _isize2slice(r, py);
                let col_slice = _isize2slice(c, py);
                let val = vec![vec![v]];
                (row_slice, col_slice, val)
            }
            (PyIdx::RSS((rs, cs)), PyAssign::D2(d2)) => {
                let row_slice = rs.0;
                let col_slice = cs.0;
                let val = d2;
                (row_slice, col_slice, val)
            }
            (PyIdx::RIS((r, cs)), PyAssign::D1(d1)) => {
                let row_slice = _isize2slice(r, py);
                let col_slice = cs.0;
                let val = vec![d1];
                (row_slice, col_slice, val)
            }
            (PyIdx::RSI((rs, c)), PyAssign::D1(d1)) => {
                let row_slice = rs.0;
                let col_slice = _isize2slice(c, py);
                let val = d1.into_iter().map(|e| vec![e]).collect();
                (row_slice, col_slice, val)
            }
            (i, a) => {
                return Err(anyhow!("mismatch assignment, idx: {:?}, asn: {:?}", i, a));
            }
        };

        slice_fqx_mut(d, row_slice, col_slice, val)?;

        Ok(())
    }
}

// ================================================================================================
// PyX
// ================================================================================================

#[pyclass]
#[pyo3(name = "X")]
pub(crate) struct PyX(pub(crate) Vec<FqxRow>);

#[pymethods]
impl PyX {
    fn __repr__(&self) -> PyResult<String> {
        Ok(serde_json::to_string(&self.0).map_err(anyhow::Error::msg)?)
    }

    fn __getitem__(&self, py: Python<'_>, idx: PyObject) -> PyResult<Vec<FqxRow>> {
        let idx = idx.extract::<PyIdx>(py)?;

        Ok(idx.slice_d2(py, self))
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
