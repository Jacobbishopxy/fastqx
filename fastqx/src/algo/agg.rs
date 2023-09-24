//! file: agg.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:21:51 Sunday
//! brief:

use anyhow::Result;

use crate::adt::{FqxData, FqxRow, FqxValue};

// ================================================================================================
// AlgoAgg
// ================================================================================================

pub trait AlgoAgg {
    fn sum(&self) -> FqxRow;

    fn min(&self) -> FqxRow;

    fn max(&self) -> FqxRow;

    fn mean(&self) -> FqxRow;
}

// ================================================================================================
// Impl
// ================================================================================================

impl AlgoAgg for FqxData {
    fn sum(&self) -> FqxRow {
        todo!()
    }

    fn min(&self) -> FqxRow {
        todo!()
    }

    fn max(&self) -> FqxRow {
        todo!()
    }

    fn mean(&self) -> FqxRow {
        todo!()
    }
}
