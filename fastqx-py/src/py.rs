//! file: py.rs
//! author: Jacob Xie
//! date: 2023/09/10 19:49:32 Sunday
//! brief:

use fastqx::prelude::*;
use pyo3::prelude::*;

use crate::csv::{fqx_data_from_csv, fqx_data_to_csv};
use crate::data::new_fqx_data;
use crate::sql::PyConnector;

// ================================================================================================
// Sql
// ================================================================================================

fn module_sql(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "fastqx.sql")?;
    m.add_class::<Driver>()?;
    m.add_class::<ConnectorConfig>()?;
    m.add_class::<PyConnector>()?;

    Ok(m)
}

// ================================================================================================
// Csv
// ================================================================================================

#[allow(dead_code)]
#[allow(unused_variables)]
fn module_csv(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "fastqx.csv")?;
    m.add_wrapped(wrap_pyfunction!(fqx_data_from_csv))?;
    m.add_wrapped(wrap_pyfunction!(fqx_data_to_csv))?;

    Ok(m)
}
// ================================================================================================
// Http
// ================================================================================================

#[allow(dead_code)]
#[allow(unused_variables)]
fn module_http(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "fastqx.http")?;

    Ok(m)
}

// ================================================================================================
// PyModule
// ================================================================================================

#[pymodule]
#[pyo3(name = "fastqx")]
fn py_fastqx(py: Python, m: &PyModule) -> PyResult<()> {
    // main
    m.add_class::<SaveMode>()?;
    m.add_class::<FqxValueType>()?;
    m.add_class::<FqxRow>()?;
    m.add_class::<FqxData>()?;
    m.add_wrapped(wrap_pyfunction!(new_fqx_data))?;

    // submodule: sql
    let sql = module_sql(py)?;
    pyo3::py_run!(py, sql, "import sys; sys.modules['fastqx.sql'] = sql");
    m.add_submodule(sql)?;

    // submodule: csv
    let csv = module_csv(py)?;
    pyo3::py_run!(py, csv, "import sys; sys.modules['fastqx.csv'] = csv");
    m.add_submodule(csv)?;

    Ok(())
}
