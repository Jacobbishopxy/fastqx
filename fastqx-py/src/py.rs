//! file: py.rs
//! author: Jacob Xie
//! date: 2023/09/10 19:49:32 Sunday
//! brief:

use pyo3::prelude::*;

use crate::csv::{fqx_data_from_csv, fqx_data_to_csv};
use crate::data::new_fqx_data;
use crate::sql::PyConnector;
use crate::{ConnectorConfig, Driver, FqxData, FqxRow, FqxValueType, SaveMode};

// ================================================================================================
// Sql
// ================================================================================================

#[allow(dead_code)]
#[allow(unused_variables)]
fn register_module_sql(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "sql")?;
    // child_module.add_class()

    Ok(())
}

// ================================================================================================
// Csv
// ================================================================================================

#[allow(dead_code)]
#[allow(unused_variables)]
fn register_module_csv(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "csv")?;
    // child_module.add_class()

    Ok(())
}
// ================================================================================================
// Http
// ================================================================================================

#[allow(dead_code)]
#[allow(unused_variables)]
fn register_module_http(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "http")?;
    // child_module.add_class()

    Ok(())
}

// ================================================================================================
// PyModule
// ================================================================================================

#[pymodule]
fn fastqx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SaveMode>()?;
    m.add_class::<Driver>()?;
    m.add_class::<ConnectorConfig>()?;
    m.add_class::<PyConnector>()?;
    m.add_class::<FqxValueType>()?;
    m.add_class::<FqxRow>()?;
    m.add_class::<FqxData>()?;
    m.add_wrapped(wrap_pyfunction!(new_fqx_data))?;
    m.add_wrapped(wrap_pyfunction!(fqx_data_from_csv))?;
    m.add_wrapped(wrap_pyfunction!(fqx_data_to_csv))?;

    Ok(())
}
