//! file: data.rs
//! author: Jacob Xie
//! date: 2023/09/12 23:16:27 Tuesday
//! brief:

use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use fastqx::prelude::*;
use fastqx::serde_json;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyType};

use super::idx::{PyAssign, PyIdx};
use crate::sql::PySqlConnector;

// ================================================================================================
// Cst
// ================================================================================================

#[pyfunction]
#[pyo3(signature = (data, columns=None))]
pub fn new_fqx_data(data: Vec<Vec<FqxValue>>, columns: Option<Vec<String>>) -> PyResult<PyData> {
    let mut d = FqxData::new_by_data(data)?;
    if let Some(c) = columns {
        d.set_columns(c)?;
    }

    Ok(PyData::from(d))
}

// ================================================================================================
// PyData
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxData")]
#[derive(Clone)]
pub struct PyData {
    pub(crate) inner: Py<FqxData>,
}

#[pymethods]
impl PyData {
    #[new]
    fn __new__(
        columns: Vec<String>,
        types: Vec<FqxValueType>,
        data: Vec<FqxRow>,
    ) -> PyResult<PyData> {
        Ok(PyData::from(FqxData::new(columns, types, data)?))
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(self.inner.borrow(py).to_pretty_string()?)
    }

    fn __str__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(self.inner.borrow(py).to_string()?)
    }

    fn __getitem__(&self, py: Python<'_>, idx: PyObject) -> PyResult<PyData> {
        let idx = idx.extract::<PyIdx>(py)?;
        let d = idx.slice_owned(py, &self.inner.borrow(py));

        Ok(PyData::from(d))
    }

    fn __setitem__(&mut self, py: Python<'_>, idx: PyObject, val: PyObject) -> PyResult<()> {
        let idx = idx.extract::<PyIdx>(py)?;
        let val = val.extract::<PyAssign>(py)?;

        Ok(idx.slice_mut(py, &mut self.inner.borrow_mut(py), val)?)
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyIter>> {
        let iter = PyIter {
            inner: self.inner.borrow(py).data().to_vec().into_iter(),
        };

        Py::new(py, iter)
    }

    fn __len__(&self, py: Python<'_>) -> usize {
        self.inner.borrow(py).height()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // getter & setter

    #[getter]
    fn columns(&self, py: Python<'_>) -> Vec<String> {
        self.inner.borrow(py).columns().to_vec()
    }

    #[setter]
    fn set_columns(&mut self, py: Python<'_>, value: Vec<String>) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).set_columns(value)?)
    }

    #[getter]
    fn types(&self, py: Python<'_>) -> Vec<FqxValueType> {
        self.inner.borrow(py).types().to_vec()
    }

    #[setter]
    fn set_types(&mut self, py: Python<'_>, value: Vec<FqxValueType>) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).set_types(value)?)
    }

    #[getter]
    fn data(&self, py: Python<'_>) -> Vec<FqxRow> {
        self.inner.borrow(py).data().to_vec()
    }

    #[setter]
    fn set_data(&mut self, py: Python<'_>, value: Vec<FqxRow>) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).set_data(value)?)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // FqxD methods

    fn height(&self, py: Python<'_>) -> usize {
        self.inner.borrow(py).height()
    }

    fn width(&self, py: Python<'_>) -> usize {
        self.inner.borrow(py).width()
    }

    fn shape(&self, py: Python<'_>) -> (usize, usize) {
        self.inner.borrow(py).shape()
    }

    fn push(&mut self, py: Python<'_>, row: FqxRow) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).push(row)?)
    }

    fn extend(&mut self, py: Python<'_>, rows: Vec<FqxRow>) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).extend(rows)?)
    }

    fn insert(&mut self, py: Python<'_>, idx: usize, row: FqxRow) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).insert(idx, row)?)
    }

    fn pop(&mut self, py: Python<'_>) -> Option<FqxRow> {
        self.inner.borrow_mut(py).pop()
    }

    fn remove(&mut self, py: Python<'_>, idx: usize) -> Option<FqxRow> {
        self.inner.borrow_mut(py).remove(idx)
    }

    fn retain(&mut self, py: Python<'_>, lambda: Bound<PyAny>) -> PyResult<()> {
        let f = |r: &FqxRow| {
            let res = lambda.call1((r.clone(),))?.extract::<bool>()?;
            Ok::<bool, PyErr>(res)
        };

        self.inner.borrow_mut(py).retain(|r| f(r).unwrap_or(true));

        Ok(())
    }

    fn reverse(&mut self, py: Python<'_>) {
        self.inner.borrow_mut(py).reverse()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // FqxData methods

    fn type_coercion(&mut self, py: Python<'_>) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).type_coercion()?)
    }

    fn cast(&mut self, py: Python<'_>, idx: usize, typ: String) -> PyResult<()> {
        let typ = FqxValueType::from_str(&typ)?;
        Ok(self.inner.borrow_mut(py).cast(idx, &typ)?)
    }

    fn empty_row(&self, py: Python<'_>) -> FqxRow {
        self.inner.borrow(py).empty_row()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Python methods

    #[classmethod]
    fn from_list(_cls: Bound<PyType>, data: Vec<Vec<FqxValue>>) -> PyResult<Self> {
        if data.is_empty() {
            return Err(anyhow!("data is empty").into());
        }

        let columns = (0..data.first().unwrap().len())
            .map(|i| format!("col_{i}"))
            .collect::<Vec<_>>();

        let types = data
            .first()
            .unwrap()
            .iter()
            .map(FqxValueType::from)
            .collect::<Vec<_>>();

        let d = FqxData::new(columns, types, data)?;

        Ok(PyData::from(d))
    }

    fn to_list(&self, py: Python<'_>) -> PyObject {
        let res = self
            .inner
            .borrow(py)
            .data()
            .iter()
            .cloned()
            .map(|row| row.into_iter().map(|e| e.into_py(py)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        res.into_py(py)
    }

    #[classmethod]
    fn from_records(_cls: Bound<PyType>, data: Vec<HashMap<String, FqxValue>>) -> PyResult<Self> {
        let res = FqxData::from_hashmaps(data)?;

        Ok(PyData::from(res))
    }

    fn to_records(&self, py: Python<'_>) -> PyObject {
        self.inner.borrow(py).to_hashmaps().into_py(py)
    }

    fn to_dataframe(&self, py: Python<'_>) -> PyResult<PyObject> {
        let module = PyModule::import_bound(py, "pandas")?;
        let df = module.getattr("DataFrame")?;
        let data = self
            .inner
            .borrow(py)
            .iter()
            .cloned()
            .map(FqxRow::to_values)
            .collect::<Vec<_>>();
        let df = df.call1((data,))?;
        df.setattr("columns", self.inner.borrow(py).columns())?;

        Ok(df.into())
    }

    fn to_dataclasses<'p>(
        &self,
        py: Python<'_>,
        dataclass_type: &'p Bound<PyAny>,
    ) -> PyResult<Vec<Bound<'p, PyAny>>> {
        let mut res = vec![];

        for row in self.inner.borrow(py).data().iter() {
            let args = row
                .data()
                .into_iter()
                .cloned()
                .map(|e| e.into_py(py))
                .collect::<Vec<_>>();
            let py_args = PyTuple::new_bound(py, args);
            let obj = dataclass_type.call(py_args, None)?;

            res.push(obj);
        }

        Ok(res)
    }

    fn to_str(&self, py: Python<'_>) -> PyResult<String> {
        self.__str__(py)
    }

    fn to_str_pretty(&self, py: Python<'_>) -> PyResult<String> {
        self.__repr__(py)
    }

    fn to_json(&self, py: Python<'_>) -> PyResult<PyObject> {
        let module = PyModule::import_bound(py, "json")?;
        let dumps = module.getattr("dumps")?;
        let data = self.inner.borrow(py).to_hashmaps();
        let json = dumps.call1((data,))?;

        Ok(json.into())
    }

    fn to_json_records(&self, py: Python<'_>) -> PyResult<PyObject> {
        let module = PyModule::import_bound(py, "json")?;
        let dumps = module.getattr("dumps")?;
        let head = self
            .inner
            .borrow(py)
            .columns()
            .iter()
            .map(|e| FqxValue::from(e.clone()))
            .collect::<Vec<_>>();
        let mut data = vec![head];

        let body = self
            .inner
            .borrow(py)
            .iter()
            .cloned()
            .map(FqxRow::to_values)
            .collect::<Vec<_>>();
        data.extend(body);

        let json = dumps.call1((data,))?;

        Ok(json.into())
    }

    #[classmethod]
    fn from_csv(_cls: &Bound<PyType>, path: String, type_hints: Vec<String>) -> PyResult<Self> {
        let type_hints = type_hints
            .into_iter()
            .map(|s| FqxValueType::from_str(&s))
            .collect::<Result<Vec<_>>>()?;
        let res = csv_read_rd(path, &type_hints)?;

        Ok(PyData::from(res))
    }

    fn to_csv(&self, py: Python<'_>, path: String) -> PyResult<()> {
        Ok(csv_write_rd(&self.inner.borrow(py), path)?)
    }

    #[classmethod]
    fn from_sql(_cls: &Bound<PyType>, sql: String, conn: &PySqlConnector) -> PyResult<Self> {
        let res = conn.fetch(&sql)?;

        Ok(PyData::from(res))
    }

    fn to_sql(
        &self,
        py: Python<'_>,
        table: String,
        conn: &PySqlConnector,
        mode: SaveMode,
    ) -> PyResult<()> {
        Ok(conn.save(py, self, &table, mode)?)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // x

    #[getter]
    fn x(&self, py: Python<'_>) -> PyX {
        PyX(self.inner.clone_ref(py))
    }
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

// ================================================================================================
// PyX
// ================================================================================================

#[pyclass]
pub struct PyX(Py<FqxData>);

#[pymethods]
impl PyX {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(serde_json::to_string_pretty(self.0.borrow(py).data()).map_err(anyhow::Error::msg)?)
    }

    fn __str__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(serde_json::to_string(self.0.borrow(py).data()).map_err(anyhow::Error::msg)?)
    }

    fn __getitem__(&self, py: Python<'_>, idx: PyObject) -> PyResult<Vec<Vec<FqxValue>>> {
        let idx = idx.extract::<PyIdx>(py)?;

        Ok(idx.slice_d2(py, &self.0.borrow(py)))
    }

    fn __setitem__(&mut self, py: Python<'_>, idx: PyObject, val: PyObject) -> PyResult<()> {
        let idx = idx.extract::<PyIdx>(py)?;
        let val = val.extract::<PyAssign>(py)?;

        Ok(idx.slice_mut(py, &mut self.0.borrow_mut(py), val)?)
    }
}

// ================================================================================================
// From
// ================================================================================================

impl From<FqxData> for PyData {
    fn from(value: FqxData) -> Self {
        Python::with_gil(|py| PyData {
            inner: Py::new(py, value).expect("Python GIL failure"),
        })
    }
}
