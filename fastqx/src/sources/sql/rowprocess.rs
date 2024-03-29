//! file: rowprocess.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:49:27 Wednesday
//! brief:

use std::borrow::Cow;

use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Row, TypeInfo};
use tiberius::Row as MsSqlRow;

use crate::adt::*;
use crate::constant::*;

// ================================================================================================
// SqlxRow
// ================================================================================================

pub(crate) enum FqxSqlRow {
    M(MySqlRow),
    P(PgRow),
    S(SqliteRow),
    Q(MsSqlRow),
}

impl From<MySqlRow> for FqxSqlRow {
    fn from(r: MySqlRow) -> Self {
        FqxSqlRow::M(r)
    }
}

impl From<PgRow> for FqxSqlRow {
    fn from(r: PgRow) -> Self {
        FqxSqlRow::P(r)
    }
}

impl From<SqliteRow> for FqxSqlRow {
    fn from(r: SqliteRow) -> Self {
        FqxSqlRow::S(r)
    }
}

impl From<MsSqlRow> for FqxSqlRow {
    fn from(r: MsSqlRow) -> Self {
        FqxSqlRow::Q(r)
    }
}

/// generated code:
///
/// ```rs,ignore
/// let v: Option<bool> = r.try_get(idx)?;
/// Ok(v.map_or(FqxValue::Null, FqxValue::Bool))
/// ```
macro_rules! get_value {
    ($t:ty, $p:ident, $row:expr, $idx:expr) => {{
        let v: Option<$t> = $row.try_get($idx)?;
        Ok(v.map_or(FqxValue::Null, FqxValue::$p))
    }};
}

impl FqxSqlRow {
    fn get_type<S: AsRef<str>>(&self, type_name: S) -> FqxValueType {
        let tn = type_name.as_ref();
        match self {
            FqxSqlRow::M(_) => MYSQL_TMAP.get(tn).cloned().unwrap_or(FqxValueType::String),
            FqxSqlRow::P(_) => POSTGRES_TMAP
                .get(tn)
                .cloned()
                .unwrap_or(FqxValueType::String),
            FqxSqlRow::S(_) => SQLITE_TMAP.get(tn).cloned().unwrap_or(FqxValueType::String),
            FqxSqlRow::Q(_) => unimplemented!(),
        }
    }

    fn get_sqlx_value(
        &self,
        idx: usize,
        value_type: &FqxValueType,
    ) -> Result<FqxValue, sqlx::Error> {
        match self {
            FqxSqlRow::M(r) => match value_type {
                FqxValueType::Bool => get_value!(bool, Bool, r, idx),
                FqxValueType::U8 => get_value!(u8, U8, r, idx),
                FqxValueType::U16 => get_value!(u16, U16, r, idx),
                FqxValueType::U32 => get_value!(u32, U32, r, idx),
                FqxValueType::U64 => get_value!(u64, U64, r, idx),
                FqxValueType::I8 => get_value!(i8, I8, r, idx),
                FqxValueType::I16 => get_value!(i16, I16, r, idx),
                FqxValueType::I32 => get_value!(i32, I32, r, idx),
                FqxValueType::I64 => get_value!(i64, I64, r, idx),
                FqxValueType::F32 => get_value!(f32, F32, r, idx),
                FqxValueType::F64 => get_value!(f64, F64, r, idx),
                FqxValueType::String => get_value!(String, String, r, idx),
                FqxValueType::Blob => get_value!(Vec<u8>, Blob, r, idx),
                FqxValueType::Timestamp => get_value!(DateTime<Local>, Timestamp, r, idx),
                FqxValueType::DateTime => get_value!(NaiveDateTime, DateTime, r, idx),
                FqxValueType::Date => get_value!(NaiveDate, Date, r, idx),
                FqxValueType::Time => get_value!(NaiveTime, Time, r, idx),
                _ => get_value!(String, String, r, idx),
            },
            FqxSqlRow::P(r) => match value_type {
                FqxValueType::Bool => get_value!(bool, Bool, r, idx),
                FqxValueType::I8 => get_value!(i8, I8, r, idx),
                FqxValueType::I16 => get_value!(i16, I16, r, idx),
                FqxValueType::I32 => get_value!(i32, I32, r, idx),
                FqxValueType::I64 => get_value!(i64, I64, r, idx),
                FqxValueType::F32 => get_value!(f32, F32, r, idx),
                FqxValueType::F64 => get_value!(f64, F64, r, idx),
                FqxValueType::String => get_value!(String, String, r, idx),
                FqxValueType::Blob => get_value!(Vec<u8>, Blob, r, idx),
                FqxValueType::Timestamp => get_value!(DateTime<Local>, Timestamp, r, idx),
                FqxValueType::DateTime => get_value!(NaiveDateTime, DateTime, r, idx),
                FqxValueType::Date => get_value!(NaiveDate, Date, r, idx),
                FqxValueType::Time => get_value!(NaiveTime, Time, r, idx),
                _ => get_value!(String, String, r, idx),
            },
            FqxSqlRow::S(r) => match value_type {
                FqxValueType::Bool => get_value!(bool, Bool, r, idx),
                FqxValueType::U8 => get_value!(u8, U8, r, idx),
                FqxValueType::U16 => get_value!(u16, U16, r, idx),
                FqxValueType::U32 => get_value!(u32, U32, r, idx),
                FqxValueType::I8 => get_value!(i8, I8, r, idx),
                FqxValueType::I16 => get_value!(i16, I16, r, idx),
                FqxValueType::I32 => get_value!(i32, I32, r, idx),
                FqxValueType::I64 => get_value!(i64, I64, r, idx),
                FqxValueType::F32 => get_value!(f32, F32, r, idx),
                FqxValueType::F64 => get_value!(f64, F64, r, idx),
                FqxValueType::String => get_value!(String, String, r, idx),
                FqxValueType::Blob => get_value!(Vec<u8>, Blob, r, idx),
                FqxValueType::DateTime => get_value!(NaiveDateTime, DateTime, r, idx),
                FqxValueType::Date => get_value!(NaiveDate, Date, r, idx),
                FqxValueType::Time => get_value!(NaiveTime, Time, r, idx),
                _ => get_value!(String, String, r, idx),
            },
            FqxSqlRow::Q(_) => unimplemented!(),
        }
    }

