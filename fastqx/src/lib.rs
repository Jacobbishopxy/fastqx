//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/10 12:33:47 Sunday
//! brief:

pub mod adt;
mod constant;
#[cfg(test)]
pub(crate) mod mock;
pub mod ops;
pub mod sources;
pub mod utils;

#[doc(inline)]
pub use fastqx_macros;

// re-export
#[doc(hidden)]
pub use anyhow;
#[doc(hidden)]
pub use sea_query;
#[doc(hidden)]
pub use serde;
#[doc(hidden)]
pub use serde_json;
#[doc(hidden)]
pub use sqlx;
#[doc(hidden)]
pub use tiberius;

// prelude
pub mod prelude {
    pub use super::fastqx_macros::*;

    pub use super::adt::*;
    pub use super::ops::*;
    pub use super::sources::adt::*;
    pub use super::sources::csv::read::*;
    pub use super::sources::csv::write::*;
    pub use super::sources::http::adt::*;
    pub use super::sources::http::conn::*;
    pub use super::sources::http::dynm::*;
    pub use super::sources::sql::adt::*;
    pub use super::sources::sql::conn::*;
    pub use super::sources::sql::sqx::*;
    pub use super::sources::sql::tbr::*;
}
