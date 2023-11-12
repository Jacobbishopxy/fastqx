//! file: sql.rs
//! author: Jacob Xie
//! date: 2023/11/12 12:39:49 Sunday
//! brief:

use fastqx::prelude::*;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

use crate::PyData;

// ================================================================================================
// Classes & Functions exported to Py
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxSqlConnector", subclass)]
pub struct PySqlConnector {
    inner: SqlConnector,
    runtime: Runtime,
}

#[pymethods]
impl PySqlConnector {
    #[new]
    fn new(conn_str: &str) -> PyResult<Self> {
        let runtime = Runtime::new()?;

        let inner = runtime
            .block_on(async { Ok::<_, anyhow::Error>(SqlConnector::new(conn_str).await?) })?;

        Ok(PySqlConnector { inner, runtime })
    }

    fn conn_str(&self) -> &str {
        &self.inner.conn_str()
    }

    fn close(self_: PyRef<Self>) -> PyResult<()> {
        let res = self_.runtime.block_on(async {
            self_.inner.close().await?;

            Ok::<_, anyhow::Error>(())
        });
        res?;

        Ok(())
    }

    fn is_close(&self) -> bool {
        self.inner.is_close()
    }

    fn execute(&self, sql: &str) -> PyResult<()> {
        let conn = self.inner.clone();

        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async move {
            conn.execute(sql).await?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }

    pub fn fetch(&self, sql: &str) -> PyResult<FqxData> {
        let res = self.runtime.block_on(async {
            let d = self.inner.dyn_fetch(sql).await?;

            Ok::<_, anyhow::Error>(d)
        })?;

        Ok(res)
    }

    pub fn save(
        &self,
        py: Python<'_>,
        data: &PyData,
        table_name: &str,
        mode: SaveMode,
    ) -> PyResult<()> {
        let conn = self.inner.clone();

        self.runtime.block_on(async move {
            conn.dyn_save(data.inner.borrow(py).clone(), table_name, mode, true)
                .await?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }

    fn uncheck_save(
        self_: PyRef<'_, Self>,
        py: Python<'_>,
        data: &PyData,
        table_name: &str,
        mode: SaveMode,
    ) -> PyResult<()> {
        let conn = self_.inner.clone();

        self_.runtime.block_on(async move {
            conn.dyn_save(data.inner.borrow(py).clone(), table_name, mode, false)
                .await?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }
}
