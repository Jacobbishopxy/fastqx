//! file: r.rs
//! author: Jacob Xie
//! date: 2023/12/06 16:37:18 Wednesday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxValue, FqxValueType};

// ================================================================================================
// RowProps
// ================================================================================================

pub trait RowProps: Clone + Extend<FqxValue> {
    fn nulls(len: usize) -> Self;

    fn get(&self, idx: usize) -> Option<&FqxValue>;

    fn get_mut(&mut self, idx: usize) -> Option<&mut FqxValue>;

    fn len(&self) -> usize;

    fn types(&self) -> Vec<FqxValueType>;

    fn to_values(self) -> Vec<FqxValue>;

    fn from_values(d: Vec<FqxValue>) -> Self;

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    fn iter_owned(self) -> std::vec::IntoIter<FqxValue>;

    fn iter(&self) -> std::slice::Iter<'_, FqxValue>;

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, FqxValue>;

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    fn add(self, rhs: Self) -> Self;

    fn sub(self, rhs: Self) -> Self;

    fn mul(self, rhs: Self) -> Self;

    fn div(self, rhs: Self) -> Self;

    fn rem(self, rhs: Self) -> Self;

    // ================================================================================================
    // default impl
    // ================================================================================================

    fn select(&self, idx: &[usize]) -> Vec<&FqxValue> {
        idx.into_iter().fold(vec![], |mut acc, i| {
            if let Some(e) = self.get(*i) {
                acc.push(e);
            }
            acc
        })
    }

    fn select_owned(&self, idx: &[usize]) -> Vec<FqxValue> {
        idx.into_iter().fold(vec![], |mut acc, i| {
            if let Some(e) = self.get(*i) {
                acc.push(e.clone());
            }
            acc
        })
    }

    fn select_mut(&mut self, d: HashMap<usize, FqxValue>) {
        let len = self.len();
        let typs = self.types();
        for (k, v) in d.into_iter() {
            if k <= len && typs[k] == FqxValueType::from(&v) {
                self.get_mut(k).map(|e| *e = v);
            }
        }
    }
}
