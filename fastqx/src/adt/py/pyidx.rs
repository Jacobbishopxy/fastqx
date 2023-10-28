//! file: pyidx.rs
//! author: Jacob Xie
//! date: 2023/10/27 23:54:11 Friday
//! brief:

use pyo3::types::PySlice;
use pyo3::FromPyObject;

use super::utils::{slice_1d, slice_2d};
use crate::adt::FqxData;
use crate::ops::{OpOwned, OpSelect};

// ================================================================================================
//
// ================================================================================================

pub(crate) struct IdxSlice<'a>(&'a PySlice);

impl<'a> FromPyObject<'a> for IdxSlice<'a> {
    fn extract(ob: &'a pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(IdxSlice(ob.downcast::<PySlice>()?))
    }
}

#[derive(FromPyObject)]
pub(crate) enum PyIdx<'a> {
    S(usize),
    VS(Vec<usize>),
    VST(Vec<String>),
    PS(IdxSlice<'a>),
    PST((IdxSlice<'a>, IdxSlice<'a>)),
}

impl<'a> PyIdx<'a> {
    pub fn slice_owned(self, d: &FqxData) -> FqxData {
        match self {
            // column-wise
            PyIdx::S(s) => d.select(s).to_owned(),
            PyIdx::VS(vs) => d.select(vs).to_owned(),
            PyIdx::VST(vst) => d.select(vst).to_owned(),
            // row-wise
            PyIdx::PS(ps) => FqxData {
                columns: d.columns.clone(),
                types: d.types.clone(),
                data: slice_1d(&d.data, d.height() as isize, ps.0),
            },
            // row-column
            PyIdx::PST((r, c)) => {
                let (h, w) = d.shape();
                let (h, w) = (h as isize, w as isize);
                let data = slice_2d(&d.data, h, w, r.0, c.0);
                let columns = slice_1d(&d.columns, w, c.0);
                let types = slice_1d(&d.types, w, c.0);
                FqxData {
                    columns,
                    types,
                    data,
                }
            }
        }
    }
}
