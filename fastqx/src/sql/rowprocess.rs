//! file: rowprocess.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:49:27 Wednesday
//! brief:

use anyhow::Result;
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Row, TypeInfo};

use crate::adt::constant::*;
use crate::adt::value::*;

// ================================================================================================
// SqlxRow
// ================================================================================================

pub enum SqlxRow {
    M(MySqlRow),
    P(PgRow),
    S(SqliteRow),
}

impl From<MySqlRow> for SqlxRow {
    fn from(r: MySqlRow) -> Self {
        SqlxRow::M(r)
    }
}

impl From<PgRow> for SqlxRow {
    fn from(r: PgRow) -> Self {
        SqlxRow::P(r)
    }
}

impl From<SqliteRow> for SqlxRow {
    fn from(r: SqliteRow) -> Self {
        SqlxRow::S(r)
    }
}

/// generated code:
///
/// ```rs,ignore
/// let v: Option<bool> = r.try_get(idx)?;
/// Ok(v.map_or(RoughValue::Null, RoughValue::Bool))
/// ```
macro_rules! get_value {
    ($t:ty, $p:ident, $row:expr, $idx:expr) => {{
        let v: Option<$t> = $row.try_get($idx)?;
        Ok(v.map_or(RoughValue::Null, RoughValue::$p))
    }};
}

impl SqlxRow {
    pub fn get_type<S: AsRef<str>>(&self, type_name: S) -> RoughValueType {
        let tn = type_name.as_ref();
        match self {
            SqlxRow::M(_) => MYSQL_TMAP
                .get(tn)
                .cloned()
                .unwrap_or(RoughValueType::String),
            SqlxRow::P(_) => POSTGRES_TMAP
                .get(tn)
                .cloned()
                .unwrap_or(RoughValueType::String),
            SqlxRow::S(_) => SQLITE_TMAP
                .get(tn)
                .cloned()
                .unwrap_or(RoughValueType::String),
        }
    }

    pub fn get_value(
        &self,
        idx: usize,
        value_type: &RoughValueType,
    ) -> Result<RoughValue, sqlx::Error> {
        match &self {
            SqlxRow::M(r) => match value_type {
                RoughValueType::Bool => get_value!(bool, Bool, r, idx),
                RoughValueType::U8 => get_value!(u8, U8, r, idx),
                RoughValueType::U16 => get_value!(u16, U16, r, idx),
                RoughValueType::U32 => get_value!(u32, U32, r, idx),
                RoughValueType::U64 => get_value!(u64, U64, r, idx),
                RoughValueType::I8 => get_value!(i8, I8, r, idx),
                RoughValueType::I16 => get_value!(i16, I16, r, idx),
                RoughValueType::I32 => get_value!(i32, I32, r, idx),
                RoughValueType::I64 => get_value!(i64, I64, r, idx),
                RoughValueType::F32 => get_value!(f32, F32, r, idx),
                RoughValueType::F64 => get_value!(f64, F64, r, idx),
                RoughValueType::String => get_value!(String, String, r, idx),
                RoughValueType::Blob => get_value!(Vec<u8>, Blob, r, idx),
                _ => get_value!(String, String, r, idx),
            },
            SqlxRow::P(r) => match value_type {
                RoughValueType::Bool => get_value!(bool, Bool, r, idx),
                RoughValueType::I8 => get_value!(i8, I8, r, idx),
                RoughValueType::I16 => get_value!(i16, I16, r, idx),
                RoughValueType::I32 => get_value!(i32, I32, r, idx),
                RoughValueType::I64 => get_value!(i64, I64, r, idx),
                RoughValueType::F32 => get_value!(f32, F32, r, idx),
                RoughValueType::F64 => get_value!(f64, F64, r, idx),
                RoughValueType::String => get_value!(String, String, r, idx),
                RoughValueType::Blob => get_value!(Vec<u8>, Blob, r, idx),
                _ => get_value!(String, String, r, idx),
            },
            SqlxRow::S(r) => match value_type {
                RoughValueType::Bool => get_value!(bool, Bool, r, idx),
                RoughValueType::U8 => get_value!(u8, U8, r, idx),
                RoughValueType::U16 => get_value!(u16, U16, r, idx),
                RoughValueType::U32 => get_value!(u32, U32, r, idx),
                RoughValueType::I8 => get_value!(i8, I8, r, idx),
                RoughValueType::I16 => get_value!(i16, I16, r, idx),
                RoughValueType::I32 => get_value!(i32, I32, r, idx),
                RoughValueType::I64 => get_value!(i64, I64, r, idx),
                RoughValueType::F32 => get_value!(f32, F32, r, idx),
                RoughValueType::F64 => get_value!(f64, F64, r, idx),
                RoughValueType::String => get_value!(String, String, r, idx),
                RoughValueType::Blob => get_value!(Vec<u8>, Blob, r, idx),
                _ => get_value!(String, String, r, idx),
            },
        }
    }
}

// ================================================================================================
// SqlxRowProcessor
// ================================================================================================

pub struct SqlxRowProcessor {
    cache: Option<Vec<(String, RoughValueType)>>,
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

impl SqlxRowProcessor {
    pub fn new() -> Self {
        Self { cache: None }
    }

    pub fn cache(&self) -> Option<&[(String, RoughValueType)]> {
        self.cache.as_deref()
    }

    pub fn columns(&self) -> Option<Vec<String>> {
        self.cache
            .as_ref()
            .map(|v| v.into_iter().map(|(e, _)| e.clone()).collect())
    }

    pub fn types(&self) -> Option<Vec<RoughValueType>> {
        self.cache
            .as_ref()
            .map(|v| v.into_iter().map(|(_, t)| t.clone()).collect())
    }

    fn cache_info(&mut self, row: &SqlxRow) -> &[(String, RoughValueType)] {
        if self.cache.is_none() {
            match row {
                SqlxRow::M(r) => cache_info_branch!(row, r, self),
                SqlxRow::P(r) => cache_info_branch!(row, r, self),
                SqlxRow::S(r) => cache_info_branch!(row, r, self),
            }
        }

        self.cache.as_deref().unwrap()
    }

    pub fn process<S: Into<SqlxRow>>(&mut self, row: S) -> Result<Vec<RoughValue>, sqlx::Error> {
        let row: SqlxRow = row.into();
        let cache = self.cache_info(&row);

        let res = cache
            .iter()
            .enumerate()
            .map(|(idx, (_, vt))| row.get_value(idx, vt))
            .collect::<Result<Vec<_>, sqlx::Error>>()?;

        Ok(res)
    }
}
