//! file: constant.rs
//! author: Jacob Xie
//! date: 2023/09/16 10:32:32 Saturday
//! brief:

// ================================================================================================
// Const
// ================================================================================================

pub(crate) const MYSQL: &str = "mysql";
pub(crate) const POSTGRES: &str = "postgres";
#[allow(dead_code)]
pub(crate) const MSSQL: &str = "mssql";
pub(crate) const SQLITE: &str = "sqlite";

pub enum SaveMode {
    Override,
    Append,
}
