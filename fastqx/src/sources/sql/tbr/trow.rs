//! file: row.rs
//! author: Jacob Xie
//! date: 2023/09/17 08:32:33 Sunday
//! brief:

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, LocalResult, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use tiberius::Row;

pub trait FromTiberiusRow: Sized {
    fn from_row(row: Row) -> Result<Self>;
}

pub trait TryGetFromTiberiusRow: Sized {
    fn try_get(row: &Row, col_name: &str) -> Result<Self>;
}

macro_rules! impl_try_get_from_row {
    ($t:ty) => {
        impl TryGetFromTiberiusRow for $t {
            fn try_get(row: &Row, col_name: &str) -> Result<Self> {
                let val: $t = row
                    .try_get(col_name)?
                    .ok_or(anyhow!(format!("{} is None", col_name)))?;

                Ok(val)
            }
        }

        impl TryGetFromTiberiusRow for Option<$t> {
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

impl TryGetFromTiberiusRow for String {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: &str = row
            .try_get(col_name)?
            .ok_or(anyhow!(format!("{} is None", col_name)))?;

        Ok(val.to_string())
    }
}

impl TryGetFromTiberiusRow for Option<String> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: Option<&str> = row.try_get(col_name)?;

        Ok(val.map(str::to_string))
    }
}

impl TryGetFromTiberiusRow for Vec<u8> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: &[u8] = row
            .try_get(col_name)?
            .ok_or(anyhow!(format!("{} is None", col_name)))?;

        Ok(val.to_vec())
    }
}

impl TryGetFromTiberiusRow for Option<Vec<u8>> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: Option<&[u8]> = row.try_get(col_name)?;

        Ok(val.map(Vec::from))
    }
}

impl TryGetFromTiberiusRow for NaiveDateTime {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val = row
            .try_get(col_name)?
            .ok_or(anyhow!(format!("{} is None", col_name)))?;

        Ok(val)
    }
}

impl TryGetFromTiberiusRow for Option<NaiveDateTime> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val = row.try_get(col_name)?;

        Ok(val)
    }
}

impl TryGetFromTiberiusRow for DateTime<Local> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: NaiveDateTime = row
            .try_get(col_name)?
            .ok_or(anyhow!(format!("{} is None", col_name)))?;

        if let LocalResult::Single(dtl) = Local.from_local_datetime(&val) {
            return Ok(dtl);
        }

        Err(anyhow!("convert err"))
    }
}

impl TryGetFromTiberiusRow for Option<DateTime<Local>> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val: Option<NaiveDateTime> = row.try_get(col_name)?;

        let val = val.and_then(|e| {
            if let LocalResult::Single(dtl) = Local.from_local_datetime(&e) {
                Some(dtl)
            } else {
                None
            }
        });

        Ok(val)
    }
}

impl TryGetFromTiberiusRow for NaiveDate {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val = row
            .try_get(col_name)?
            .ok_or(anyhow!(format!("{} is None", col_name)))?;

        Ok(val)
    }
}

impl TryGetFromTiberiusRow for Option<NaiveDate> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val = row.try_get(col_name)?;

        Ok(val)
    }
}

impl TryGetFromTiberiusRow for NaiveTime {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val = row
            .try_get(col_name)?
            .ok_or(anyhow!(format!("{} is None", col_name)))?;

        Ok(val)
    }
}

impl TryGetFromTiberiusRow for Option<NaiveTime> {
    fn try_get(row: &Row, col_name: &str) -> Result<Self> {
        let val = row.try_get(col_name)?;

        Ok(val)
    }
}
