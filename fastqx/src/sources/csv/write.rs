//! file: write.rs
//! author: Jacob Xie
//! date: 2023/09/14 21:24:33 Thursday
//! brief:

use std::borrow::Borrow;
use std::path::Path;

use anyhow::Result;
use csv::Writer;
use serde::Serialize;

use crate::adt::FqxData;

pub fn csv_write<I, E, S, P>(data: I, path: P) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: Borrow<E>,
    E: Serialize,
    P: AsRef<Path>,
{
    let mut wtr = Writer::from_path(path)?;

    for row in data.into_iter() {
        wtr.serialize(row.borrow())?;
    }

    wtr.flush()?;

    Ok(())
}

pub fn csv_write_rd<P>(data: &FqxData, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut wtr = Writer::from_path(path)?;

    wtr.write_record(&data.columns)?;

    for row in &data.data {
        wtr.write_record(row.0.iter().map(ToString::to_string).collect::<Vec<_>>())?;
    }

    wtr.flush()?;

    Ok(())
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_write {
    use super::*;
    use crate::adt::{FqxValue, FqxValueType};

    #[derive(Serialize)]
    struct User {
        id: i32,
        user: String,
        description: Option<String>,
        score: f32,
    }

    #[test]
    fn csv_write_success() {
        let users = vec![
            User {
                id: 1,
                user: "A".to_string(),
                description: None,
                score: 2.1,
            },
            User {
                id: 2,
                user: "B".to_string(),
                description: Some("2nd".to_string()),
                score: 2.1,
            },
        ];

        let res = csv_write::<&Vec<User>, User, &User, _>(&users, "temp.csv");
        assert!(res.is_ok());

        let res = csv_write(users, "temp.csv");
        assert!(res.is_ok());
    }

    #[test]
    fn csv_write_rd_success() {
        let data = FqxData::new(
            vec![String::from("c1"), String::from("c2"), String::from("c3")],
            vec![FqxValueType::I32, FqxValueType::String, FqxValueType::F32],
            vec![
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("A")),
                    FqxValue::F32(2.1),
                ],
                vec![
                    FqxValue::I32(2),
                    FqxValue::String(String::from("B")),
                    FqxValue::F32(1.3),
                ],
                vec![
                    FqxValue::I32(3),
                    FqxValue::String(String::from("C")),
                    FqxValue::F32(3.2),
                ],
            ],
        )
        .unwrap();

        let res = csv_write_rd(&data, "temp_rd.csv");
        assert!(res.is_ok());
    }
}
