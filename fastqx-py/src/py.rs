//! file: py.rs
//! author: Jacob Xie
//! date: 2023/09/10 19:49:32 Sunday
//! brief:
//!
//! submodule registration: https://github.com/PyO3/pyo3/issues/1517#issuecomment-808664021

use fastqx::prelude::*;
use fastqx::sources::sql::ConnectorConfig;
use pyo3::prelude::*;

use crate::csv::{fqx_data_from_csv, fqx_data_to_csv};
use crate::http::PyHttpConnector;
use crate::sql::PySqlConnector;
use crate::{new_fqx_data, PyData, PyGroup, PyGroupKey};

// ================================================================================================
// Sql
// ================================================================================================

fn module_sql(py: Python<'_>) -> PyResult<Bound<PyModule>> {
    let m = PyModule::new_bound(py, "fastqx.sql")?;
    m.add_class::<Driver>()?;
    m.add_class::<ConnectorConfig>()?;
    m.add_class::<PySqlConnector>()?;

    Ok(m)
}

// ================================================================================================
// Csv
// ================================================================================================

#[allow(dead_code)]
#[allow(unused_variables)]
fn module_csv(py: Python<'_>) -> PyResult<Bound<PyModule>> {
    let m = PyModule::new_bound(py, "fastqx.csv")?;
    m.add_wrapped(wrap_pyfunction!(fqx_data_from_csv))?;
    m.add_wrapped(wrap_pyfunction!(fqx_data_to_csv))?;

    Ok(m)
}
// ================================================================================================
// Http
// ================================================================================================

#[allow(dead_code)]
#[allow(unused_variables)]
fn module_http(py: Python<'_>) -> PyResult<Bound<PyModule>> {
    let m = PyModule::new_bound(py, "fastqx.http")?;
    m.add_class::<PyHttpConnector>()?;

    Ok(m)
}

// ================================================================================================
// PyModule
// ================================================================================================

#[pymodule]
#[pyo3(name = "fastqx")]
fn py_fastqx(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // main
    m.add_class::<SaveMode>()?;
    m.add_class::<FqxValueType>()?;
    m.add_class::<FqxRow>()?;
    m.add_class::<PyData>()?;
    m.add_class::<PyGroup>()?;
    m.add_class::<PyGroupKey>()?;
    m.add_wrapped(wrap_pyfunction!(new_fqx_data))?;

    // submodule: sql
    let sql = module_sql(py)?;
    pyo3::py_run!(py, sql, "import sys; sys.modules['fastqx.sql'] = sql");
    m.add_submodule(&sql)?;

    // submodule: csv
    let csv = module_csv(py)?;
    pyo3::py_run!(py, csv, "import sys; sys.modules['fastqx.csv'] = csv");
    m.add_submodule(&csv)?;

    // submodule: http
    let http = module_http(py)?;
    pyo3::py_run!(py, http, "import sys; sys.modules['fastqx.http'] = http");
    m.add_submodule(&http)?;

    Ok(())
}
