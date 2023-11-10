//! file: data.rs
//! author: Jacob Xie
//! date: 2023/09/12 23:16:27 Tuesday
//! brief:

use std::collections::HashMap;

use anyhow::anyhow;
use fastqx::prelude::*;
use fastqx::sources::sql::pysql::PySqlConnector;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyType};

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

    fn retain(&mut self, py: Python<'_>, lambda: &PyAny) -> PyResult<()> {
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

    fn set_columns(&mut self, py: Python<'_>, columns: Vec<String>) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).set_columns(columns)?)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // FqxData methods

    fn type_coercion(&mut self, py: Python<'_>) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).type_coercion()?)
    }

    fn cast(&mut self, py: Python<'_>, idx: usize, typ: FqxValueType) -> PyResult<()> {
        Ok(self.inner.borrow_mut(py).cast(idx, &typ)?)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Python methods

    #[classmethod]
    fn from_list(_cls: &PyType, py: Python<'_>, data: Vec<Vec<FqxValue>>) -> PyResult<Self> {
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

        Ok(PyData {
            inner: Py::new(py, d)?,
        })
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
    fn from_records(
        _cls: &PyType,
        py: Python<'_>,
        data: Vec<HashMap<String, FqxValue>>,
    ) -> PyResult<Self> {
        let res = FqxData::from_hashmaps(data)?;

        Ok(PyData {
            inner: Py::new(py, res)?,
        })
    }

    fn to_records(&self, py: Python<'_>) -> PyObject {
        self.inner.borrow(py).to_hashmaps().into_py(py)
    }

    fn to_pandas(&self, py: Python<'_>) -> PyResult<PyObject> {
        let module = PyModule::import(py, "pandas")?;
        let df = module.getattr("DataFrame")?;
        let data = self
            .inner
            .borrow(py)
            .iter()
            .cloned()
            .map(FqxRow::to_values)
            .collect::<Vec<_>>();
        let df = df.call1((data,))?;
        df.setattr("columns", self.inner.borrow(py).columns().clone())?;

        Ok(df.into())
    }

    fn to_str(&self, py: Python<'_>) -> PyResult<String> {
        self.__str__(py)
    }

    fn to_str_pretty(&self, py: Python<'_>) -> PyResult<String> {
        self.__repr__(py)
    }

    fn to_json(&self, py: Python<'_>) -> PyResult<PyObject> {
        let module = PyModule::import(py, "json")?;
        let dumps = module.getattr("dumps")?;
        let data = self.inner.borrow(py).to_hashmaps();
        let json = dumps.call1((data,))?;

        Ok(json.into())
    }

    fn to_json_records(&self, py: Python<'_>) -> PyResult<PyObject> {
        let module = PyModule::import(py, "json")?;
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
    fn from_csv(
        _cls: &PyType,
        py: Python<'_>,
        path: String,
        type_hints: Vec<FqxValueType>,
    ) -> PyResult<Self> {
        let res = csv_read_rd(path, &type_hints)?;

        Ok(PyData {
            inner: Py::new(py, res)?,
        })
    }

    fn to_csv(&self, py: Python<'_>, path: String) -> PyResult<()> {
        Ok(csv_write_rd(&self.inner.borrow(py), path)?)
    }

    #[classmethod]
    fn from_sql(
        _cls: &PyType,
        py: Python<'_>,
        sql: String,
        conn: &PySqlConnector,
    ) -> PyResult<Self> {
        let res = conn.fetch(&sql)?;

        Ok(PyData {
            inner: Py::new(py, res)?,
        })
    }

    fn to_sql(
        &self,
        py: Python<'_>,
        table: String,
        conn: &PySqlConnector,
        mode: SaveMode,
    ) -> PyResult<()> {
        let d = self.inner.borrow(py).clone();
        Ok(conn.save(d, &table, mode)?)
    }

    fn to_dataclasses<'p>(
        &self,
        py: Python<'_>,
        dataclass_type: &'p PyAny,
    ) -> PyResult<Vec<&'p PyAny>> {
        let mut res = vec![];

        for row in self.inner.borrow(py).data().iter() {
            let args = row
                .clone()
                .to_values()
                .into_iter()
                .map(|e| e.into_py(py))
                .collect::<Vec<_>>();
            let py_args = PyTuple::new(py, args);
            let obj = dataclass_type.call(py_args, None)?;

            res.push(obj);
        }

        Ok(res)
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