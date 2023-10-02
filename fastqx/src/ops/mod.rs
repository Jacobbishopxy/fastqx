//! file: mod.rs
//! author: Jacob Xie
//! date: 2023/09/22 20:05:37 Friday
//! brief:

pub mod agg;
pub mod apply;
pub mod compare;
pub mod fold;
pub mod group;
pub mod reduce;
pub mod select;
pub mod slice;
mod utils;

pub use agg::*;
pub use apply::*;
pub use compare::*;
pub use fold::*;
pub use group::*;
pub use reduce::*;
pub use select::*;
pub use slice::*;
