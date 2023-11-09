//! file: data.rs
//! author: Jacob Xie
//! date: 2023/09/12 23:16:27 Tuesday
//! brief:

use fastqx::prelude::*;
use pyo3::prelude::*;

use super::idx::{PyAssign, PyIdx};

#[pyfunction]
#[pyo3(signature = (data, columns=None))]
pub fn new_fqx_data(data: Vec<Vec<FqxValue>>, columns: Option<Vec<String>>) -> PyResult<FqxData> {
    let mut d = FqxData::new_by_data(data)?;
    if let Some(c) = columns {
        d.set_columns(c)?;
    }

    Ok(d)
}

// ================================================================================================
// PyData
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxData")]
pub struct PyData {
    inner: Py<FqxData>,
}

#[pymethods]
impl PyData {
    #[new]
    fn __new__(py: Python<'_>) -> PyResult<PyData> {
        Ok(PyData {
            inner: Py::new(py, FqxData::default())?,
        })
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(self.inner.borrow(py).to_pretty_string()?)
    }

    fn __str__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(self.inner.borrow(py).to_string()?)
    }

    fn __getitem__(&self, py: Python<'_>, idx: PyObject) -> PyResult<FqxData> {
        let idx = idx.extract::<PyIdx>(py)?;

        Ok(idx.slice_owned(py, &self.inner.borrow(py)))
    }

    fn __setitem__(&mut self, py: Python<'_>, idx: PyObject, val: PyObject) -> PyResult<()> {
        let idx = idx.extract::<PyIdx>(py)?;
        let val = val.extract::<PyAssign>(py)?;

        Ok(idx.slice_mut(py, &mut self.inner.borrow_mut(py), val)?)
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyIter>> {
        let iter = PyIter {
            inner: self.inner.borrow(py).data().clone().into_iter(),
        };

        Py::new(py, iter)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    #[pyo3(name = "shape")]
    fn shape(&self, py: Python<'_>) -> (usize, usize) {
        self.inner.borrow(py).shape()
    }

    #[pyo3(name = "type_coercion")]
    fn type_coercion(&mut self, py: Python<'_>) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).type_coercion()?)
    }

    #[pyo3(name = "cast")]
    fn cast(&mut self, py: Python<'_>, idx: usize, typ: &FqxValueType) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).cast(idx, typ)?)
    }

    // TODO
}

// ================================================================================================
// PyIter
// ================================================================================================

#[pyclass]
pub struct PyIter {
    inner: std::vec::IntoIter<FqxRow>,
}

#[pymethods]
impl PyIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<FqxRow> {
        slf.inner.next()
    }
}
