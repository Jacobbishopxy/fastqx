//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/10 12:33:47 Sunday
//! brief:

pub mod conn;

#[doc(inline)]
pub use fastqx_macros;

// re-export
#[doc(hidden)]
pub use anyhow;
#[doc(hidden)]
pub use sea_query;

// prelude
pub mod prelude {
    pub use super::conn::db::ConnectorStatement;
    pub use super::fastqx_macros::*;
    pub use super::sea_query::*;
}
