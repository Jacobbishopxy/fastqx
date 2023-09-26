//! file: agg.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:21:51 Sunday
//! brief:

use std::cmp::Ordering;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::op::{FqxRowSelect, FqxSlice, OpReduce, OpReduceFqxRow};

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
        self.reduce_fqx_row(|p, c| {
            if let Some(Ordering::Less) = p.partial_cmp(&c) {
                c
            } else {
                p
            }
        })
    }

    fn max(self) -> Option<FqxRow> {
        self.reduce_fqx_row(|p, c| {
            if let Some(Ordering::Greater) = p.partial_cmp(&c) {
                c
            } else {
                p
            }
        })
    }

    fn mean(self) -> Option<FqxRow> {
        let len = self.height();
        self.sum().map(|r| {
            let inner =
                r.0.into_iter()
                    .map(|e| e / FqxValue::U64(len as u64))
                    .collect::<Vec<_>>();
            FqxRow(inner)
        })
    }
}

impl<'a> OpAgg for &'a FqxData {
    fn sum(self) -> Option<FqxRow> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Option<FqxRow> {
        self.reduce_fqx_row(|p, c| {
            if let Some(Ordering::Less) = p.partial_cmp(&c) {
                c
            } else {
                p
            }
        })
    }

    fn max(self) -> Option<FqxRow> {
        self.reduce_fqx_row(|p, c| {
            if let Some(Ordering::Greater) = p.partial_cmp(&c) {
                c
            } else {
                p
            }
        })
    }

    fn mean(self) -> Option<FqxRow> {
        let len = self.height();
        self.sum().map(|r| {
            let inner =
                r.0.into_iter()
                    .map(|e| e / FqxValue::U64(len as u64))
                    .collect::<Vec<_>>();
            FqxRow(inner)
        })
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSlice

impl<'a> OpAgg for &'a FqxSlice {
    fn sum(self) -> Option<FqxRow> {
        self.reduce(|p, c| p + c)
    }

    fn min(self) -> Option<FqxRow> {
        self.reduce_fqx_row(|p, c| {
            if let Some(Ordering::Less) = p.partial_cmp(&c) {
                c
            } else {
                p
            }
        })
    }

    fn max(self) -> Option<FqxRow> {
        self.reduce_fqx_row(|p, c| {
            if let Some(Ordering::Greater) = p.partial_cmp(&c) {
                c
            } else {
                p
            }
        })
    }

    fn mean(self) -> Option<FqxRow> {
        let len = self.0.len();
        self.sum().map(|r| {
            let inner =
                r.0.into_iter()
                    .map(|e| e / FqxValue::U64(len as u64))
                    .collect::<Vec<_>>();
            FqxRow(inner)
        })
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSelect

impl OpAgg for Vec<FqxRowSelect<FqxValue>> {
    fn sum(self) -> Option<FqxRow> {
        todo!()
    }

    fn min(self) -> Option<FqxRow> {
        todo!()
    }

    fn max(self) -> Option<FqxRow> {
        todo!()
    }

    fn mean(self) -> Option<FqxRow> {
        todo!()
    }
}
