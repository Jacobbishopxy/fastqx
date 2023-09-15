//! file: read.rs
//! author: Jacob Xie
//! date: 2023/09/14 21:24:25 Thursday
//! brief:

use std::path::Path;

use anyhow::Result;
use csv::Reader;
use serde::de::DeserializeOwned;

use crate::adt::*;

pub fn csv_read<S, P>(path: P) -> Result<Vec<S>>
where
    S: DeserializeOwned,
    P: AsRef<Path>,
{
    let mut rdr = Reader::from_path(path)?;
    let mut res = vec![];

    for result in rdr.deserialize() {
        let record: S = result?;
        res.push(record);
    }

    Ok(res)
}

pub fn csv_read_rd<P>(path: P, type_hints: &[FastqxValueType]) -> Result<FastqxData>
where
    P: AsRef<Path>,
{
    let mut rdr = Reader::from_path(path)?;

    let mut types = type_hints.to_vec();

    let mut columns = vec![];
    let head_record = rdr.headers()?;
    for (idx, e) in head_record.into_iter().enumerate() {
        columns.push(String::from(e));
        // if `type_hints` its length is shorter than a record, default to `RoughValueType::String`
        if types.get(idx).is_none() {
            types.push(FastqxValueType::String);
        }
    }

    let mut data = vec![];
    for result in rdr.records() {
        let record = result?;
        let mut row = vec![];

        for (idx, e) in record.into_iter().enumerate() {
            if e == "" {
                row.push(FastqxValue::Null);
                continue;
            }

            let th = &types[idx]; // no need edge check
            let value = try_from_str_with_type_hints(e, th)?;
            row.push(value);
        }

        data.push(row);
    }

    Ok(FastqxData {
        columns,
        types,
        data,
    })
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_read {
    use serde::Deserialize;

    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct User {
        id: i32,
        user: String,
        description: Option<String>,
        score: f32,
    }

    #[test]
    fn csv_read_success() {
        let res = csv_read::<User, _>("temp.csv");

        println!("{:?}", res);
        assert!(res.is_ok())
    }

    #[test]
    fn csv_read_rd_success() {
        let res = csv_read_rd(
            "temp.csv",
            &[
                FastqxValueType::I32,
                FastqxValueType::String,
                FastqxValueType::String,
            ],
        );

        println!("{:?}", res);
        assert!(res.is_ok())
    }
}
