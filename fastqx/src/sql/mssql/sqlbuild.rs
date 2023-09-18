//! file: sqlbuild.rs
//! author: Jacob Xie
//! date: 2023/09/18 09:29:22 Monday
//! brief:

use anyhow::{anyhow, Result};

use crate::adt::*;

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
            FqxValueType::U16 => return Err(anyhow!("unsupport type: u16")),
            FqxValueType::U32 => return Err(anyhow!("unsupport type: u32")),
            FqxValueType::U64 => return Err(anyhow!("unsupport type: u64")),
            FqxValueType::I8 => return Err(anyhow!("unsupport type: i8")),
            FqxValueType::I16 => cols.push(format!("{} {}", cn, "SMALLINT")),
            FqxValueType::I32 => cols.push(format!("{} {}", cn, "INT")),
            FqxValueType::I64 => cols.push(format!("{} {}", cn, "BIGINT")),
            FqxValueType::F32 => cols.push(format!("{} {}", cn, "FLOAT")),
            FqxValueType::F64 => cols.push(format!("{} {}", cn, "REAL")),
            FqxValueType::String => cols.push(format!("{} {}", cn, "VARCHAR")),
            FqxValueType::Blob => cols.push(format!("{} {}", cn, "BINARY")),
            FqxValueType::Null => return Err(anyhow!("unsupport type: null")),
        }
    }

    let cols = cols.join(",");
    res.push('(');
    res.push_str(&cols);
    res.push(')');
    res.push(';');

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
    res.push_str(") values ");

    let mut vals = vec![];
    for row in data.data.into_iter() {
        let mut r = vec![];
        for v in row.into_iter() {
            let s = match v {
                FqxValue::Bool(v) => format!("{}", v),
                FqxValue::U8(v) => format!("{}", v),
                FqxValue::U16(v) => format!("{}", v),
                FqxValue::U32(v) => format!("{}", v),
                FqxValue::U64(v) => format!("{}", v),
                FqxValue::I8(v) => format!("{}", v),
                FqxValue::I16(v) => format!("{}", v),
                FqxValue::I32(v) => format!("{}", v),
                FqxValue::I64(v) => format!("{}", v),
                FqxValue::F32(v) => format!("{}", v),
                FqxValue::F64(v) => format!("{}", v),
                FqxValue::String(v) => format!("'{}'", v),
                FqxValue::Blob(v) => v
                    .iter()
                    .map(|b| format!("{:02x?}", b).to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                FqxValue::Null => String::from("NULL"),
            };
            r.push(s);
        }
        let r = format!("({})", (r.join(",")));
        vals.push(r);
    }

    res.push_str(&vals.join(", "));
    res.push(';');

    println!(">>> \n{:?}", res);

    res
}
