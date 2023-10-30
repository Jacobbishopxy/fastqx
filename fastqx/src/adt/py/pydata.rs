//! file: pydata.rs
//! author: Jacob Xie
//! date: 2023/10/02 23:16:01 Monday
//! brief:

use std::collections::HashMap;

use anyhow::anyhow;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyType};

use super::pyidx::{PyAssign, PyIdx, PyX};
use crate::adt::ab::iter::FqxII;
use crate::adt::{FqxData, FqxValue, FqxValueType};
use crate::sources::adt::SaveMode;
use crate::sources::csv::*;
use crate::sources::sql::pysql::PySqlConnector;

// ================================================================================================
// PyMethods
// ================================================================================================

#[pymethods]
impl FqxData {
    #[new]
    fn py_new(
        columns: Vec<String>,
        types: Vec<FqxValueType>,
        data: Vec<Vec<FqxValue>>,
    ) -> PyResult<Self> {
        Ok(FqxData::new(columns, types, data)?)
    }

    #[pyo3(name = "shape", text_signature = "($self)")]
    fn py_shape(&self) -> (usize, usize) {
        self.shape()
    }

    #[pyo3(name = "type_coercion")]
    fn py_type_coercion(&mut self) -> PyResult<()> {
        Ok(self.type_coercion()?)
    }

    #[pyo3(name = "cast")]
    fn py_cast(&mut self, idx: usize, typ: &FqxValueType) -> PyResult<()> {
        Ok(self.cast(idx, typ)?)
    }

    #[pyo3(name = "set_columns")]
    fn py_set_columns(&mut self, columns: Vec<String>) -> PyResult<()> {
        Ok(self.set_columns(columns)?)
    }

    #[classmethod]
    #[pyo3(name = "from_list", text_signature = "(path, data)")]
    fn py_from_list(_cls: &PyType, data: Vec<Vec<FqxValue>>) -> PyResult<Self> {
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

        Ok(FqxData::new(columns, types, data)?)
    }

    #[pyo3(name = "to_list", text_signature = "($self)")]
    fn py_to_list(&self, py: Python<'_>) -> PyObject {
        let res = self
            .data
            .iter()
            .cloned()
            .map(|row| row.0.into_iter().map(|e| e.into_py(py)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        res.into_py(py)
    }

    #[classmethod]
    #[pyo3(name = "from_dict", text_signature = "($self)")]
    fn py_from_dict(_cls: &PyType, data: Vec<HashMap<String, FqxValue>>) -> PyResult<Self> {
        Ok(FqxData::from_objects(data)?)
    }

    #[pyo3(name = "to_records", text_signature = "($self)")]
    fn py_to_records(&self, py: Python<'_>) -> PyObject {
        self.to_objects().into_py(py)
    }

    #[pyo3(name = "to_dataframe", text_signature = "($self)")]
    fn py_to_pandas(&self, py: Python<'_>) -> PyResult<PyObject> {
        let pd = PyModule::import(py, "pandas")?;
        let dataframe = pd.getattr("DataFrame")?;
        let data = self.iter().cloned().map(|r| r.0).collect::<Vec<_>>();
        let df = dataframe.call1((data,))?;
        df.setattr("columns", self.columns.clone())?;

        Ok(df.into())
    }

    #[pyo3(name = "to_json", text_signature = "($self)")]
    fn py_to_json(&self) -> PyResult<String> {
        Ok(serde_json::to_string(&self).map_err(anyhow::Error::msg)?)
    }

    #[pyo3(name = "to_json_pretty", text_signature = "($self)")]
    fn py_to_json_pretty(&self) -> PyResult<String> {
        Ok(serde_json::to_string_pretty(&self).map_err(anyhow::Error::msg)?)
    }

    #[classmethod]
    #[pyo3(name = "from_csv", text_signature = "(path, type_hints)")]
    fn py_from_csv(_cls: &PyType, path: String, type_hints: Vec<FqxValueType>) -> PyResult<Self> {
        Ok(csv_read_rd(path, &type_hints)?)
    }

    #[pyo3(name = "to_csv", text_signature = "($self, path)")]
    fn py_to_csv(&self, path: String) -> PyResult<()> {
        Ok(csv_write_rd(&self, path)?)
    }

    #[classmethod]
    #[pyo3(name = "from_sql", text_signature = "(path, sql, conn)")]
    fn py_from_sql(_cls: &PyType, sql: String, conn: &PySqlConnector) -> PyResult<Self> {
        Ok(conn.fetch(&sql)?)
    }

    #[pyo3(name = "to_sql", text_signature = "($self, table, conn, mode)")]
    fn py_to_sql(&self, table: String, conn: &PySqlConnector, mode: SaveMode) -> PyResult<()> {
        Ok(conn.save(self.clone(), &table, mode)?)
    }

    #[pyo3(name = "to_dataclass", text_signature = "(dataclass_type)")]
    fn py_to_dataclass<'p>(
        &self,
        py: Python<'p>,
        dataclass_type: &'p PyAny,
    ) -> PyResult<Vec<&'p PyAny>> {
        let mut res = vec![];

        for row in self.data.iter() {
            let args = row
                .0
                .iter()
                .cloned()
                .map(|e| e.into_py(py))
                .collect::<Vec<_>>();
            let py_args = PyTuple::new(py, args);
            let obj = dataclass_type.call(py_args, None)?;

            res.push(obj);
        }

        Ok(res)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    // TODO: https://docs.rs/pyo3/latest/pyo3/prelude/struct.Py.html

    #[getter]
    #[pyo3(name = "x")]
    fn py_x<'p>(&self) -> PyX {
        PyX(self.data.clone())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Python native methods

    fn __repr__(&self) -> PyResult<String> {
        self.py_to_json()
    }

    fn __getitem__(&self, py: Python<'_>, idx: PyObject) -> PyResult<Self> {
        let idx = idx.extract::<PyIdx>(py)?;

        Ok(idx.slice_owned(py, self))
    }

    fn __setitem__(&mut self, py: Python<'_>, idx: PyObject, val: PyObject) -> PyResult<()> {
        let idx = idx.extract::<PyIdx>(py)?;
        let val = val.extract::<PyAssign>(py)?;

        Ok(idx.slice_mut(py, self, val)?)
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<FqxII>> {
        let iter = slf.clone().iter_owned();

        Py::new(slf.py(), iter)
    }
}
