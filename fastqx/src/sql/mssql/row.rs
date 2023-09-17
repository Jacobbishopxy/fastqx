//! file: row.rs
//! author: Jacob Xie
//! date: 2023/09/17 08:32:33 Sunday
//! brief:

use anyhow::{anyhow, Result};
use tiberius::Row;

pub trait FromTiberiusRow<'r>: Sized {
    fn from_row(row: &'r Row) -> Result<Self>;
}

pub trait TryGetFromRow: Sized {
    fn try_get(row: &Row, col_name: &str) -> Result<Self>;
}

macro_rules! impl_try_get_from_row {
    ($t:ty) => {
        impl TryGetFromRow for $t {
            fn try_get(row: &Row, col_name: &str) -> Result<Self> {
                let val: $t = row
                    .try_get(col_name)?
                    .ok_or(anyhow!(format!("{} is None", col_name)))?;

                Ok(val)
            }
        }

        impl TryGetFromRow for Option<$t> {
            fn try_get(row: &Row, col_name: &str) -> Result<Self> {
                let val: Option<$t> = row.try_get(col_name)?;

                Ok(val)
            }
        }
    };
}

impl_try_get_from_row!(u8);
impl_try_get_from_row!(i16);
impl_try_get_from_row!(i32);
impl_try_get_from_row!(i64);
impl_try_get_from_row!(f32);
impl_try_get_from_row!(f64);
impl_try_get_from_row!(bool);

impl TryGetFromRow for String {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: &str = row
            .try_get(col_name)?
            .ok_or(anyhow!(format!("{} is None", col_name)))?;

        Ok(val.to_string())
    }
}

impl TryGetFromRow for Option<String> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: Option<&str> = row.try_get(col_name)?;

        Ok(val.map(str::to_string))
    }
}

impl TryGetFromRow for Vec<u8> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: &[u8] = row
            .try_get(col_name)?
            .ok_or(anyhow!(format!("{} is None", col_name)))?;

        Ok(val.to_vec())
    }
}

impl TryGetFromRow for Option<Vec<u8>> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: Option<&[u8]> = row.try_get(col_name)?;

        Ok(val.map(Vec::from))
    }
}
