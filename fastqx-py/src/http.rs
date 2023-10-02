//! file: http.rs
//! author: Jacob Xie
//! date: 2023/10/02 19:01:05 Monday
//! brief:

use fastqx::prelude::*;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

// ================================================================================================
// Classes
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxHttpConnector", subclass)]
pub struct PyConnector {
    inner: HttpConnector,
    runtime: Runtime,
}

#[pymethods]
impl PyConnector {
    #[new]
    fn new(url: &str, auth: Option<&str>) -> PyResult<Self> {
        let runtime = Runtime::new()?;

        let inner = HttpConnector::new(url, auth)?;

        Ok(PyConnector { inner, runtime })
    }

    fn url(&self) -> &str {
        self.inner.url()
    }

    fn get(self_: PyRef<Self>, subpath: &str) -> PyResult<JSON> {
        let res = self_.runtime.block_on(async {
            let json = self_.inner.dyn_get(subpath).await?;

            Ok::<_, anyhow::Error>(json)
        })?;

        Ok(res)
    }

    fn post(self_: PyRef<Self>, subpath: &str, req: &JSON) -> PyResult<JSON> {
        let res = self_.runtime.block_on(async {
            let json = self_.inner.dyn_post(subpath, req).await?;

            Ok::<_, anyhow::Error>(json)
        })?;

        Ok(res)
    }
}
