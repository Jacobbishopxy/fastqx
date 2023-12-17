//! file: data.rs
//! author: Jacob Xie
//! date: 2023/09/11 08:54:05 Monday
//! brief: for both dynamic query and Pyo3

use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use itertools::Itertools;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::adt::{FqxD, FqxRow, FqxValue, FqxValueType, RowProps};

// ================================================================================================
// FqxData
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxInner")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FqxData {
    pub(crate) columns: Vec<String>,
    pub(crate) types: Vec<FqxValueType>,
    pub(crate) data: Vec<FqxRow>,
}

impl FqxData {
    pub fn new<I, S, T, R>(columns: I, types: T, data: Vec<R>) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: ToString,
        T: IntoIterator<Item = FqxValueType>,
        R: Into<FqxRow>,
    {
        let columns = columns
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let types = types.into_iter().collect::<Vec<_>>();

        let c_l = columns.len();
        let t_l = types.len();
        if c_l != t_l {
            bail!(format!("columns len: {c_l}, types len: {t_l}"));
        }

        let mut d = vec![];

        for (idx, row) in data.into_iter().enumerate() {
            let row = row.into();
            let r_l = row.len();
            if c_l != r_l {
                bail!(format!("columns len: {c_l}, row[{idx}] len: {r_l}"));
            }

            d.push(row);
        }

        Ok(Self {
            columns,
            types,
            data: d,
        })
    }

    pub fn new_empty<I, S, T>(columns: I, types: T) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: ToString,
        T: IntoIterator<Item = FqxValueType>,
    {
        let columns = columns
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let types = types.into_iter().collect::<Vec<_>>();

        let c_l = columns.len();
        let t_l = types.len();
        if c_l != t_l {
            bail!(format!("columns len: {c_l}, types len: {t_l}"));
        }

        Ok(Self {
            columns,
            types,
            data: vec![],
        })
    }

    pub fn new_by_data(data: Vec<Vec<FqxValue>>) -> Result<Self> {
        Self::try_from(data)
    }

    pub fn new_uncheck(columns: Vec<String>, types: Vec<FqxValueType>, data: Vec<FqxRow>) -> Self {
        FqxData {
            columns,
            types,
            data,
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn type_coercion(&mut self) -> Result<()> {
        let types = &self.types;

        for row in self.data.iter_mut() {
            for (idx, e) in row.0.iter_mut().enumerate() {
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
                    FqxValueType::Timestamp => {
                        *e = FqxValue::Timestamp(DateTime::<Local>::try_from(e.clone())?)
                    }
                    FqxValueType::DateTime => {
                        *e = FqxValue::DateTime(NaiveDateTime::try_from(e.clone())?)
                    }
                    FqxValueType::Date => *e = FqxValue::Date(NaiveDate::try_from(e.clone())?),
                    FqxValueType::Time => *e = FqxValue::Time(NaiveTime::try_from(e.clone())?),
                    FqxValueType::Null => {
                        // Do nothing
                    }
                }
            }
        }

        Ok(())
    }

    pub fn cast(&mut self, idx: usize, typ: &FqxValueType) -> Result<()> {
        if idx >= self.width() {
            bail!(format!("idx: {idx} out of boundary {}", self.width()))
        }
        for r in self.iter_mut() {
            r.uncheck_cast(idx, typ)?;
        }

        self.types[idx] = typ.clone();

        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn from_hashmaps(objs: Vec<HashMap<String, FqxValue>>) -> Result<Self> {
        let mut peek = objs.into_iter().peekable();
        let first = peek.peek().ok_or_else(|| anyhow!("objects is empty"))?;

        let mut columns = vec![];
        let mut types = vec![];
        for (n, v) in first.iter() {
            columns.push(n.to_owned());
            types.push(FqxValueType::from(v));
        }

        let mut data = vec![];
        for mut obj in peek {
            let mut row = vec![];
            for name in columns.iter() {
                if let Some(v) = obj.remove(name) {
                    row.push(v);
                } else {
                    row.push(FqxValue::Null)
                }
            }
            data.push(FqxRow(row));
        }

        Ok(Self {
            columns,
            types,
            data,
        })
    }

    pub fn to_hashmaps(&self) -> Vec<HashMap<String, FqxValue>> {
        let mut res = vec![];
        for row in self.data.iter() {
            let mut obj = HashMap::new();
            for (i, e) in row.0.iter().enumerate() {
                obj.insert(self.columns[i].clone(), e.clone());
            }
            res.push(obj);
        }

        res
    }

    pub fn from_string(s: &str) -> Result<Self> {
        Ok(serde_json::from_str::<Self>(s).map_err(anyhow::Error::msg)?)
    }

    pub fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self).map_err(anyhow::Error::msg)?)
    }

    pub fn to_pretty_string(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self).map_err(anyhow::Error::msg)?)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl TryFrom<Vec<Vec<FqxValue>>> for FqxData {
    type Error = anyhow::Error;

    fn try_from(value: Vec<Vec<FqxValue>>) -> Result<Self> {
        let mut types = None::<Vec<FqxValueType>>;
        let mut data = vec![];

        for row in value.into_iter() {
            let cur_types = row.iter().map(FqxValueType::from).collect_vec();
            match types {
                Some(t) => {
                    if t.len() != cur_types.len() {
                        bail!("row lengths not equal");
                    }
                    let t = t
                        .into_iter()
                        .zip(cur_types.into_iter())
                        .map(|(p, c)| match (&p, &c) {
                            // update previous `null` type
                            (FqxValueType::Null, _) => Ok(c),
                            // ignore current `null` type
                            (_, FqxValueType::Null) => Ok(p),
                            // type mismatch -> err
                            (t1, t2) if t1 != t2 => {
                                bail!("type mismatch");
                            }
                            _ => Ok(p),
                        })
                        .collect::<Result<Vec<_>>>()?;
                    types = Some(t);
                    data.push(FqxRow(row));
                }
                None => {
                    types = Some(cur_types);
                    data.push(FqxRow(row));
                }
            }
        }
        let types = types.unwrap();
        // default column names
        let columns = (0..types.len())
            .into_iter()
            .map(|i| format!("col_{i}"))
            .collect();
        let res = FqxData {
            columns,
            types,
            data,
        };
        Ok(res)
    }
}

