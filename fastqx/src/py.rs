//! file: py.rs
//! author: Jacob Xie
//! date: 2023/09/10 19:49:32 Sunday
//! brief:

use std::sync::Arc;

use pyo3::prelude::*;

use fastqx_core::conn::db::Connector;

use crate::helper::convert_result;

// ================================================================================================
// PyModule
// ================================================================================================

#[pymodule]
fn fastqx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyConnector>()?;

    Ok(())
}

// ================================================================================================
// Classes & Functions exported to Py
// ================================================================================================

#[pyclass]
struct PyConnector {
    inner: Arc<Connector>,
}

#[pymethods]
impl PyConnector {
    #[new]
    fn new(conn_str: &str) -> PyResult<Self> {
        let connector = convert_result(Connector::new(conn_str))?;
        Ok(PyConnector {
            inner: Arc::new(connector),
        })
    }

    fn close<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let conn = Arc::clone(&self.inner);

        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert_result(conn.close().await)?;

            Ok(())
        })
    }

    fn execute<'a>(&self, py: Python<'a>, sql: String) -> PyResult<&'a PyAny> {
        let conn = Arc::clone(&self.inner);

        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert_result(conn.execute(&sql).await)?;

            Ok(())
        })
    }

    // TODO
}
