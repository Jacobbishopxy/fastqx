//! file: r.rs
//! author: Jacob Xie
//! date: 2023/12/06 16:37:18 Wednesday
//! brief:

use crate::adt::{FqxValue, FqxValueType};

// ================================================================================================
// RowProps
// ================================================================================================

pub trait RowProps: Clone {
    fn get_nth(&self, idx: usize) -> Option<&FqxValue>;

    fn len(&self) -> usize;

    fn types(&self) -> Vec<FqxValueType>;

    fn to_values(self) -> Vec<FqxValue>;
}
