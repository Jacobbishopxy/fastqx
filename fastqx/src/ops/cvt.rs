//! file: convt.rs
//! author: Jacob Xie
//! date: 2023/12/12 20:46:48 Tuesday
//! brief:

use crate::adt::{FqxData, FqxDataCow};

// ================================================================================================
// OpCvt
// ================================================================================================

pub trait OpCvt<T> {
    fn convert(self) -> T;
}

// ================================================================================================
// Impl
// ================================================================================================

impl OpCvt<FqxData> for FqxData {
    fn convert(self) -> FqxData {
        self
    }
}

impl<'a> OpCvt<FqxDataCow<'a>> for FqxData {
    fn convert(self) -> FqxDataCow<'a> {
        FqxDataCow::from(self)
    }
}

impl<'a> OpCvt<FqxDataCow<'a>> for FqxDataCow<'a> {
    fn convert(self) -> FqxDataCow<'a> {
        self
    }
}

impl<'a> OpCvt<FqxData> for FqxDataCow<'a> {
    fn convert(self) -> FqxData {
        FqxData::from(self)
    }
}
