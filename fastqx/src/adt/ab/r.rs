//! file: r.rs
//! author: Jacob Xie
//! date: 2023/12/06 16:37:18 Wednesday
//! brief:

use std::borrow::Cow;

use crate::adt::{FqxRow, FqxValue};

// ================================================================================================
// RowProps
// ================================================================================================

pub trait RowProps {
    fn get_nth(&self, idx: usize) -> Option<&FqxValue>;
}

impl RowProps for FqxRow {
    fn get_nth(&self, idx: usize) -> Option<&FqxValue> {
        self.0.get(idx)
    }
}

impl<'a> RowProps for Cow<'a, [FqxValue]> {
    fn get_nth(&self, idx: usize) -> Option<&FqxValue> {
        match self {
            Cow::Borrowed(b) => b.get(idx),
            Cow::Owned(o) => o.get(idx),
        }
    }
}
