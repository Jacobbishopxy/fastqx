//! file: adt.rs
//! author: Jacob Xie
//! date: 2023/10/02 23:22:11 Monday
//! brief:

use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::adt::FqxData;
use crate::sources::http::HttpConnector;

// ================================================================================================
// HttpMethod
// ================================================================================================

#[pyclass]
#[pyo3(name = "FqxHttpMethod")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::Get
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[pymethods]
impl HttpMethod {
    fn __repr__(&self) -> &'static str {
        match self {
            HttpMethod::Get => "HttpMethod::Get",
            HttpMethod::Post => "HttpMethod::Post",
            HttpMethod::Put => "HttpMethod::Put",
            HttpMethod::Delete => "HttpMethod::Delete",
            HttpMethod::Patch => "HttpMethod::Patch",
        }
    }
}

// ================================================================================================
// Impl FqxData
// ================================================================================================

impl FqxData {
    pub async fn curl<P>(
        client: &HttpConnector,
        subpath: &str,
        mtd: &HttpMethod,
        payload: Option<P>,
    ) -> Result<Self>
    where
        P: Serialize,
    {
        match mtd {
            HttpMethod::Get => client.get(subpath).await,
            HttpMethod::Post => {
                if let Some(p) = payload {
                    client.post(subpath, &p).await
                } else {
                    Err(anyhow!("method POST payload is empty"))
                }
            }
            HttpMethod::Put => {
                if let Some(p) = payload {
                    client.put(subpath, &p).await
                } else {
                    Err(anyhow!("method PUT payload is empty"))
                }
            }
            HttpMethod::Patch => {
                if let Some(p) = payload {
                    client.patch(subpath, &p).await
                } else {
                    Err(anyhow!("method PATCH payload is empty"))
                }
            }
            _ => Err(anyhow!("only accept GET/POST/PUT/PATCH")),
        }
    }
}
