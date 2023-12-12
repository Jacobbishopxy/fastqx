//! file: s.rs
//! author: Jacob Xie
//! date: 2023/12/08 15:10:39 Friday
//! brief:

use std::{
    borrow::Cow,
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use crate::adt::util::{slice_vec, takes_vec};

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
// FromTo
// ================================================================================================

pub trait FromTo {
    fn from_to(&self, max_len: usize) -> (usize, usize);
}

impl FromTo for RangeFull {
    fn from_to(&self, max_len: usize) -> (usize, usize) {
        (0, max_len)
    }
}

impl FromTo for Range<usize> {
    fn from_to(&self, max_len: usize) -> (usize, usize) {
        (self.start, (self.end - 1).min(max_len))
    }
}

impl FromTo for RangeFrom<usize> {
    fn from_to(&self, max_len: usize) -> (usize, usize) {
        (self.start, max_len)
    }
}

impl FromTo for RangeInclusive<usize> {
    fn from_to(&self, max_len: usize) -> (usize, usize) {
        (*self.start(), *self.end().min(&max_len))
    }
}

impl FromTo for RangeTo<usize> {
    fn from_to(&self, max_len: usize) -> (usize, usize) {
        (0, (self.end - 1).min(max_len))
    }
}

impl FromTo for RangeToInclusive<usize> {
    fn from_to(&self, max_len: usize) -> (usize, usize) {
        (0, self.end.min(max_len))
    }
}

// ================================================================================================
// FqxSlice
// ================================================================================================

pub trait SeqSlice {
    fn empty() -> Self;

    fn sliced<I>(self, range: I) -> Self
    where
        I: FromTo;

    fn takes<I>(self, indices: I) -> Self
    where
        I: IntoIterator<Item = usize>;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<E> SeqSlice for Vec<E> {
    fn empty() -> Self {
        vec![]
    }

    fn sliced<I>(self, range: I) -> Self
    where
        I: FromTo,
    {
        slice_vec(self, range)
    }

    fn takes<I>(self, indices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        takes_vec(self, indices)
    }
}

// ================================================================================================
// SeqAppend
// ================================================================================================

pub trait SeqAppend<E> {
    fn append(&mut self, other: Self);
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<E> SeqAppend<E> for Vec<E> {
    fn append(&mut self, other: Self) {
        self.extend(other.into_iter())
    }
}

impl<'a, E> SeqAppend<E> for Cow<'a, [E]>
where
    [E]: ToOwned<Owned = Vec<E>>,
    E: Clone,
{
    fn append(&mut self, other: Self) {
        match self {
            Cow::Borrowed(b) => {
                let mut b = b.to_vec();
                b.extend(other.to_vec().into_iter());
                *self = Cow::Owned(b);
            }
            Cow::Owned(o) => o.extend(other.to_vec().into_iter()),
        }
    }
}
