//! file: data.rs
//! author: Jacob Xie
//! date: 2023/09/12 23:16:27 Tuesday
//! brief:

use anyhow::anyhow;
use fastqx::prelude::*;
use pyo3::prelude::*;

#[pyfunction]
pub fn new_fqx_data(columns: Vec<String>, data: Vec<Vec<FqxValue>>) -> PyResult<FqxData> {
    if data.is_empty() {
        return Err(anyhow!("data is empty").into());
    }

    let types = data
        .first()
        .unwrap()
        .iter()
        .map(FqxValueType::from)
        .collect::<Vec<_>>();

    Ok(FqxData::new(columns, types, data)?)
}
