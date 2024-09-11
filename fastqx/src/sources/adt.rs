//! file: adt.rs
//! author: Jacob Xie
//! date: 2023/10/19 09:50:54 Thursday
//! brief:

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ================================================================================================
// SaveMode
// ================================================================================================

#[pyclass(eq, eq_int)]
#[pyo3(name = "FqxSaveMode")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SaveMode {
    Override,
    Append,
}