// ================================================================================================
// impl FqxR
// ================================================================================================

impl FqxD for FqxData {
    type ColumnsT = Vec<String>;

    type TypesT = Vec<FqxValueType>;

    type RowT = FqxRow;

    fn cst(c: Self::ColumnsT, t: Self::TypesT, d: Vec<Self::RowT>) -> Self {
        FqxData {
            columns: c,
            types: t,
            data: d,
        }
    }

    fn dcst(self) -> (Self::ColumnsT, Self::TypesT, Vec<Self::RowT>) {
        (self.columns, self.types, self.data)
    }

    fn columns(&self) -> &[String] {
        &self.columns
    }

    fn columns_mut(&mut self) -> &mut [String] {
        &mut self.columns
    }

    fn set_columns(&mut self, cols: Self::ColumnsT) -> Result<()> {
        if self.width() != cols.len() {
            bail!("length mismatch")
        }

        self.columns = cols;

        Ok(())
    }

    fn columns_take(self) -> Vec<String> {
        self.columns
    }

    fn types(&self) -> &[FqxValueType] {
        &self.types
    }

    fn types_mut(&mut self) -> &mut [FqxValueType] {
        &mut self.types
    }

    fn set_types(&mut self, types: Self::TypesT) -> Result<()> {
        if self.width() != types.len() {
            bail!("length mismatch")
        }

        self.types = types;

        Ok(())
    }

    fn types_take(self) -> Vec<FqxValueType> {
        self.types
    }

    fn data(&self) -> &[Self::RowT] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Vec<Self::RowT> {
        &mut self.data
    }

    fn set_data(&mut self, data: Vec<Self::RowT>) -> Result<()> {
        let width = self.width();

        let mut _data = vec![];
        for row in data.into_iter() {
            let mut count = 0;

            for (d, t) in (&row).into_iter().zip(self.types().iter()) {
                if !d.eq(t) {
                    bail!("type mismatch")
                }
                count += 1;
            }

            if width != count {
                bail!("length mismatch")
            }

            _data.push(row);
        }

        *self.data_mut() = _data;

        Ok(())
    }

    fn data_take(self) -> Vec<Self::RowT> {
        self.data
    }

    fn check_row_validation(&self, row: &Self::RowT) -> bool {
        if self.width() != row.len() {
            return false;
        }

        for (v, t) in row.into_iter().zip(self.types()) {
            if v.is_null() {
                continue;
            }
            if !v.is_type(&t) {
                return false;
            }
        }

        true
    }

    fn iter_owned(self) -> std::vec::IntoIter<Self::RowT> {
        self.data.into_iter()
    }

    fn iter(&self) -> std::slice::Iter<'_, Self::RowT> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Self::RowT> {
        self.data.iter_mut()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_data {
    use crate::fqx;

    use super::*;

    #[test]
    fn fqxvalue_print() {
        let foo = FqxValue::F64(123.456);
        println!("{:?}", serde_json::to_string(&foo));

        let foo = FqxValue::Null;
        println!("{:?}", serde_json::to_string(&foo));
    }

    #[test]
    fn new_by_data_success() {
        let d = FqxData::new_by_data(vec![
            vec![fqx!("Apple"), fqx!(107)],
            vec![fqx!("Pear"), fqx!(358)],
        ]);
        assert!(d.is_ok());

        println!("{:?}", d.unwrap());
    }
}
