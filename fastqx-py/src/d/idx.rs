//! file: idx.rs
//! author: Jacob Xie
//! date: 2023/10/27 23:54:11 Friday
//! brief:

use anyhow::{bail, Result};
use pyo3::prelude::*;
use pyo3::types::PySlice;

use super::utils::*;
use fastqx::adt::{FqxD, FqxData, FqxRow, FqxValue, RowProps};

// ================================================================================================
// PyIdx & PyIdxD
// ================================================================================================

#[derive(Debug, FromPyObject)]
pub(crate) enum PyIdx<'a> {
    R(isize),                                      // a single row
    RS(Bound<'a, PySlice>),                        // row slice
    V((isize, isize)),                             // a single value
    RSS((Bound<'a, PySlice>, Bound<'a, PySlice>)), // row-col slice
    RIS((isize, Bound<'a, PySlice>)),              // a single row, slice of value
    RSI((Bound<'a, PySlice>, isize)),              // a slice of row, single value
}

// IMPORTANT: The order of the variants effects deserialization!
#[derive(Debug, FromPyObject)]
pub(crate) enum PyAssign {
    R(FqxRow),
    RS(Vec<FqxRow>),
    D2(Vec<Vec<FqxValue>>),
    D1(Vec<FqxValue>),
    S(FqxValue),
}

impl<'a> PyIdx<'a> {
    pub fn slice_owned(self, py: Python<'_>, d: &FqxData) -> FqxData {
        match self {
            PyIdx::R(r) => FqxData::new_uncheck(
                d.columns().to_vec(),
                d.types().to_vec(),
                slice_vec(d.data(), d.height() as isize, _isize2slice(r, py)),
            ),
            PyIdx::RS(rs) => FqxData::new_uncheck(
                d.columns().to_vec(),
                d.types().to_vec(),
                slice_vec(d.data(), d.height() as isize, rs),
            ),
            PyIdx::V((r, c)) => slice_fqx(d, _isize2slice(r, py), _isize2slice(c, py)),
            PyIdx::RSS((r, c)) => slice_fqx(d, r, c),
            PyIdx::RIS((r, c)) => slice_fqx(d, _isize2slice(r, py), c),
            PyIdx::RSI((r, c)) => slice_fqx(d, r, _isize2slice(c, py)),
        }
    }

    pub fn slice_d2(self, py: Python<'_>, d: &FqxData) -> Vec<Vec<FqxValue>> {
        match self {
            PyIdx::R(r) => slice_data_to_value(d.data(), _isize2slice(r, py), _full_slice(py)),
            PyIdx::RS(rs) => slice_data_to_value(d.data(), rs, _full_slice(py)),
            PyIdx::V((r, c)) => {
                slice_data_to_value(d.data(), _isize2slice(r, py), _isize2slice(c, py))
            }
            PyIdx::RSS((r, c)) => slice_data_to_value(d.data(), r, c),
            PyIdx::RIS((r, c)) => slice_data_to_value(d.data(), _isize2slice(r, py), c),
            PyIdx::RSI((r, c)) => slice_data_to_value(d.data(), r, _isize2slice(c, py)),
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
            (PyIdx::R(r), PyAssign::R(row)) => {
                let row_slice = _isize2slice(r, py);
                let col_slice = _full_slice(py);
                let val = vec![row.to_values()];
                (row_slice, col_slice, val)
            }
            (PyIdx::RS(rs), PyAssign::RS(rows)) => {
                let row_slice = rs;
                let col_slice = _full_slice(py);
                let val = rows.into_iter().map(FqxRow::to_values).collect();
                (row_slice, col_slice, val)
            }
            (PyIdx::RS(rs), PyAssign::D2(d2)) => {
                let row_slice = rs;
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
            (PyIdx::RSS((rs, cs)), PyAssign::RS(rows)) => {
                let row_slice = rs;
                let col_slice = cs;
                let val = rows.into_iter().map(FqxRow::to_values).collect();
                (row_slice, col_slice, val)
            }
            (PyIdx::RSS((rs, cs)), PyAssign::D2(d2)) => {
                let row_slice = rs;
                let col_slice = cs;
                let val = d2;
                (row_slice, col_slice, val)
            }
            (PyIdx::RIS((r, cs)), PyAssign::D1(d1)) => {
                let row_slice = _isize2slice(r, py);
                let col_slice = cs;
                let val = vec![d1];
                (row_slice, col_slice, val)
            }
            (PyIdx::RSI((rs, c)), PyAssign::D1(d1)) => {
                let row_slice = rs;
                let col_slice = _isize2slice(c, py);
                let val = d1.into_iter().map(|e| vec![e]).collect();
                (row_slice, col_slice, val)
            }
            (i, a) => {
                bail!("mismatch assignment, idx: {:?}, asn: {:?}", i, a);
            }
        };

        slice_fqx_mut(d, row_slice, col_slice, val)?;

        Ok(())
    }
}

// ================================================================================================
// Helpers
// ================================================================================================

fn _full_slice(py: Python<'_>) -> Bound<PySlice> {
    PySlice::full_bound(py)
}

fn _isize2slice(i: isize, py: Python<'_>) -> Bound<PySlice> {
    PySlice::new_bound(py, i, i + 1, 1)
}
