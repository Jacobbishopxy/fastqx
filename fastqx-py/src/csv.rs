//! file: csv.rs
//! author: Jacob Xie
//! date: 2023/09/14 23:22:25 Thursday
//! brief:

use fastqx::prelude::*;
use pyo3::prelude::*;

use crate::PyData;

#[pyfunction]
pub fn fqx_data_from_csv(path: String, type_hints: Vec<FqxValueType>) -> PyResult<PyData> {
    Ok(PyData::from(csv_read_rd(path, &type_hints)?))
}

#[pyfunction]
pub fn fqx_data_to_csv(data: PyData, path: String) -> PyResult<()> {
    Python::with_gil(|py| Ok(csv_write_rd(&data.inner.borrow(py), path)?))
}
