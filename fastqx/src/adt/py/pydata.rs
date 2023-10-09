//! file: pydata.rs
//! author: Jacob Xie
//! date: 2023/10/02 23:16:01 Monday
//! brief:

use std::collections::HashMap;

use anyhow::anyhow;
use pyo3::prelude::*;
use pyo3::types::{PySlice, PyTuple, PyType};

use crate::adt::ab::iter::FqxII;
use crate::adt::{FqxData, FqxRow, FqxValue, FqxValueType};
use crate::sources::csv::*;

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

    #[pyo3(name = "to_dict", text_signature = "($self)")]
    fn py_to_dict(&self, py: Python<'_>) -> PyObject {
        self.to_objects().into_py(py)
    }

    #[pyo3(name = "to_dict_json", text_signature = "($self)")]
    fn py_to_dict_json(&self) -> PyResult<String> {
        Ok(serde_json::to_string(&self.to_objects()).map_err(|e| anyhow!(e))?)
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
        Ok(serde_json::to_string(&self).map_err(|e| anyhow!(e))?)
    }

    #[pyo3(name = "to_json_pretty", text_signature = "($self)")]
    fn py_to_json_pretty(&self) -> PyResult<String> {
        Ok(serde_json::to_string_pretty(&self).map_err(|e| anyhow!(e))?)
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

    fn __repr__(&self) -> PyResult<String> {
        self.py_to_json()
    }

    fn __getitem__(&self, py: Python<'_>, mtd: PyObject) -> PyResult<PyObject> {
        let len = self.data.len() as isize;
        if let Ok(idx) = mtd.extract::<isize>(py) {
            let idx = if idx < 0 { len + idx } else { idx };
            // return `Vec<FqxValue>`
            Ok(self[idx as usize].0.clone().into_py(py))
        } else if let Ok((row, col)) = mtd.extract::<(isize, isize)>(py) {
            let row = if row < 0 { len + row } else { row };
            let col = if col < 0 { len + col } else { col };
            // return `FqxValue`
            Ok(self[row as usize][col as usize].clone().into_py(py))
        } else if let Ok(slice) = mtd.downcast::<PySlice>(py) {
            // return `Vec<Vec<FqxValue>>`
            let rows = slice_data(self, len, slice);
            Ok(rows.into_py(py))
        } else {
            Err(anyhow!("unrecognized mtd, accept: data[x], data[x,y], data[x1:x2]").into())
        }
    }

    fn __setitem__(&mut self, py: Python<'_>, mtd: PyObject, val: PyObject) -> PyResult<()> {
        let len = self.data.len() as isize;
        if let Ok(idx) = mtd.extract::<isize>(py) {
            let idx = if idx < 0 { len + idx } else { idx };
            let val = val.extract::<Vec<FqxValue>>(py)?;
            // set `Vec<FqxValue>`
            self[idx as usize].0 = val;
            Ok(())
        } else if let Ok((row, col)) = mtd.extract::<(isize, isize)>(py) {
            let row = if row < 0 { len + row } else { row };
            let col = if col < 0 { len + col } else { col };
            let val = val.extract::<FqxValue>(py)?;
            // set `FqxValue`
            self[row as usize][col as usize] = val;
            Ok(())
        } else if let Ok(slice) = mtd.downcast::<PySlice>(py) {
            // set `Vec<Vec<FqxValue>>`
            let val = val
                .extract::<Vec<Vec<FqxValue>>>(py)?
                .into_iter()
                .map(FqxRow)
                .collect();
            slice_data_mut(self, len, slice, val);
            Ok(())
        } else {
            Err(anyhow!("unrecognized mtd, accept: data[x], data[x,y], data[x1:x2]").into())
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<FqxII>> {
        let iter = slf.clone().iter_owned();

        Py::new(slf.py(), iter)
    }
}

// ================================================================================================
// Helpers
// ================================================================================================

fn de_slice(len: isize, slice: &PySlice) -> (isize, isize, isize, isize) {
    let mut start = slice
        .getattr("start")
        .and_then(|s| s.extract::<isize>())
        .unwrap_or(0);
    if start < 0 {
        start = len + start
    }
    let mut stop = slice
        .getattr("stop")
        .and_then(|s| s.extract::<isize>())
        .unwrap_or(len);
    if stop < 0 {
        stop = len + stop;
    }
    let mut step = slice
        .getattr("step")
        .and_then(|s| s.extract::<isize>())
        .unwrap_or(1);
    if step < 0 {
        step = -step;
    }

    let i = if start < stop { start } else { stop };

    (start, stop, step, i)
}

fn slice_data<I, O>(input: &I, len: isize, slice: &PySlice) -> Vec<O>
where
    I: std::ops::Index<usize, Output = O>,
    O: Clone,
{
    let (start, stop, step, mut i) = de_slice(len, slice);
    let mut res = vec![];

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < len {
            res.push(input[i as usize].clone())
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }

    res
}

fn slice_data_mut<'m, I, O>(input: &'m mut I, len: isize, slice: &PySlice, val: Vec<O>)
where
    I: std::ops::IndexMut<usize, Output = O>,
    O: Sized + Clone,
{
    let (start, stop, step, mut i) = de_slice(len, slice);
    let mut val_i = 0;

    while (start < stop && i < stop) || (start > stop && i > stop) {
        if i >= 0 && i < len {
            input[i as usize] = val[val_i].clone();
            val_i += 1;
        }

        if start < stop {
            i += step;
        } else {
            i -= step;
        }
    }
}
