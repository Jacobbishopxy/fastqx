//! file: py.rs
//! author: Jacob Xie
//! date: 2023/09/10 19:49:32 Sunday
//! brief:

use fastqx_core::prelude::*;
use pyo3::exceptions;
use pyo3::prelude::*;

use crate::data::new_fqx_data;

// ================================================================================================
// PyModule
// ================================================================================================

#[pymodule]
fn fastqx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyConnector>()?;
    m.add_class::<RoughData>()?;
    m.add_wrapped(wrap_pyfunction!(new_fqx_data))?;

    Ok(())
}

// ================================================================================================
// Classes & Functions exported to Py
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxConnector", subclass)]
struct PyConnector {
    conn_str: String,
    inner: Option<Connector>,
}

macro_rules! guard {
    ($s:expr) => {
        if $s.inner.is_none() {
            return Err(exceptions::PyException::new_err("connection is not opened"));
        }
    };
}

#[pymethods]
impl PyConnector {
    #[new]
    fn new(conn_str: &str) -> PyResult<Self> {
        Ok(PyConnector {
            conn_str: conn_str.to_owned(),
            inner: None,
        })
    }

    fn open(mut self_: PyRefMut<Self>) -> PyResult<()> {
        if self_.inner.is_some() {
            return Ok(());
        }

        let conn_str = self_.conn_str.clone();

        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async move {
            self_.inner = Some(Connector::new(conn_str).unwrap());
        });

        Ok(())
    }

    fn close(self_: PyRef<Self>) -> PyResult<()> {
        if self_.inner.is_none() {
            return Ok(());
        }

        let rt = tokio::runtime::Runtime::new()?;
        let res = rt.block_on(async move {
            self_.inner.as_ref().unwrap().close().await?;

            Ok::<_, anyhow::Error>(())
        });
        res?;

        Ok(())
    }

    fn is_close(&self) -> bool {
        self.inner.as_ref().map(|c| c.is_close()).unwrap_or(true)
    }

    // Python asyncio
    fn execute<'a>(&self, py: Python<'a>, sql: String) -> PyResult<&'a PyAny> {
        guard!(self);

        let conn = self.inner.clone().unwrap();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            conn.execute(&sql).await?;

            Ok(())
        })
    }

    // Python asyncio
    fn fetch<'a>(&self, py: Python<'a>, sql: String) -> PyResult<&'a PyAny> {
        guard!(self);

        let conn = self.inner.clone().unwrap();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = conn.dyn_fetch(&sql).await?;

            Ok(res)
        })
    }
}
