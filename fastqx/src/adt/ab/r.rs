//! file: r.rs
//! author: Jacob Xie
//! date: 2023/12/06 16:37:18 Wednesday
//! brief:

use std::ops::{Add, Div, Mul, Rem};
use std::{collections::HashMap, ops::Sub};

use crate::adt::{FqxValue, FqxValueType};

// ================================================================================================
// RowProps
// ================================================================================================

pub trait RowProps
where
    Self: Clone + Extend<FqxValue> + Default,
    Self: Add<Output = Self>,
    Self: Sub<Output = Self>,
    Self: Mul<Output = Self>,
    Self: Div<Output = Self>,
    Self: Rem<Output = Self>,
    Self: FromIterator<FqxValue>,
{
    fn nulls(len: usize) -> Self;

    fn get(&self, idx: usize) -> Option<&FqxValue>;

    fn get_mut(&mut self, idx: usize) -> Option<&mut FqxValue>;

    fn len(&self) -> usize;

    fn types(&self) -> Vec<FqxValueType>;

    fn values(&self) -> &[FqxValue];

    fn to_values(self) -> Vec<FqxValue>;

    fn from_values(d: Vec<FqxValue>) -> Self;

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    fn iter_owned(self) -> std::vec::IntoIter<FqxValue>;

    fn iter(&self) -> std::slice::Iter<'_, FqxValue>;

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, FqxValue>;

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    fn min(&self, rhs: &Self) -> Self;

    fn max(&self, rhs: &Self) -> Self;

    // ================================================================================================
    // default impl
    // ================================================================================================

    fn select_vals(&self, idx: &[usize]) -> Vec<&FqxValue> {
        idx.into_iter().fold(vec![], |mut acc, i| {
            if let Some(e) = self.get(*i) {
                acc.push(e);
            }
            acc
        })
    }

    fn select_vals_owned(&self, idx: &[usize]) -> Vec<FqxValue> {
        idx.into_iter().fold(vec![], |mut acc, i| {
            if let Some(e) = self.get(*i) {
                acc.push(e.clone());
            }
            acc
        })
    }

    fn select_vals_mut(&mut self, d: HashMap<usize, FqxValue>) {
        let len = self.len();
        let typs = self.types();
        for (k, v) in d.into_iter() {
            if k <= len && typs[k] == FqxValueType::from(&v) {
                self.get_mut(k).map(|e| *e = v);
            }
        }
    }

    fn select(&self, idx: &[usize]) -> Self {
        let v = self.select_vals_owned(idx);
        Self::from_values(v)
    }
}
