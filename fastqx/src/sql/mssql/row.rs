//! file: row.rs
//! author: Jacob Xie
//! date: 2023/09/17 08:32:33 Sunday
//! brief:

use tiberius::error::Error;
use tiberius::Row;

pub trait FromTiberiusRow<'r>: Sized {
    fn from_row(row: &'r Row) -> Result<Self, Error>;
}
