//! file: ops.rs
//! author: Jacob Xie
//! date: 2023/11/11 21:46:22 Saturday
//! brief:

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use fastqx::prelude::*;
use fastqx::serde_json;
use pyo3::prelude::*;

use crate::PyData;

#[pymethods]
impl PyData {
    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // apply

    fn apply(&self, py: Python<'_>, lambda: &PyAny) -> PyResult<Vec<PyObject>> {
        let res = self
            .inner
            .borrow(py)
            .iter()
            .cloned()
            .map(|r| {
                let ans = lambda.call1((r,))?.to_object(py);
                Ok(ans)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(res)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // agg

    fn sum(&self, py: Python<'_>) -> Option<FqxRow> {
        self.inner.borrow(py).data().sum()
    }

    fn min(&self, py: Python<'_>) -> Option<FqxRow> {
        OpAgg::min(self.inner.borrow(py).data())
    }

    fn max(&self, py: Python<'_>) -> Option<FqxRow> {
        OpAgg::max(self.inner.borrow(py).data())
    }

    fn mean(&self, py: Python<'_>) -> Option<FqxRow> {
        self.inner.borrow(py).data().mean()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // cum_agg

    fn cum_sum(&self, py: Python<'_>) -> Vec<FqxRow> {
        self.inner.borrow(py).data().cum_sum()
    }

    fn cum_min(&self, py: Python<'_>) -> Vec<FqxRow> {
        self.inner.borrow(py).data().cum_min()
    }

    fn cum_max(&self, py: Python<'_>) -> Vec<FqxRow> {
        self.inner.borrow(py).data().cum_max()
    }

    fn cum_mean(&self, py: Python<'_>) -> Vec<FqxRow> {
        self.inner.borrow(py).data().cum_mean()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // filter

    fn filter(&self, py: Python<'_>, lambda: &PyAny) -> PyResult<Self> {
        let f = |r: &FqxRow| {
            let res = lambda.call1((r.clone(),))?.extract::<bool>()?;
            Ok::<bool, PyErr>(res)
        };
        let b = self.inner.borrow(py);
        let data = b
            .data()
            .iter()
            .filter(|r| f(r).unwrap_or(false))
            .cloned()
            .collect::<Vec<_>>();

        let res = FqxData::new_uncheck(b.columns().clone(), b.types().clone(), data);

        Ok(Self::from(res))
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // reduce

    fn reduce(&self, py: Python<'_>, lambda: &PyAny) -> PyResult<Option<FqxRow>> {
        let f = |p: FqxRow, c: FqxRow| {
            let res = lambda.call1((p, c))?.extract::<FqxRow>()?;
            Ok(res)
        };
        let b = self.inner.borrow(py);
        let mut iter = b.data().into_iter().cloned();
        let data = iter
            .next()
            .map(|ini| iter.try_fold(ini, |acc, c| f(acc, c)))
            .transpose()?;

        Ok(data)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // group

    fn group_by(&self, py: Python<'_>, keys: Vec<String>) -> PyGroup {
        let res = self
            .inner
            .borrow(py)
            .clone()
            .group_by(keys)
            .to_hashmap()
            .into_iter()
            .map(|(k, v)| (PyGroupKey(k), PyData::from(v)))
            .collect::<HashMap<_, PyData>>();

        PyGroup(res)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // sort

    fn sort_by(&self, py: Python<'_>, lambda: &PyAny) -> PyResult<Self> {
        let f = |p: &FqxRow, c: &FqxRow| {
            let res = lambda.call1((p.clone(), c.clone()))?.extract::<bool>()?;
            Ok::<bool, PyErr>(res)
        };

        let res = self
            .inner
            .borrow(py)
            .clone()
            .sorted_by(|p, c| f(p, c).unwrap_or(true));

        Ok(Self::from(res))
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // merge

    fn merge(
        &self,
        py: Python<'_>,
        other: PyData,
        left_on: Vec<String>,
        right_on: Vec<String>,
        how: String,
    ) -> PyResult<Self> {
        let how = match &how[..] {
            "left" => FqxJoinType::Left,
            "right" => FqxJoinType::Right,
            "outer" => FqxJoinType::Outer,
            "inner" => FqxJoinType::Inner,
            _ => return Err(anyhow!("how: left/right/outer/inner").into()),
        };
        let res = self.inner.borrow(py).clone().merge(
            other.inner.borrow(py).clone(),
            left_on,
            right_on,
            how,
        );

        Ok(Self::from(res))
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // join

    fn join(&self, py: Python<'_>, other: PyData, on: Vec<String>, how: String) -> PyResult<Self> {
        let how = match &how[..] {
            "left" => FqxJoinType::Left,
            "right" => FqxJoinType::Right,
            "outer" => FqxJoinType::Outer,
            "inner" => FqxJoinType::Inner,
            _ => return Err(anyhow!("how: left/right/outer/inner").into()),
        };
        let res = self
            .inner
            .borrow(py)
            .clone()
            .join(other.inner.borrow(py).clone(), on, how);

        Ok(Self::from(res))
    }
}

// ================================================================================================
// PyGroupKey & PyGroup
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxGroupKey")]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PyGroupKey(Vec<FqxValue>);

#[pymethods]
impl PyGroupKey {
    fn __get__(&self, _instance: PyObject, _owner: PyObject) -> Vec<FqxValue> {
        self.0.clone()
    }

    fn __set__(&mut self, _instance: PyObject, value: Vec<FqxValue>) {
        self.0 = value
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(serde_json::to_string_pretty(&self.0).map_err(anyhow::Error::msg)?)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(serde_json::to_string(&self.0).map_err(anyhow::Error::msg)?)
    }

    fn __getitem__(&self, idx: isize) -> FqxValue {
        self.0[idx as usize].clone()
    }

    fn __setitem__(&mut self, idx: isize, val: FqxValue) {
        self.0[idx as usize] = val;
    }

    #[pyo3(name = "to_str")]
    fn py_to_str(&self) -> PyResult<String> {
        self.__str__()
    }

    #[pyo3(name = "types")]
    fn py_types(&self) -> Vec<FqxValueType> {
        self.0.iter().map(FqxValueType::from).collect()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: group ops

#[pyclass]
#[pyo3(name = "FqxGroup")]
pub struct PyGroup(HashMap<PyGroupKey, PyData>);

#[pymethods]
impl PyGroup {
    fn __get__(&self, _instance: PyObject, _owner: PyObject) -> HashMap<PyGroupKey, PyData> {
        self.0.clone()
    }

    fn __set__(&mut self, _instance: PyObject, value: HashMap<PyGroupKey, PyData>) {
        self.0 = value;
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __getitem__(&self, key: Vec<FqxValue>) -> Option<PyData> {
        self.0.get(&PyGroupKey(key)).map(|v| v.clone())
    }

    fn __setitem__(&mut self, key: Vec<FqxValue>, value: PyData) {
        self.0.insert(PyGroupKey(key), value);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // py dict iter

    fn items(&self, py: Python<'_>) -> PyResult<Py<PyGroupIter>> {
        let iter = PyGroupIter {
            inner: self.0.clone().into_iter(),
        };

        Py::new(py, iter)
    }

    fn keys(&self, py: Python<'_>) -> PyResult<Py<PyGroupKeyIter>> {
        let iter = PyGroupKeyIter {
            inner: self.0.clone().into_keys(),
        };

        Py::new(py, iter)
    }

    fn values(&self, py: Python<'_>) -> PyResult<Py<PyGroupValueIter>> {
        let iter = PyGroupValueIter {
            inner: self.0.clone().into_values(),
        };

        Py::new(py, iter)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // ops

    // TODO
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// `.items()`

#[pyclass]
struct PyGroupIter {
    inner: std::collections::hash_map::IntoIter<PyGroupKey, PyData>,
}

#[pymethods]
impl PyGroupIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(PyGroupKey, PyData)> {
        slf.inner.next()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// `.keys()`

#[pyclass]
struct PyGroupKeyIter {
    inner: std::collections::hash_map::IntoKeys<PyGroupKey, PyData>,
}

#[pymethods]
impl PyGroupKeyIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyGroupKey> {
        slf.inner.next()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// `.values()`

#[pyclass]
struct PyGroupValueIter {
    inner: std::collections::hash_map::IntoValues<PyGroupKey, PyData>,
}

#[pymethods]
impl PyGroupValueIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyData> {
        slf.inner.next()
    }
}
