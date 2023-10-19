//! file: data.rs
//! author: Jacob Xie
//! date: 2023/09/12 23:16:27 Tuesday
//! brief:

use fastqx::prelude::*;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (data, columns=None))]
pub fn new_fqx_data(data: Vec<Vec<FqxValue>>, columns: Option<Vec<String>>) -> PyResult<FqxData> {
    let mut d = FqxData::new_by_data(data)?;
    if let Some(c) = columns {
        d.set_columns(c)?;
    }

    Ok(d)
}
