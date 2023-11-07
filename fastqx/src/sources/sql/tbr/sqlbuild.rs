//! file: sqlbuild.rs
//! author: Jacob Xie
//! date: 2023/09/18 09:29:22 Monday
//! brief:

use anyhow::{anyhow, Result};

use crate::adt::*;

// ================================================================================================
// ToSqlString
// ================================================================================================

pub trait ToSqlString: Sized {
    fn to_sql(self) -> String;
}

macro_rules! impl_try_to_sql_string {
    ($t:ty) => {
        impl ToSqlString for $t {
            fn to_sql(self) -> String {
                format!("{}", self)
            }
        }

        impl ToSqlString for Option<$t> {
            fn to_sql(self) -> String {
                String::from("NULL")
            }
        }
    };
}

impl_try_to_sql_string!(u8);
impl_try_to_sql_string!(u16);
impl_try_to_sql_string!(u32);
impl_try_to_sql_string!(u64);
impl_try_to_sql_string!(i8);
impl_try_to_sql_string!(i16);
impl_try_to_sql_string!(i32);
impl_try_to_sql_string!(i64);
impl_try_to_sql_string!(f32);
impl_try_to_sql_string!(f64);

impl ToSqlString for bool {
    fn to_sql(self) -> String {
        if self {
            String::from("1")
        } else {
            String::from("0")
        }
    }
}

impl ToSqlString for Option<bool> {
    fn to_sql(self) -> String {
        String::from("NULL")
    }
}

impl ToSqlString for String {
    fn to_sql(self) -> String {
        format!("'{}'", self)
    }
}

impl ToSqlString for Option<String> {
    fn to_sql(self) -> String {
        String::from("NULL")
    }
}

impl ToSqlString for Vec<u8> {
    fn to_sql(self) -> String {
        self.iter()
            .map(|b| format!("{:02x?}", b).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl ToSqlString for Option<Vec<u8>> {
    fn to_sql(self) -> String {
        String::from("NULL")
    }
}

// ================================================================================================
// MsSql Sql Builder
// ================================================================================================

pub(crate) fn create_table(data: &FqxData, table_name: &str) -> Result<String> {
    let mut res = format!(
        "IF OBJECT_ID(N'{}', N'U') IS NULL CREATE TABLE {} ",
        table_name, table_name
    );

    let mut cols = vec![];
    for (cn, ty) in data.columns.iter().zip(data.types.iter()) {
        match ty {
            FqxValueType::Bool => cols.push(format!("{} {}", cn, "BIT")),
            FqxValueType::U8 => cols.push(format!("{} {}", cn, "TINYINT")),
            FqxValueType::U16 => cols.push(format!("{} {}", cn, "SMALLINT")),
            FqxValueType::U32 => cols.push(format!("{} {}", cn, "INT")),
            FqxValueType::U64 => cols.push(format!("{} {}", cn, "BIGINT")),
            FqxValueType::I8 => cols.push(format!("{} {}", cn, "TINYINT")),
            FqxValueType::I16 => cols.push(format!("{} {}", cn, "SMALLINT")),
            FqxValueType::I32 => cols.push(format!("{} {}", cn, "INT")),
            FqxValueType::I64 => cols.push(format!("{} {}", cn, "BIGINT")),
            FqxValueType::F32 => cols.push(format!("{} {}", cn, "FLOAT(24)")),
            FqxValueType::F64 => cols.push(format!("{} {}", cn, "FLOAT(53)")),
            FqxValueType::String => cols.push(format!("{} {}", cn, "VARCHAR(100)")),
            FqxValueType::Blob => cols.push(format!("{} {}", cn, "BINARY")),
            FqxValueType::Null => return Err(anyhow!("unsupport type: null")),
            _ => todo!(),
        }
    }

    let cols = cols.join(",");
    res.push('(');
    res.push_str(&cols);
    res.push_str(");");

    Ok(res)
}

pub(crate) fn drop_table(table_name: &str) -> String {
    format!("DROP TABLE IF EXISTS {};", table_name)
}

pub(crate) fn insert(data: FqxData, table_name: &str) -> String {
    let mut res = format!("INSERT INTO {} ", table_name);

    let cols = data.columns.join(",");
    res.push('(');
    res.push_str(&cols);
    res.push_str(") VALUES ");

    let mut vals = vec![];
    for row in data.data.into_iter() {
        let mut r = vec![];
        for v in row.0.into_iter() {
            let s = match v {
                FqxValue::Bool(v) => ToSqlString::to_sql(v),
                FqxValue::U8(v) => ToSqlString::to_sql(v),
                FqxValue::U16(v) => ToSqlString::to_sql(v),
                FqxValue::U32(v) => ToSqlString::to_sql(v),
                FqxValue::U64(v) => ToSqlString::to_sql(v),
                FqxValue::I8(v) => ToSqlString::to_sql(v),
                FqxValue::I16(v) => ToSqlString::to_sql(v),
                FqxValue::I32(v) => ToSqlString::to_sql(v),
                FqxValue::I64(v) => ToSqlString::to_sql(v),
                FqxValue::F32(v) => ToSqlString::to_sql(v),
                FqxValue::F64(v) => ToSqlString::to_sql(v),
                FqxValue::String(v) => ToSqlString::to_sql(v),
                FqxValue::Blob(v) => ToSqlString::to_sql(v),
                FqxValue::Null => String::from("NULL"),
                _ => todo!(),
            };
            r.push(s);
        }
        let r = format!("({})", r.join(","));
        vals.push(r);
    }

    res.push_str(&vals.join(", "));
    res.push(';');

    res
}
