//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/10 12:33:47 Sunday
//! brief:

pub mod adt;
pub mod csv;
pub mod sql;
pub mod utils;

#[doc(inline)]
pub use fastqx_macros;

// re-export
#[doc(hidden)]
pub use anyhow;
#[doc(hidden)]
pub use sea_query;
#[doc(hidden)]
pub use sqlx;

// prelude
pub mod prelude {
    pub use super::adt::*;
    pub use super::csv::*;
    pub use super::fastqx_macros::*;
    pub use super::sql::*;
}
