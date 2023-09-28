//! file: agg.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:21:51 Sunday
//! brief:

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::op::utils::*;
use crate::op::{FqxRowSelect, FqxSlice, OpFoldFqxRow, OpReduce, OpReduceFqxRow};

// ================================================================================================
// OpAgg
// ================================================================================================

pub trait OpAgg {
    fn sum(self) -> Option<FqxRow>;

    fn min(self) -> Option<FqxRow>;

    fn max(self) -> Option<FqxRow>;

    fn mean(self) -> Option<FqxRow>;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxData

impl OpAgg for FqxData {
    fn sum(self) -> Option<FqxRow> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Option<FqxRow> {
        self.reduce_fqx_row(get_min)
    }

    fn max(self) -> Option<FqxRow> {
        self.reduce_fqx_row(get_max)
    }

    fn mean(self) -> Option<FqxRow> {
        let len = self.height();
        self.sum().map(|r| calc_mean(r, len))
    }
}

impl<'a> OpAgg for &'a FqxData {
    fn sum(self) -> Option<FqxRow> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Option<FqxRow> {
        self.reduce_fqx_row(get_min)
    }

    fn max(self) -> Option<FqxRow> {
        self.reduce_fqx_row(get_max)
    }

    fn mean(self) -> Option<FqxRow> {
        let len = self.height();
        self.sum().map(|r| calc_mean(r, len))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSlice

impl<'a> OpAgg for &'a FqxSlice {
    fn sum(self) -> Option<FqxRow> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Option<FqxRow> {
        self.reduce_fqx_row(get_min)
    }

    fn max(self) -> Option<FqxRow> {
        self.reduce_fqx_row(get_max)
    }

    fn mean(self) -> Option<FqxRow> {
        let len = self.0.len();
        self.sum().map(|r| calc_mean(r, len))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSelect

impl OpAgg for Vec<FqxRowSelect<FqxValue>> {
    fn sum(self) -> Option<FqxRow> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|ini| iter.fold(FqxRow::from(ini), |acc, c| acc + FqxRow::from(c)))
    }

    fn min(mut self) -> Option<FqxRow> {
        self.pop()
            .map(|fst| self.fold_fqx_row(FqxRow::from(fst), get_min))
    }

    fn max(mut self) -> Option<FqxRow> {
        self.pop()
            .map(|fst| self.fold_fqx_row(FqxRow::from(fst), get_max))
    }

    fn mean(self) -> Option<FqxRow> {
        let len = self.len();
        self.sum().map(|r| calc_mean(r, len))
    }
}

impl<'a> OpAgg for Vec<FqxRowSelect<&'a FqxValue>> {
    fn sum(self) -> Option<FqxRow> {
        let mut iter = self.into_iter();
        iter.next()
            .map(|ini| iter.fold(FqxRow::from(ini), |acc, c| acc + FqxRow::from(c)))
    }

    fn min(mut self) -> Option<FqxRow> {
        self.pop()
            .map(|fst| self.fold_fqx_row(FqxRow::from(fst), get_min))
    }

    fn max(mut self) -> Option<FqxRow> {
        self.pop()
            .map(|fst| self.fold_fqx_row(FqxRow::from(fst), get_max))
    }

    fn mean(self) -> Option<FqxRow> {
        let len = self.len();
        self.sum().map(|r| calc_mean(r, len))
    }
}
