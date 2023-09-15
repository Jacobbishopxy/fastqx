//! file: csv.rs
//! author: Jacob Xie
//! date: 2023/09/14 23:22:25 Thursday
//! brief:

use fastqx::prelude::*;
use pyo3::prelude::*;

#[pyfunction]
pub fn fqx_data_from_csv(path: String, type_hints: Vec<String>) -> PyResult<RoughData> {
    Ok(rough_data_from_csv_(path, type_hints)?)
}

#[pyfunction]
pub fn fqx_data_to_csv(data: RoughData, path: String) -> PyResult<()> {
    Ok(csv_write_rd(&data, path)?)
}
