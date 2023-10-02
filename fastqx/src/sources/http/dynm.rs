//! file: dynm.rs
//! author: Jacob Xie
//! date: 2023/10/02 18:55:00 Monday
//! brief:

use anyhow::Result;
use pyo3::prelude::*;
use ref_cast::RefCast;
use serde_json::Value;

use crate::sources::http::*;

// ================================================================================================
// Value wrapper
// ================================================================================================

#[pyclass]
#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct JSON(pub(crate) Value);

impl HttpConnector {
    pub async fn dyn_get(&self, subpath: &str) -> Result<JSON> {
        self.raw_get(subpath).await.map(JSON)
    }

    pub async fn dyn_post(&self, subpath: &str, req: &JSON) -> Result<JSON> {
        self.raw_post(subpath, &req.0).await.map(JSON)
    }
}
