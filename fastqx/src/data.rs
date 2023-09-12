//! file: data.rs
//! author: Jacob Xie
//! date: 2023/09/12 23:16:27 Tuesday
//! brief:

use fastqx_core::prelude::*;
use pyo3::exceptions;
use pyo3::prelude::*;

#[pyfunction]
pub fn new_fqx_data(columns: Vec<String>, data: Vec<Vec<RoughValue>>) -> PyResult<RoughData> {
    if data.is_empty() {
        return Err(exceptions::PyException::new_err("data is empty"));
    }

    let c_l = columns.len();

    let types = data
        .first()
        .unwrap()
        .iter()
        .map(RoughValueType::from)
        .collect::<Vec<_>>();

    for (idx, row) in data.iter().enumerate() {
        let r_l = row.len();
        if c_l != r_l {
            return Err(exceptions::PyException::new_err(format!(
                "columns len: {c_l}, row[{idx}] len: {r_l}"
            )));
        }
    }

    Ok(RoughData {
        columns,
        types,
        data,
    })
}
