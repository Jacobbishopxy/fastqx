//! file: helper.rs
//! author: Jacob Xie
//! date: 2023/09/10 19:49:15 Sunday
//! brief:

use anyhow::Result;
use pyo3::exceptions;
use pyo3::prelude::*;

pub(crate) fn convert_result<T>(result: Result<T>) -> PyResult<T> {
    match result {
        Ok(v) => Ok(v),
        Err(err) => Err(exceptions::PyException::new_err(format!("{:#}", err))),
    }
}
