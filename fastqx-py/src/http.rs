//! file: http.rs
//! author: Jacob Xie
//! date: 2023/10/02 19:01:05 Monday
//! brief:

use fastqx::prelude::*;
use fastqx::serde_json::Value;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use tokio::runtime::Runtime;

use crate::PyData;

// ================================================================================================
// Classes
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxHttpConnector", subclass)]
pub struct PyHttpConnector {
    inner: HttpConnector,
    runtime: Runtime,
}

#[pymethods]
impl PyHttpConnector {
    #[new]
    fn new(url: &str, auth: Option<&str>) -> PyResult<Self> {
        let runtime = Runtime::new()?;

        let inner = HttpConnector::new(url, auth)?;

        Ok(PyHttpConnector { inner, runtime })
    }

    fn url(&self) -> &str {
        self.inner.url()
    }

    fn get(self_: PyRef<Self>, py: Python<'_>, subpath: &str) -> PyResult<PyObject> {
        let res = self_.runtime.block_on(async {
            let json = self_.inner.dyn_get(subpath).await?;

            Ok::<_, anyhow::Error>(pythonize(py, &json)?)
        })?;

        Ok(res)
    }

    fn post(self_: PyRef<Self>, py: Python<'_>, subpath: &str, req: &PyAny) -> PyResult<PyObject> {
        let req = depythonize(req)?;
        let res = self_.runtime.block_on(async {
            let json = self_.inner.dyn_post(subpath, &req).await?;

            Ok::<_, anyhow::Error>(pythonize(py, &json)?)
        })?;

        Ok(res)
    }

    fn put(self_: PyRef<Self>, py: Python<'_>, subpath: &str, req: &PyAny) -> PyResult<PyObject> {
        let req = depythonize(req)?;
        let res = self_.runtime.block_on(async {
            let json = self_.inner.dyn_put(subpath, &req).await?;

            Ok::<_, anyhow::Error>(pythonize(py, &json)?)
        })?;

        Ok(res)
    }

    fn delete(self_: PyRef<Self>, py: Python<'_>, subpath: &str) -> PyResult<PyObject> {
        let res = self_.runtime.block_on(async {
            let json = self_.inner.dyn_delete(subpath).await?;

            Ok::<_, anyhow::Error>(pythonize(py, &json)?)
        })?;

        Ok(res)
    }

    fn patch(self_: PyRef<Self>, py: Python<'_>, subpath: &str, req: &PyAny) -> PyResult<PyObject> {
        let req = depythonize(req)?;
        let res = self_.runtime.block_on(async {
            let json = self_.inner.dyn_patch(subpath, &req).await?;

            Ok::<_, anyhow::Error>(pythonize(py, &json)?)
        })?;

        Ok(res)
    }

    fn fetch(
        self_: PyRef<Self>,
        subpath: &str,
        method: &HttpMethod,
        payload: Option<&PyAny>,
    ) -> PyResult<PyData> {
        let payload = payload.and_then(|p| depythonize::<Value>(p).ok());
        let data = self_.runtime.block_on(async {
            let res = FqxData::curl(&self_.inner, subpath, method, payload).await?;

            Ok::<_, anyhow::Error>(res)
        })?;

        Ok(PyData::from(data))
    }
}
