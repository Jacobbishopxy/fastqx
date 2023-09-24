//! file: fold.rs
//! author: Jacob Xie
//! date: 2023/09/24 18:50:53 Sunday
//! brief:

use itertools::Itertools;

use crate::adt::{FqxData, FqxRow};

// ================================================================================================
// AlgoFold
// ================================================================================================

pub trait AlgoFold {
    type IterItem<'a>;

    fn fold<'a, A, F>(&'a self, accumulator: A, f: F) -> A
    where
        F: FnMut(A, Self::IterItem<'a>) -> A;
}

// ================================================================================================
// Impl
// ================================================================================================

impl AlgoFold for FqxData {
    type IterItem<'a> = &'a FqxRow;

    fn fold<'a, A, F>(&'a self, accumulator: A, f: F) -> A
    where
        F: FnMut(A, Self::IterItem<'a>) -> A,
    {
        self.iter_ref().fold(accumulator, f)
    }
}
