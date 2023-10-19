//! file: pysql.rs
//! author: Jacob Xie
//! date: 2023/10/19 09:46:18 Thursday
//! brief:

use pyo3::prelude::*;
use tokio::runtime::Runtime;

use crate::adt::FqxData;
use crate::sources::sql::SqlConnector;
use crate::sources::SaveMode;

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

    #[pyo3(text_signature = "($self)")]
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

    // Python asyncio
    fn async_execute<'a>(&self, py: Python<'a>, sql: String) -> PyResult<&'a PyAny> {
        let conn = self.inner.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            conn.execute(&sql).await?;

            Ok(())
        })
    }

    // Python asyncio
    fn async_fetch<'a>(&self, py: Python<'a>, sql: String) -> PyResult<&'a PyAny> {
        let conn = self.inner.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = conn.dyn_fetch(&sql).await?;

            Ok(res)
        })
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

    // #[pyo3(text_signature = "($self, sql)")]
    // fn fetch(self_: PyRef<'_, Self>, sql: &str) -> PyResult<FqxData> {
    //     let res = self_.runtime.block_on(async {
    //         let d = self_.inner.dyn_fetch(sql).await?;

    //         Ok::<_, anyhow::Error>(d)
    //     })?;

    //     Ok(res)
    // }

    #[pyo3(text_signature = "($self, sql)")]
    pub fn fetch(&self, sql: &str) -> PyResult<FqxData> {
        let res = self.runtime.block_on(async {
            let d = self.inner.dyn_fetch(sql).await?;

            Ok::<_, anyhow::Error>(d)
        })?;

        Ok(res)
    }

    // #[pyo3(text_signature = "($self, data, table_name, mode)")]
    // fn save(
    //     self_: PyRef<'_, Self>,
    //     data: FqxData,
    //     table_name: &str,
    //     mode: SaveMode,
    // ) -> PyResult<()> {
    //     let conn = self_.inner.clone();

    //     self_.runtime.block_on(async move {
    //         conn.dyn_save(data, table_name, mode, true).await?;

    //         Ok::<_, anyhow::Error>(())
    //     })?;

    //     Ok(())
    // }

    #[pyo3(text_signature = "($self, data, table_name, mode)")]
    pub fn save(&self, data: FqxData, table_name: &str, mode: SaveMode) -> PyResult<()> {
        let conn = self.inner.clone();

        self.runtime.block_on(async move {
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
        mode: SaveMode,
    ) -> PyResult<()> {
        let conn = self_.inner.clone();

        self_.runtime.block_on(async move {
            conn.dyn_save(data, table_name, mode, false).await?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }
}
