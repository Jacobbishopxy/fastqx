//! file: d.rs
//! author: Jacob Xie
//! date: 2023/10/16 13:21:56 Monday
//! brief:

use std::{
    marker::PhantomData,
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

// ================================================================================================
// Abbr ranges
// ================================================================================================

pub(crate) type S = usize;
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
    C: Clone,
    T: Clone,
    I: Default + Clone,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    fn columns(&self) -> &[C];

    fn types(&self) -> &[T];

    fn data(&self) -> &[I];

    fn dcst(self) -> (Vec<C>, Vec<T>, Vec<I>);

    fn cst(columns: Vec<C>, types: Vec<T>, data: Vec<I>) -> Self;

    // ================================================================================================
    // default implement
    // ================================================================================================

    fn row_wise_empty(self) -> Self {
        let (c, t, _) = self.dcst();
        FqxD::cst(c, t, vec![])
    }

    fn row_wise_s(self, idx: S) -> Self {
        let (c, t, d) = self.dcst();
        let d = d.into_iter().nth(idx).map_or(vec![], |r| vec![r]);
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
