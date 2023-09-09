//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/09 14:56:36 Saturday
//! brief:

pub mod conn;

pub use anyhow;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn fastqx(_py: Python, _m: &PyModule) -> PyResult<()> {
    // TODO

    Ok(())
}
