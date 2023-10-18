//! file: constant.rs
//! author: Jacob Xie
//! date: 2023/09/16 10:32:32 Saturday
//! brief:

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ================================================================================================
// Const
// ================================================================================================

pub(crate) const MYSQL: &str = "mysql";
pub(crate) const POSTGRES: &str = "postgresql";
pub(crate) const MSSQL: &str = "mssql";
pub(crate) const SQLITE: &str = "sqlite";

// ================================================================================================
// SaveMode
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxSaveMode")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SaveMode {
    Override,
    Append,
}

// ================================================================================================
// ConnectorConfig
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxSqlConnectorConfig")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    host: String,
    port: u16,
    user: String,
    pswd: String,
    db: String,
    extra: Option<String>,
}

// ================================================================================================
// Driver
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxSqlDriver")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Driver {
    MYSQL,
    POSTGRES,
    MSSQL,
    SQLITE,
}

#[pymethods]
impl Driver {
    pub fn to_conn_str(&self, config: ConnectorConfig) -> String {
        let ConnectorConfig {
            host,
            port,
            user,
            pswd,
            db,
            extra,
        } = config;

        let conn_str = format!("{user}:{pswd}@{host}:{port}/{db}");

        let mut cs = match self {
            Driver::MYSQL => format!("mysql://{conn_str}"),
            Driver::POSTGRES => format!("postgresql://{conn_str}"),
            Driver::MSSQL => format!("mssql://{conn_str}"),
            Driver::SQLITE => format!("sqlite://{conn_str}"),
        };
        if let Some(ref e) = extra {
            cs.push_str(e);
        }
        cs
    }
}