    // ref: https://docs.rs/tiberius/latest/tiberius/trait.FromSql.html
    fn get_tiberius_value(
        &self,
        idx: usize,
        value_type: &mut FqxValueType,
    ) -> Result<FqxValue, tiberius::error::Error> {
        if let FqxSqlRow::Q(row) = self {
            match value_type {
                FqxValueType::Bool => get_value!(bool, Bool, row, idx),
                FqxValueType::U8 => get_value!(u8, U8, row, idx),
                FqxValueType::I16 => get_value!(i16, I16, row, idx),
                FqxValueType::I32 => get_value!(i32, I32, row, idx),
                FqxValueType::I64 => {
                    // since `tiberius` only returns `intn`, we should do a manually conversion
                    let val: Result<Option<i64>, tiberius::error::Error> = row.try_get(idx);
                    if let Ok(v) = val {
                        return Ok(v.map_or(FqxValue::Null, FqxValue::I64));
                    }

                    let val: Result<Option<i32>, tiberius::error::Error> = row.try_get(idx);
                    if let Ok(v) = val {
                        *value_type = FqxValueType::I32;
                        return Ok(v.map_or(FqxValue::Null, FqxValue::I32));
                    }

                    let val: Result<Option<i16>, tiberius::error::Error> = row.try_get(idx);
                    if let Ok(v) = val {
                        *value_type = FqxValueType::I16;
                        return Ok(v.map_or(FqxValue::Null, FqxValue::I16));
                    }

                    let val: Result<Option<u8>, tiberius::error::Error> = row.try_get(idx);
                    if let Ok(v) = val {
                        *value_type = FqxValueType::U8;
                        return Ok(v.map_or(FqxValue::Null, FqxValue::U8));
                    }

                    Err(tiberius::error::Error::Conversion(Cow::Borrowed(
                        "Conversion of FqxValueType::I64 error",
                    )))
                }
                FqxValueType::F32 => get_value!(f32, F32, row, idx),
                FqxValueType::F64 => {
                    let val: Result<Option<f64>, tiberius::error::Error> = row.try_get(idx);
                    if let Ok(v) = val {
                        return Ok(v.map_or(FqxValue::Null, FqxValue::F64));
                    }

                    let val: Result<Option<f32>, tiberius::error::Error> = row.try_get(idx);
                    if let Ok(v) = val {
                        *value_type = FqxValueType::F32;
                        return Ok(v.map_or(FqxValue::Null, FqxValue::F32));
                    }

                    Err(tiberius::error::Error::Conversion(Cow::Borrowed(
                        "Conversion of FqxValueType::F64 error",
                    )))
                }
                FqxValueType::String => {
                    let v: Option<&str> = row.try_get(idx)?;
                    Ok(v.map_or(FqxValue::Null, |s| FqxValue::String(s.to_owned())))
                }
                FqxValueType::Blob => {
                    let v: Option<&[u8]> = row.try_get(idx)?;
                    Ok(v.map_or(FqxValue::Null, |s| FqxValue::Blob(s.to_owned())))
                }
                FqxValueType::Timestamp => {
                    let v: Option<DateTime<Utc>> = row.try_get(idx)?;
                    Ok(v.map_or(FqxValue::Null, |s| {
                        FqxValue::Timestamp(DateTime::<Local>::from(s))
                    }))
                }
                FqxValueType::DateTime => {
                    let v: Option<NaiveDateTime> = row.try_get(idx)?;
                    Ok(v.map_or(FqxValue::Null, FqxValue::DateTime))
                }
                FqxValueType::Date => {
                    let v: Option<NaiveDate> = row.try_get(idx)?;
                    Ok(v.map_or(FqxValue::Null, FqxValue::Date))
                }
                FqxValueType::Time => {
                    let v: Option<NaiveTime> = row.try_get(idx)?;
                    Ok(v.map_or(FqxValue::Null, FqxValue::Time))
                }
                _ => {
                    let v: Option<&str> = row.try_get(idx)?;
                    Ok(v.map_or(FqxValue::Null, |s| FqxValue::String(s.to_owned())))
                }
            }
        } else {
            unimplemented!()
        }
    }
}

