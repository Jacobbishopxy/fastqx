//! file: pyops.rs
//! author: Jacob Xie
//! date: 2023/10/09 10:37:20 Monday
//! brief:

use anyhow::anyhow;
use pyo3::prelude::*;

use crate::adt::FqxData;
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

        let res =
            self.try_apply::<Vec<_>, _, _>(|r| Ok(lambda.call1((r.clone(),))?.to_object(py)))?;

        Ok(res)
    }
}
