//! file: rowprocess.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:49:27 Wednesday
//! brief:

use anyhow::Result;
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Row, TypeInfo};
use tiberius::Row as MsSqlRow;

use crate::adt::constant::*;
use crate::adt::value::*;

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
/// Ok(v.map_or(FastqxValue::Null, FastqxValue::Bool))
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
                _ => get_value!(String, String, r, idx),
            },
            FqxSqlRow::Q(_) => unimplemented!(),
        }
    }

    fn get_tiberius_value(
        row: &MsSqlRow,
        idx: usize,
        value_type: &FqxValueType,
    ) -> Result<FqxValue, tiberius::error::Error> {
        match value_type {
            FqxValueType::Bool => get_value!(bool, Bool, row, idx),
            FqxValueType::U8 => get_value!(u8, U8, row, idx),
            FqxValueType::I16 => get_value!(i16, I16, row, idx),
            FqxValueType::I32 => get_value!(i32, I32, row, idx),
            FqxValueType::I64 => get_value!(i64, I64, row, idx),
            FqxValueType::F32 => get_value!(f32, F32, row, idx),
            FqxValueType::F64 => get_value!(f64, F64, row, idx),
            FqxValueType::String => {
                let v: Option<&str> = row.try_get(idx)?;
                Ok(v.map_or(FqxValue::Null, |s| FqxValue::String(s.to_owned())))
            }
            FqxValueType::Blob => {
                let v: Option<&[u8]> = row.try_get(idx)?;
                Ok(v.map_or(FqxValue::Null, |s| FqxValue::Blob(s.to_owned())))
            }
            _ => {
                let v: Option<&str> = row.try_get(idx)?;
                Ok(v.map_or(FqxValue::Null, |s| FqxValue::String(s.to_owned())))
            }
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

    fn cache_sqlx_info(&mut self, row: &FqxSqlRow) -> &[(String, FqxValueType)] {
        if self.cache.is_none() {
            match row {
                FqxSqlRow::M(r) => cache_info_branch!(row, r, self),
                FqxSqlRow::P(r) => cache_info_branch!(row, r, self),
                FqxSqlRow::S(r) => cache_info_branch!(row, r, self),
                FqxSqlRow::Q(_) => unimplemented!(),
            }
        }

        self.cache.as_deref().unwrap()
    }

    fn cache_tiberius_info(&mut self, row: &MsSqlRow) -> &[(String, FqxValueType)] {
        if self.cache.is_none() {
            let c = row
                .columns()
                .iter()
                .map(|e| {
                    let name = e.name().to_string();

                    // TODO
                    dbg!(e.column_type());
                    let ty = FqxValueType::from(e.column_type());
                    (name, ty)
                })
                .collect::<Vec<_>>();

            self.cache = Some(c)
        }

        self.cache.as_deref().unwrap()
    }

    pub fn process_sqlx_row<S: Into<FqxSqlRow>>(
        &mut self,
        row: S,
    ) -> Result<Vec<FqxValue>, sqlx::Error> {
        let row: FqxSqlRow = row.into();
        let cache = self.cache_sqlx_info(&row);

        let res = cache
            .iter()
            .enumerate()
            .map(|(idx, (_, vt))| row.get_sqlx_value(idx, vt))
            .collect::<Result<Vec<_>, sqlx::Error>>()?;

        Ok(res)
    }

    pub fn process_tiberius_row(
        &mut self,
        row: MsSqlRow,
    ) -> Result<Vec<FqxValue>, tiberius::error::Error> {
        let cache = self.cache_tiberius_info(&row);

        let res = cache
            .iter()
            .enumerate()
            .map(|(idx, (_, vt))| FqxSqlRow::get_tiberius_value(&row, idx, vt))
            .collect::<Result<Vec<_>, tiberius::error::Error>>()?;

        Ok(res)
    }
}
