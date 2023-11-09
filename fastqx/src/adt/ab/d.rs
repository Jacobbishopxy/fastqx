//! file: d.rs
//! author: Jacob Xie
//! date: 2023/10/16 13:21:56 Monday
//! brief:

use std::marker::PhantomData;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use anyhow::{bail, Result};

// ================================================================================================
// Abbr ranges
// ================================================================================================

pub(crate) type S = usize;
pub(crate) type VS = Vec<usize>;
pub(crate) type VST = Vec<String>;
pub(crate) type F = RangeFull;
pub(crate) type R = Range<usize>;
pub(crate) type RF = RangeFrom<usize>;
pub(crate) type RI = RangeInclusive<usize>;
pub(crate) type RT = RangeTo<usize>;
pub(crate) type RTI = RangeToInclusive<usize>;

// ================================================================================================
// FqxD
// ================================================================================================

pub trait FqxD<C, T, I, E>
where
    Self: Sized,
    I: Default + Clone,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    fn columns(&self) -> &Vec<C>;

    fn columns_mut(&mut self) -> &mut Vec<C>;

    fn types(&self) -> &Vec<T>;

    fn types_mut(&mut self) -> &mut Vec<T>;

    fn data(&self) -> &Vec<I>;

    fn data_mut(&mut self) -> &mut Vec<I>;

    fn dcst(self) -> (Vec<C>, Vec<T>, Vec<I>);

    fn cst(columns: Vec<C>, types: Vec<T>, data: Vec<I>) -> Self;

    fn check_row_validation(&self, row: &I) -> bool;

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

    fn push(&mut self, row: I) -> Result<()> {
        todo!()
    }

    fn extend(&mut self, rows: Vec<I>) -> Result<()> {
        todo!()
    }

    fn insert(&mut self, idx: usize, row: I) -> Result<()> {
        todo!()
    }

    fn pop(&mut self) -> Option<I> {
        todo!()
    }

    fn remove(&mut self, idx: usize) -> Result<I> {
        todo!()
    }

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&I) -> bool,
    {
        todo!()
    }

    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut I) -> bool,
    {
        todo!()
    }

    fn reverse(&mut self) {
        todo!()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    fn set_columns<IC>(&mut self, columns: Vec<IC>) -> Result<()>
    where
        IC: Into<C>,
    {
        if self.columns().len() != columns.len() {
            bail!("length mismatch")
        }

        *self.columns_mut() = columns.into_iter().map(|e| e.into()).collect();
        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // row_wise taken

    fn row_wise_empty(self) -> Self {
        let (c, t, _) = self.dcst();
        FqxD::cst(c, t, vec![])
    }

    fn row_wise_s(self, idx: S) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.into_iter().nth(idx).map_or(vec![], |r| vec![r]);
        FqxD::cst(c, t, d)
    }

    fn row_wise_vs(self, idx: VS) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.into_iter().enumerate().fold(vec![], |mut acc, (i, e)| {
            if idx.contains(&i) {
                acc.push(e);
            }
            acc
        });
        FqxD::cst(c, t, d)
    }

    fn row_wise_f(self, _idx: F) -> Self {
        self
    }

    fn row_wise_r(self, idx: R) -> Self {
        let (c, t, d) = self.dcst();
        let d = d
            .into_iter()
            .skip(idx.start)
            .take(idx.end - idx.start)
            .collect();
        FqxD::cst(c, t, d)
    }

    fn row_wise_rf(self, idx: RF) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.into_iter().skip(idx.start).collect();
        FqxD::cst(c, t, d)
    }

    fn row_wise_ri(self, idx: RI) -> Self {
        let (c, t, d) = self.dcst();
        let d = d
            .into_iter()
            .skip(*idx.start())
            .take(*idx.end() - *idx.start() + 1)
            .collect();
        FqxD::cst(c, t, d)
    }

    fn row_wise_rt(self, idx: RT) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.into_iter().take(idx.end).collect();
        FqxD::cst(c, t, d)
    }

    fn row_wise_rti(self, idx: RTI) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.into_iter().take(idx.end + 1).collect();
        FqxD::cst(c, t, d)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // col_wise taken

    fn col_wise_empty(self) -> Self {
        let (_, _, d) = self.dcst();
        let d = vec![I::default(); d.into_iter().count()];
        FqxD::cst(vec![], vec![], d)
    }

    fn col_wise_s(self, idx: S) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.into_iter().nth(idx).map_or(vec![], |e| vec![e]);
        let t = t.into_iter().nth(idx).map_or(vec![], |e| vec![e]);
        let d = d
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .nth(idx)
                    .map_or(I::default(), |r| I::from_iter(vec![r]))
            })
            .collect();
        FqxD::cst(c, t, d)
    }

    fn col_wise_vs(self, idx: VS) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.into_iter().enumerate().fold(vec![], |mut acc, (i, e)| {
            if idx.contains(&i) {
                acc.push(e);
            }
            acc
        });
        let t = t.into_iter().enumerate().fold(vec![], |mut acc, (i, e)| {
            if idx.contains(&i) {
                acc.push(e);
            }
            acc
        });
        let d = d
            .into_iter()
            .map(|r| {
                let r = r.into_iter().enumerate().fold(vec![], |mut acc, (i, e)| {
                    if idx.contains(&i) {
                        acc.push(e);
                    }
                    acc
                });
                I::from_iter(r)
            })
            .collect();
        FqxD::cst(c, t, d)
    }

    fn col_wise_f(self, _idx: F) -> Self {
        self
    }

    fn col_wise_r(self, idx: R) -> Self {
        let (c, t, d) = self.dcst();
        let c = c
            .into_iter()
            .skip(idx.start)
            .take(idx.end - idx.start)
            .collect();
        let t = t
            .into_iter()
            .skip(idx.start)
            .take(idx.end - idx.start)
            .collect();
        let d = d
            .into_iter()
            .map(|r| {
                let r = r.into_iter().skip(idx.start).take(idx.end - idx.start);
                I::from_iter(r)
            })
            .collect();
        FqxD::cst(c, t, d)
    }

    fn col_wise_rf(self, idx: RF) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.into_iter().skip(idx.start).collect();
        let t = t.into_iter().skip(idx.start).collect();
        let d = d
            .into_iter()
            .map(|r| r.into_iter().skip(idx.start).collect())
            .collect();
        FqxD::cst(c, t, d)
    }

    fn col_wise_ri(self, idx: RI) -> Self {
        let (c, t, d) = self.dcst();
        let c = c
            .into_iter()
            .skip(*idx.start())
            .take(*idx.end() - *idx.start() + 1)
            .collect();
        let t = t
            .into_iter()
            .skip(*idx.start())
            .take(*idx.end() - *idx.start() + 1)
            .collect();
        let d = d
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .skip(*idx.start())
                    .take(*idx.end() - *idx.start() + 1)
                    .collect()
            })
            .collect();
        FqxD::cst(c, t, d)
    }

    fn col_wise_rt(self, idx: RT) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.into_iter().take(idx.end).collect();
        let t = t.into_iter().take(idx.end).collect();
        let d = d
            .into_iter()
            .map(|r| r.into_iter().take(idx.end).collect())
            .collect();
        FqxD::cst(c, t, d)
    }

    fn col_wise_rti(self, idx: RTI) -> Self {
        let (c, t, d) = self.dcst();
        let c = c.into_iter().take(idx.end + 1).collect();
        let t = t.into_iter().take(idx.end + 1).collect();
        let d = d
            .into_iter()
            .map(|r| r.into_iter().take(idx.end + 1).collect())
            .collect();
        FqxD::cst(c, t, d)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    fn columns_position(&self, cols: Vec<C>) -> Vec<usize>
    where
        C: PartialEq,
    {
        self.columns()
            .into_iter()
            .enumerate()
            .fold(vec![], |mut acc, (i, e)| {
                if cols.contains(&e) {
                    acc.push(i);
                }
                acc
            })
    }

    fn types_position(&self, typs: Vec<T>) -> Vec<usize>
    where
        T: PartialEq,
    {
        self.types()
            .into_iter()
            .enumerate()
            .fold(vec![], |mut acc, (i, e)| {
                if typs.contains(&e) {
                    acc.push(i);
                }
                acc
            })
    }
}

// ================================================================================================
// PhantomU
// ================================================================================================

pub(crate) struct PhantomU<C, T, I, E> {
    _c: PhantomData<C>,
    _t: PhantomData<T>,
    _i: PhantomData<I>,
    _e: PhantomData<E>,
}
