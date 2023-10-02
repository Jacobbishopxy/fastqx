//! file: dynm.rs
//! author: Jacob Xie
//! date: 2023/10/02 18:55:00 Monday
//! brief:

use anyhow::Result;
use serde_json::Value;

use crate::sources::http::*;

// ================================================================================================
// Value wrapper
// ================================================================================================

impl HttpConnector {
    pub async fn dyn_get(&self, subpath: &str) -> Result<Value> {
        self.raw_get(subpath).await
    }

    pub async fn dyn_post(&self, subpath: &str, req: &Value) -> Result<Value> {
        self.raw_post(subpath, req).await
    }
}
