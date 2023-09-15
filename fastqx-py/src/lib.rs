//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/09 14:56:36 Saturday
//! brief:

pub mod csv;
pub mod data;
pub mod py;
pub mod sql;

pub(crate) use fastqx::prelude::*;
pub use py::*;
