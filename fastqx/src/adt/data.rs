//! file: data.rs
//! author: Jacob Xie
//! date: 2023/09/11 08:54:05 Monday
//! brief: for both dynamic query and Pyo3

use std::vec::IntoIter;

use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use pyo3::types::{PySlice, PyTuple, PyType};
use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use super::row::FqxRow;
use super::value::*;
use crate::csv::*;

// ================================================================================================
// FqxData
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxData", get_all)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FqxData {
    pub(crate) columns: Vec<String>,
    pub(crate) types: Vec<FqxValueType>,
    pub(crate) data: Vec<Vec<FqxValue>>,
}

impl FqxData {
    pub fn new(
        columns: Vec<String>,
        types: Vec<FqxValueType>,
        data: Vec<Vec<FqxValue>>,
    ) -> Result<Self> {
        let c_l = columns.len();
        let t_l = types.len();
        if c_l != t_l {
            return Err(anyhow!(format!("columns len: {c_l}, types len: {t_l}")).into());
        }

        for (idx, row) in data.iter().enumerate() {
            let r_l = row.len();
            if c_l != r_l {
                return Err(anyhow!(format!("columns len: {c_l}, row[{idx}] len: {r_l}")).into());
            }
        }

        Ok(Self {
            columns,
            types,
            data,
        })
    }

    pub fn type_coercion(&mut self) -> Result<()> {
        let types = &self.types;

        for row in self.data.iter_mut() {
            for (idx, e) in row.iter_mut().enumerate() {
                if matches!(e, FqxValue::Null) {
                    continue;
                }
                match &types[idx] {
                    FqxValueType::Bool => {
                        *e = FqxValue::Bool(bool::try_from(e.clone())?);
                    }
                    FqxValueType::U8 => {
                        *e = FqxValue::U8(u8::try_from(e.clone())?);
                    }
                    FqxValueType::U16 => {
                        *e = FqxValue::U16(u16::try_from(e.clone())?);
                    }
                    FqxValueType::U32 => {
                        *e = FqxValue::U32(u32::try_from(e.clone())?);
                    }
                    FqxValueType::U64 => {
                        *e = FqxValue::U64(u64::try_from(e.clone())?);
                    }
                    FqxValueType::I8 => {
                        *e = FqxValue::I8(i8::try_from(e.clone())?);
                    }
                    FqxValueType::I16 => {
                        *e = FqxValue::I16(i16::try_from(e.clone())?);
                    }
                    FqxValueType::I32 => {
                        *e = FqxValue::I32(i32::try_from(e.clone())?);
                    }
                    FqxValueType::I64 => {
                        *e = FqxValue::I64(i64::try_from(e.clone())?);
                    }
                    FqxValueType::F32 => {
                        *e = FqxValue::F32(f32::try_from(e.clone())?);
                    }
                    FqxValueType::F64 => {
                        *e = FqxValue::F64(f64::try_from(e.clone())?);
                    }
                    FqxValueType::String => {
                        *e = FqxValue::String(String::try_from(e.clone())?);
                    }
                    FqxValueType::Blob => {
                        *e = FqxValue::Blob(Vec::<u8>::try_from(e.clone())?);
                    }
                    FqxValueType::Null => {
                        // Do nothing
                    }
                }
            }
        }

        Ok(())
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.data.len(), self.data.get(0).map_or(0, |d| d.len()))
    }

    pub fn get_row(&self, r: usize) -> Result<&FqxRow> {
        let rl = self.data.len();
        if r >= rl {
            return Err(anyhow!("out of boundary, row: {rl}, r: {r}"));
        }

        Ok(FqxRow::ref_cast(self.data.get(r).unwrap()))
    }

    pub fn set_row(&mut self, r: usize, row: FqxRow) -> Result<()> {
        let (rl, cl) = self.shape();
        let rowl = row.0.len();

        if r >= rl {
            return Err(anyhow!(format!("out of boundary, row: {rl}, r: {r}")));
        }
        if rowl != cl {
            return Err(anyhow!(format!("shape mismatch, col: {rl}, c: {rl}")));
        }
        for (t, ty) in row.0.iter().zip(self.types.iter()) {
            let tt = FqxValueType::from(t);
            if &tt != ty {
                return Err(anyhow!(format!(
                    "type mismatch, type: {:?}, t: {:?}",
                    ty, tt
                )));
            }
        }

        *(&mut self[r]) = row;

        Ok(())
    }

    pub fn get_value(&self, r: usize, c: usize) -> Result<&FqxValue> {
        let (row, col) = self.shape();
        if r >= row {
            return Err(anyhow!("out of boundary, row: {row}, r: {r}"));
        }
        if c >= col {
            return Err(anyhow!("out of boundary, col: {row}, c: {r}"));
        }

        Ok(self.data.get(r).unwrap().get(c).unwrap())
    }

    pub fn set_value(&mut self, r: usize, c: usize, val: FqxValue) -> Result<()> {
        let (row, col) = self.shape();
        if r >= row {
            return Err(anyhow!("out of boundary, row: {row}, r: {r}"));
        }
        let t = &self.types[r];
        let ty = FqxValueType::from(&val);
        if t != &ty {
            return Err(anyhow!("mismatch type, type: {:?}, val: {:?}", t, ty));
        }
        if c >= col {
            return Err(anyhow!("out of boundary, col: {row}, c: {r}"));
        }

        let v = self.data.get_mut(r).unwrap().get_mut(c).unwrap();
        *v = val;

        Ok(())
    }
}

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

    #[pyo3(name = "to_list", text_signature = "($self)")]
    fn to_py(&self, py: Python<'_>) -> PyObject {
        let res = self
            .data
            .iter()
            .cloned()
            .map(|row| row.into_iter().map(|e| e.into_py(py)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        res.into_py(py)
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
        if let Ok(idx) = mtd.extract::<usize>(py) {
            // return `Vec<FqxValue>`
            Ok(self[idx].0.clone().into_py(py))
        } else if let Ok((row, col)) = mtd.extract::<(usize, usize)>(py) {
            // return `FqxValue`
            Ok(self[row][col].clone().into_py(py))
        } else if let Ok(slice) = mtd.downcast::<PySlice>(py) {
            // return `Vec<Vec<FqxValue>>`
            let res = slice_data(self, self.data.len(), slice);

            Ok(res.into_py(py))
        } else {
            Err(anyhow!("unrecognized mtd").into())
        }
    }

    fn __setitem__(&mut self, idx: usize, val: Vec<FqxValue>) {
        self[idx].0 = val;
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<FqxDataIter>> {
        let iter = FqxDataIter {
            inner: slf.data.clone().into_iter(),
        };

        Py::new(slf.py(), iter)
    }
}

// ================================================================================================
// FqxDataIter
// ================================================================================================

#[pyclass]
pub struct FqxDataIter {
    inner: IntoIter<Vec<FqxValue>>,
}

#[pymethods]
impl FqxDataIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Vec<FqxValue>> {
        slf.inner.next()
    }
}

// ================================================================================================
// Helpers
// ================================================================================================

fn slice_data<I, O>(input: &I, len: usize, slice: &PySlice) -> Vec<O>
where
    I: std::ops::Index<usize, Output = O>,
    O: Clone,
{
    let len = len as isize;
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

    let mut i = if start < stop { start } else { stop };
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

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_data {
    use super::*;

    #[test]
    fn fqxvalue_print() {
        let foo = FqxValue::F64(123.456);
        println!("{:?}", serde_json::to_string(&foo));

        let foo = FqxValue::Null;
        println!("{:?}", serde_json::to_string(&foo));
    }
}
