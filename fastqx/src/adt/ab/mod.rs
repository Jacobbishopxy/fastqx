//! file: mod.rs
//! author: Jacob Xie
//! date: 2023/09/23 22:22:59 Saturday
//! brief:

pub mod arith;
pub mod cvt;
pub mod d;
pub mod iter;

pub(crate) use d::PhantomU;
pub use d::{FqxD, FqxR, FqxSlice, FromTo};