// ================================================================================================
// FqxSqlRowProcessor
// ================================================================================================

pub(crate) struct FqxSqlRowProcessor {
    cache: Option<Vec<(String, FqxValueType)>>,
}

macro_rules! cache_info_branch {
    ($row:expr,$r:expr,$s:expr) => {{
        let c = $r
            .columns()
            .iter()
            .map(|e| {
                let name = e.name().to_string();
                let ty = $row.get_type(e.type_info().name());
                (name, ty)
            })
            .collect::<Vec<_>>();
        $s.cache = Some(c)
    }};
}

impl FqxSqlRowProcessor {
    pub fn new() -> Self {
        Self { cache: None }
    }

    pub fn columns(&self) -> Option<Vec<String>> {
        self.cache
            .as_ref()
            .map(|v| v.into_iter().map(|(e, _)| e.clone()).collect())
    }

    pub fn types(&self) -> Option<Vec<FqxValueType>> {
        self.cache
            .as_ref()
            .map(|v| v.into_iter().map(|(_, t)| t.clone()).collect())
    }

    fn cache_info(&mut self, row: &FqxSqlRow) -> &mut [(String, FqxValueType)] {
        if self.cache.is_none() {
            match row {
                FqxSqlRow::M(r) => cache_info_branch!(row, r, self),
                FqxSqlRow::P(r) => cache_info_branch!(row, r, self),
                FqxSqlRow::S(r) => cache_info_branch!(row, r, self),
                FqxSqlRow::Q(r) => {
                    if self.cache.is_none() {
                        let c = r
                            .columns()
                            .iter()
                            .map(|e| {
                                let name = e.name().to_string();
                                let ty = FqxValueType::from(e.column_type());
                                (name, ty)
                            })
                            .collect::<Vec<_>>();

                        self.cache = Some(c)
                    }
                }
            }
        }

        self.cache.as_deref_mut().unwrap()
    }

    pub fn process_sqlx_row<S: Into<FqxSqlRow>>(&mut self, row: S) -> Result<FqxRow, sqlx::Error> {
        let row: FqxSqlRow = row.into();
        let cache = self.cache_info(&row);

        let res = cache
            .iter()
            .enumerate()
            .map(|(idx, (_, vt))| row.get_sqlx_value(idx, vt))
            .collect::<Result<Vec<_>, sqlx::Error>>()?;

        Ok(FqxRow(res))
    }

    pub fn process_tiberius_row(
        &mut self,
        row: MsSqlRow,
    ) -> Result<FqxRow, tiberius::error::Error> {
        let row = FqxSqlRow::Q(row);
        let cache = self.cache_info(&row);

        let mut res = vec![];
        for (idx, (_, vt)) in cache.iter_mut().enumerate() {
            let val = row.get_tiberius_value(idx, vt)?;
            res.push(val);
        }

        Ok(FqxRow(res))
    }
}
