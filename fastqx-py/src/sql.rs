//! file: sql.rs
//! author: Jacob Xie
//! date: 2023/09/14 23:15:47 Thursday
//! brief:

use anyhow::anyhow;
use fastqx::prelude::*;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

// ================================================================================================
// ConnectorType
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxConnectorType")]
#[derive(Debug)]
pub enum ConnectorType {
    MySql,
    Postgres,
    MsSql,
    Sqlite,
}

#[pymethods]
impl ConnectorType {
    pub fn __repr__(&self) -> &'static str {
        match self {
            ConnectorType::MySql => "FqxConnectorType::MySql",
            ConnectorType::Postgres => "FqxConnectorType::Postgres",
            ConnectorType::MsSql => "FqxConnectorType::MsSql",
            ConnectorType::Sqlite => "FqxConnectorType::Sqlite",
        }
    }
}

// ================================================================================================
// Classes & Functions exported to Py
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxConnector", subclass)]
pub struct PyConnector {
    conn_str: String,
    inner: Option<Connector>,
    runtime: Runtime,
}

macro_rules! guard {
    ($s:expr) => {
        if $s.inner.is_none() {
            return Err(anyhow!("connection is not opened").into());
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
            runtime: Runtime::new()?,
        })
    }

    fn open(mut self_: PyRefMut<Self>) -> PyResult<()> {
        if self_.inner.is_some() {
            return Ok(());
        }

        let conn_str = self_.conn_str.clone();

        let res = self_
            .runtime
            .block_on(async { Ok::<_, anyhow::Error>(Connector::new(conn_str)?) })?;

        self_.inner = Some(res);

        Ok(())
    }

    #[pyo3(text_signature = "($self)")]
    fn close(self_: PyRef<Self>) -> PyResult<()> {
        if self_.inner.is_none() {
            return Ok(());
        }

        let res = self_.runtime.block_on(async {
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
    fn async_execute<'a>(&self, py: Python<'a>, sql: String) -> PyResult<&'a PyAny> {
        guard!(self);

        let conn = self.inner.clone().unwrap();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            conn.execute(&sql).await?;

            Ok(())
        })
    }

    // Python asyncio
    fn async_fetch<'a>(&self, py: Python<'a>, sql: String) -> PyResult<&'a PyAny> {
        guard!(self);

        let conn = self.inner.clone().unwrap();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = conn.dyn_fetch(&sql).await?;

            Ok(res)
        })
    }

    fn execute(&self, sql: &str) -> PyResult<()> {
        guard!(self);

        let conn = self.inner.clone().unwrap();

        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async move {
            conn.execute(sql).await?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }

    #[pyo3(text_signature = "($self, sql)")]
    fn fetch(self_: PyRef<'_, Self>, sql: &str) -> PyResult<FqxData> {
        guard!(self_);

        let res = self_.runtime.block_on(async {
            let d = self_.inner.as_ref().unwrap().dyn_fetch(sql).await?;

            Ok::<_, anyhow::Error>(d)
        })?;

        Ok(res)
    }

    #[pyo3(text_signature = "($self, data, table_name, mode)")]
    fn save(self_: PyRef<'_, Self>, data: FqxData, table_name: &str, mode: &str) -> PyResult<()> {
        guard!(self_);

        let mode = match mode {
            "override" => SaveMode::Override,
            "append" => SaveMode::Append,
            _ => return Err(anyhow!("mode: override/append").into()),
        };

        let conn = self_.inner.clone().unwrap();

        self_.runtime.block_on(async move {
            conn.dyn_save(data, table_name, mode, true).await?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }

    #[pyo3(text_signature = "($self, data, table_name, mode)")]
    fn uncheck_save(
        self_: PyRef<'_, Self>,
        data: FqxData,
        table_name: &str,
        mode: &str,
    ) -> PyResult<()> {
        guard!(self_);

        let mode = match mode {
            "override" => SaveMode::Override,
            "append" => SaveMode::Append,
            _ => return Err(anyhow!("mode: override/append").into()),
        };

        let conn = self_.inner.clone().unwrap();

        self_.runtime.block_on(async move {
            conn.dyn_save(data, table_name, mode, false).await?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }
}
