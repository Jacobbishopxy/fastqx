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
pub(crate) const POSTGRES: &str = "postgres";
pub(crate) const MSSQL: &str = "jdbc:sqlserver";
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
#[pyo3(name = "FqxConnectorConfig")]
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
#[pyo3(name = "FqxDriver")]
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
            pswd: pass,
            db,
            extra,
        } = config;

        match self {
            Driver::MYSQL => {
                let mut cs = format!("mysql://{user}:{pass}@{host}:{port}/{db}");
                if let Some(ref e) = extra {
                    cs.push_str(e);
                }
                cs
            }
            Driver::POSTGRES => {
                let mut cs = format!("postgres://{user}:{pass}@{host}:{port}/{db}");
                if let Some(ref e) = extra {
                    cs.push_str(e);
                }
                cs
            }
            Driver::MSSQL => {
                let mut cs = format!(
                "jdbc:sqlserver://{host}:{port};username={user};password={pass};databaseName={db};"
            );
                // ec. encrypt=true;integratedSecurity=true
                if let Some(ref e) = extra {
                    cs.push_str(e);
                }
                cs
            }
            Driver::SQLITE => {
                let mut cs = format!("sqlite://{user}:{pass}@{host}:{port}/{db}");
                if let Some(ref e) = extra {
                    cs.push_str(e);
                }
                cs
            }
        }
    }
}
