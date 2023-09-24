//! file: agg.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:21:51 Sunday
//! brief:

use crate::adt::FqxRow;

// ================================================================================================
// AlgoAgg
// ================================================================================================

pub trait AlgoAgg {
    fn sum(&self) -> FqxRow;

    fn min(&self) -> FqxRow;

    fn max(&self) -> FqxRow;

    fn mean(&self) -> FqxRow;
}