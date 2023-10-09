//! file: pyops.rs
//! author: Jacob Xie
//! date: 2023/10/09 10:37:20 Monday
//! brief:

use anyhow::anyhow;
use pyo3::prelude::*;

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
    #[pyo3(name = "apply")]
    fn py_apply(&self, py: Python<'_>, lambda: &PyAny) -> PyResult<Vec<PyObject>> {
        guard!(lambda);

        let res = self.try_apply::<Vec<_>, _, _>(|r| {
            let ans = lambda.call1((r.clone(),))?.to_object(py);
            Ok(ans)
        })?;

        Ok(res)
    }

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
}
