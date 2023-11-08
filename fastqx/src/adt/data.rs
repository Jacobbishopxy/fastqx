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

use crate::adt::ab::d::{FqxAffiliate, FqxD};
use crate::adt::{FqxRow, FqxValue, FqxValueType};

// ================================================================================================
// FqxData
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxData", get_all)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FqxData {
    pub(crate) columns: Vec<String>,
    pub(crate) types: Vec<FqxValueType>,
    pub(crate) data: Vec<FqxRow>,
}

impl FqxData {
    pub fn new<I, S, T>(columns: I, types: T, data: Vec<Vec<FqxValue>>) -> Result<Self>
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

        for (idx, row) in data.iter().enumerate() {
            let r_l = row.len();
            if c_l != r_l {
                bail!(format!("columns len: {c_l}, row[{idx}] len: {r_l}"));
            }
        }

        let data = data.into_iter().map(|r| FqxRow(r)).collect::<Vec<_>>();

        Ok(Self {
            columns,
            types,
            data,
        })
    }

    pub fn new_by_data(data: Vec<Vec<FqxValue>>) -> Result<Self> {
        Self::try_from(data)
    }

    pub fn set_columns<I, S>(&mut self, columns: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        let columns = columns
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        if columns.len() != self.columns().len() {
            bail!("length mismatch")
        } else {
            self.columns = columns;
            Ok(())
        }
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        // self.data.get(0).map_or(0, |d| d.len())
        self.columns.len()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.height(), self.width())
    }

    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    pub fn types(&self) -> &[FqxValueType] {
        &self.types
    }

    pub fn data(&self) -> &[FqxRow] {
        &self.data
    }

    pub fn get_positions<I, S>(&self, keys: I) -> Vec<usize>
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.columns_position(keys.into_iter().map(|e| e.to_string()).collect())
    }

    pub fn get_row(&self, r: usize) -> Result<&FqxRow> {
        let rl = self.data.len();
        if r >= rl {
            bail!("out of boundary, row: {rl}, r: {r}");
        }

        Ok(self.data.get(r).unwrap())
    }

    pub fn set_row(&mut self, r: usize, row: FqxRow) -> Result<()> {
        let (rl, cl) = self.shape();
        let rowl = row.0.len();

        if r >= rl {
            bail!(format!("out of boundary, row: {rl}, r: {r}"));
        }
        if rowl != cl {
            bail!(format!("shape mismatch, col: {rl}, c: {rl}"));
        }
        for (t, ty) in row.0.iter().zip(self.types.iter()) {
            let tt = FqxValueType::from(t);
            if &tt != ty {
                bail!(format!("type mismatch, type: {:?}, t: {:?}", ty, tt));
            }
        }

        *(&mut self[r]) = row;

        Ok(())
    }

    pub fn get_value(&self, r: usize, c: usize) -> Result<&FqxValue> {
        let (row, col) = self.shape();
        if r >= row {
            bail!("out of boundary, row: {row}, r: {r}");
        }
        if c >= col {
            bail!("out of boundary, col: {row}, c: {r}");
        }

        Ok(&self.data[r][c])
    }

    pub fn set_value(&mut self, r: usize, c: usize, val: FqxValue) -> Result<()> {
        let (row, col) = self.shape();
        if r >= row {
            bail!("out of boundary, row: {row}, r: {r}");
        }
        let t = &self.types[r];
        let ty = FqxValueType::from(&val);
        if t != &ty {
            bail!("mismatch type, type: {:?}, val: {:?}", t, ty);
        }
        if c >= col {
            bail!("out of boundary, col: {row}, c: {r}");
        }

        let v = &mut self.data[r][c];
        *v = val;

        Ok(())
    }

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
        } else {
            for r in self.iter_mut() {
                r.uncheck_cast(idx, typ)?;
            }

            self.types[idx] = typ.clone();

            Ok(())
        }
    }

    pub fn from_objects(objs: Vec<HashMap<String, FqxValue>>) -> Result<Self> {
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

    pub fn to_objects(&self) -> Vec<HashMap<String, FqxValue>> {
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

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn empty_row(&self) -> FqxRow {
        FqxRow(vec![FqxValue::Null; self.width()])
    }
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
// FqxD
// ================================================================================================

impl FqxD<String, FqxValueType, FqxRow, FqxValue> for FqxData {
    fn columns(&self) -> &[String] {
        &self.columns
    }

    fn types(&self) -> &[FqxValueType] {
        &self.types
    }

    fn data(&self) -> &[FqxRow] {
        &self.data
    }

    fn dcst(self) -> (Vec<String>, Vec<FqxValueType>, Vec<FqxRow>) {
        let FqxData {
            columns,
            types,
            data,
        } = self;
        (columns, types, data)
    }

    fn cst(columns: Vec<String>, types: Vec<FqxValueType>, data: Vec<FqxRow>) -> Self {
        Self {
            columns,
            types,
            data,
        }
    }
}

// ================================================================================================
// FqxAffiliate
// ================================================================================================

impl FqxAffiliate<String, FqxValueType, FqxRow, FqxValue> for FqxData {}

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
