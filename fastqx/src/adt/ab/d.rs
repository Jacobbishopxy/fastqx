//! file: d.rs
//! author: Jacob Xie
//! date: 2023/10/16 13:21:56 Monday
//! brief:

use std::collections::HashSet;

use anyhow::{bail, Result};

use crate::adt::ab::s::{F, R, RF, RI, RT, RTI, S, VS};
use crate::adt::{FqxValueType, RowProps, SeqAppend, SeqSlice};

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! guard {
    ($s:expr, $r:expr) => {
        if !$s.check_row_validation(&$r) {
            bail!("row mismatch")
        }
    };
}

// ================================================================================================
// FqxR
// ================================================================================================

pub trait FqxD: Sized {
    type ColumnsT: SeqSlice + SeqAppend<String> + Clone;
    type TypesT: SeqSlice + SeqAppend<FqxValueType> + Clone;
    type RowT: SeqSlice + RowProps;

    fn cst(c: Self::ColumnsT, t: Self::TypesT, d: Vec<Self::RowT>) -> Self;

    fn dcst(self) -> (Self::ColumnsT, Self::TypesT, Vec<Self::RowT>);

    fn columns(&self) -> &[String];

    fn columns_mut(&mut self) -> &mut [String];

    fn set_columns(&mut self, cols: Self::ColumnsT) -> Result<()>;

    fn columns_take(self) -> Self::ColumnsT;

    fn types(&self) -> &[FqxValueType];

    fn types_mut(&mut self) -> &mut [FqxValueType];

    fn set_types(&mut self, types: Self::TypesT) -> Result<()>;

    fn types_take(self) -> Self::TypesT;

    fn data(&self) -> &[Self::RowT];

    fn data_mut(&mut self) -> &mut Vec<Self::RowT>;

    fn set_data(&mut self, data: Vec<Self::RowT>) -> Result<()>;

    fn data_take(self) -> Vec<Self::RowT>;

    fn check_row_validation(&self, row: &Self::RowT) -> bool;

    fn iter_owned(self) -> std::vec::IntoIter<Self::RowT>;

    fn iter(&self) -> std::slice::Iter<'_, Self::RowT>;

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Self::RowT>;

    // ================================================================================================
    // default implement
    // ================================================================================================

    fn height(&self) -> usize {
        self.data().len()
    }

    fn width(&self) -> usize {
        self.columns().len()
    }

    fn shape(&self) -> (usize, usize) {
        (self.height(), self.width())
    }

    fn push(&mut self, row: Self::RowT) -> Result<()> {
        guard!(self, row);

        self.data_mut().push(row);

        Ok(())
    }

    fn extend(&mut self, rows: Vec<Self::RowT>) -> Result<()> {
        for row in rows.iter() {
            guard!(self, row);
        }

        self.data_mut().extend(rows);

        Ok(())
    }

    fn insert(&mut self, idx: usize, row: Self::RowT) -> Result<()> {
        guard!(self, row);

        if idx > self.height() {
            self.push(row)?;
            return Ok(());
        }

        self.data_mut().insert(idx, row);

        Ok(())
    }

    fn pop(&mut self) -> Option<Self::RowT> {
        self.data_mut().pop()
    }

    fn remove(&mut self, idx: usize) -> Option<Self::RowT> {
        if idx > self.height() {
            return None;
        }

        Some(self.data_mut().remove(idx))
    }

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Self::RowT) -> bool,
    {
        self.data_mut().retain(f)
    }

    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut Self::RowT) -> bool,
    {
        self.data_mut().retain_mut(f)
    }

    fn reverse(&mut self) {
        self.data_mut().reverse()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // row_wise taken

    fn row_wise_empty(self) -> Self {
        let (c, t, _) = self.dcst();
        Self::cst(c, t, vec![])
    }

    fn row_wise_s(self, idx: S) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.takes(vec![idx]);
        Self::cst(c, t, d)
    }

    fn row_wise_vs(self, idx: VS) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.takes(idx);
        Self::cst(c, t, d)
    }

    fn row_wise_f(self, _idx: F) -> Self {
        self
    }

    fn row_wise_r(self, idx: R) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.sliced(idx);
        Self::cst(c, t, d)
    }

    fn row_wise_rf(self, idx: RF) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.sliced(idx);
        Self::cst(c, t, d)
    }

    fn row_wise_ri(self, idx: RI) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.sliced(idx);
        Self::cst(c, t, d)
    }

    fn row_wise_rt(self, idx: RT) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.sliced(idx);
        Self::cst(c, t, d)
    }

    fn row_wise_rti(self, idx: RTI) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.sliced(idx);
        Self::cst(c, t, d)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // col_wise taken

    fn col_wise_empty(self) -> Self {
        let (_, _, d) = self.dcst();
        let d = vec![Self::RowT::empty(); d.into_iter().count()];
        Self::cst(Self::ColumnsT::empty(), Self::TypesT::empty(), d)
    }

    fn col_wise_s(self, idx: S) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.takes(vec![idx]);
        let t = t.takes(vec![idx]);
        let d = d.into_iter().map(|r| r.takes(vec![idx])).collect();
        Self::cst(c, t, d)
    }

    fn col_wise_vs(self, idx: VS) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.takes(idx.clone());
        let t = t.takes(idx.clone());
        let d = d.into_iter().map(|r| r.takes(idx.clone())).collect();
        Self::cst(c, t, d)
    }

    fn col_wise_f(self, _idx: F) -> Self {
        self
    }

    fn col_wise_r(self, idx: R) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.sliced(idx.clone());
        let t = t.sliced(idx.clone());
        let d = d.into_iter().map(|r| r.sliced(idx.clone())).collect();
        Self::cst(c, t, d)
    }

    fn col_wise_rf(self, idx: RF) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.sliced(idx.clone());
        let t = t.sliced(idx.clone());
        let d = d.into_iter().map(|r| r.sliced(idx.clone())).collect();
        Self::cst(c, t, d)
    }

    fn col_wise_ri(self, idx: RI) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.sliced(idx.clone());
        let t = t.sliced(idx.clone());
        let d = d.into_iter().map(|r| r.sliced(idx.clone())).collect();
        Self::cst(c, t, d)
    }

    fn col_wise_rt(self, idx: RT) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.sliced(idx);
        let t = t.sliced(idx);
        let d = d.into_iter().map(|r| r.sliced(idx)).collect();
        Self::cst(c, t, d)
    }

    fn col_wise_rti(self, idx: RTI) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.sliced(idx);
        let t = t.sliced(idx);
        let d = d.into_iter().map(|r| r.sliced(idx)).collect();
        Self::cst(c, t, d)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    fn columns_position<I, S>(&self, cols: &I) -> Vec<usize>
    where
        for<'a> &'a I: IntoIterator<Item = &'a S>,
        S: AsRef<str>,
    {
        let cols = cols.into_iter().map(|e| e.as_ref()).collect::<HashSet<_>>();
        self.columns()
            .into_iter()
            .enumerate()
            .fold(vec![], |mut acc, (i, e)| {
                if cols.contains(e.as_str()) {
                    acc.push(i);
                }
                acc
            })
    }

    fn empty_row(&self) -> Self::RowT {
        Self::RowT::empty()
    }
}
