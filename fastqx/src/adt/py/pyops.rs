//! file: pyops.rs
//! author: Jacob Xie
//! date: 2023/10/09 10:37:20 Monday
//! brief:

use anyhow::anyhow;
use pyo3::prelude::*;
use pyo3::PyObject;

use super::utils::_to_fdx_idx;
use crate::adt::{FqxData, FqxRow};
use crate::ops::*;

// ================================================================================================
// PyMethods
// ================================================================================================

macro_rules! guard {
    ($l:expr) => {
        if !$l.is_callable() {
            return Err(anyhow!("lambda is not a callable").into());
        }
    };
}

#[pymethods]
impl FqxData {
    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // apply

    #[pyo3(name = "apply")]
    fn py_apply(&self, py: Python<'_>, lambda: &PyAny) -> PyResult<Vec<PyObject>> {
        guard!(lambda);

        let res = self.try_apply(|r| {
            let ans = lambda.call1((r.clone(),))?.to_object(py);
            Ok(ans)
        })?;

        Ok(res)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // agg

    #[pyo3(name = "sum")]
    fn py_sum(&self) -> Option<FqxRow> {
        self.sum()
    }

    #[pyo3(name = "min")]
    fn py_min(&self) -> Option<FqxRow> {
        self.min()
    }

    #[pyo3(name = "max")]
    fn py_max(&self) -> Option<FqxRow> {
        self.max()
    }

    #[pyo3(name = "mean")]
    fn py_mean(&self) -> Option<FqxRow> {
        self.mean()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // cum_agg

    #[pyo3(name = "cum_sum")]
    fn py_cum_sum(&self) -> Vec<FqxRow> {
        self.cum_sum()
    }

    #[pyo3(name = "cum_min")]
    fn py_cum_min(&self) -> Vec<FqxRow> {
        self.cum_min()
    }

    #[pyo3(name = "cum_max")]
    fn py_cum_max(&self) -> Vec<FqxRow> {
        self.cum_max()
    }

    #[pyo3(name = "cum_mean")]
    fn py_cum_mean(&self) -> Vec<FqxRow> {
        self.cum_mean()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // filter

    #[pyo3(name = "filter")]
    fn py_filter(&self, lambda: &PyAny) -> PyResult<Self> {
        guard!(lambda);

        let f = |r: &FqxRow| {
            let res = lambda.call1((r.clone(),))?.extract::<bool>()?;
            Ok::<bool, PyErr>(res)
        };
        let res = self.clone().filter(|r| f(r).unwrap_or(false));

        Ok(res)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // reduce

    #[pyo3(name = "reduce")]
    fn py_reduce(&self, lambda: &PyAny) -> PyResult<Option<FqxRow>> {
        guard!(lambda);

        let f = |p: FqxRow, c: FqxRow| {
            let res = lambda.call1((p, c))?.extract::<FqxRow>()?;
            Ok(res)
        };
        let res = (&self).try_reduce(|p, c| f(p, c))?;

        Ok(res)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // select

    #[pyo3(name = "x")]
    fn py_x(&self, py: Python<'_>, idx: PyObject) -> PyResult<Self> {
        let idx = _to_fdx_idx(idx)?;

        todo!()
    }
}
