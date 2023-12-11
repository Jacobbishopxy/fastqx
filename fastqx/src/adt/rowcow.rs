//! file: rowcow.rs
//! author: Jacob Xie
//! date: 2023/12/08 15:56:31 Friday
//! brief:

use std::borrow::Cow;
use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use super::util::{slice_cow, takes_cow};
use crate::adt::{FqxRow, FqxValue, FqxValueType, FromTo, RowProps, SeqSlice};

// TODO

#[derive(
    RefCast, Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord,
)]
#[repr(transparent)]
pub struct FqxRowCow<'a>(pub(crate) Cow<'a, [FqxValue]>);

// ================================================================================================
// Conversions
// ================================================================================================

impl<'a> FqxRowCow<'a> {
    pub fn new(d: Vec<FqxValue>) -> Self {
        Self(Cow::Owned(d))
    }
}

impl<'a> From<Vec<FqxValue>> for FqxRowCow<'a> {
    fn from(value: Vec<FqxValue>) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> From<FqxRow> for FqxRowCow<'a> {
    fn from(value: FqxRow) -> Self {
        Self(Cow::Owned(value.0))
    }
}

impl<'a> From<&'a [FqxValue]> for FqxRowCow<'a> {
    fn from(value: &'a [FqxValue]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'a> From<&'a FqxRow> for FqxRowCow<'a> {
    fn from(value: &'a FqxRow) -> Self {
        Self(Cow::Borrowed(value.as_ref()))
    }
}

impl<'a> AsRef<FqxRow> for FqxRowCow<'a> {
    fn as_ref(&self) -> &FqxRow {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> From<FqxRowCow<'a>> for FqxRow {
    fn from(value: FqxRowCow<'a>) -> Self {
        match value.0 {
            Cow::Borrowed(b) => FqxRow(b.to_vec()),
            Cow::Owned(o) => FqxRow(o),
        }
    }
}

impl<'a> From<&'a FqxRowCow<'a>> for FqxRow {
    fn from(value: &'a FqxRowCow<'a>) -> Self {
        match &value.0 {
            Cow::Borrowed(b) => FqxRow(b.to_vec()),
            Cow::Owned(o) => FqxRow(o.clone()),
        }
    }
}

// ================================================================================================
// impl RowProps
// ================================================================================================

impl<'a> RowProps for FqxRowCow<'a> {
    fn get_nth(&self, idx: usize) -> Option<&FqxValue> {
        self.0.get(idx)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn types(&self) -> Vec<FqxValueType> {
        self.0.iter().map(FqxValueType::from).collect()
    }

    fn to_values(self) -> Vec<FqxValue> {
        self.0.to_vec()
    }
}

// ================================================================================================
// impl SeqSlice
// ================================================================================================

impl<'a> SeqSlice for FqxRowCow<'a> {
    fn empty() -> Self {
        FqxRowCow(Cow::Borrowed(&[]))
    }

    fn sliced<I>(self, range: I) -> Self
    where
        I: FromTo,
    {
        FqxRowCow(slice_cow(self.0, range))
    }

    fn takes<I>(self, indices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        FqxRowCow(takes_cow(self.0, indices))
    }
}

// ================================================================================================
// FqxRowCow: IntoIterator & FromIterator, Extend
// ================================================================================================

impl<'a> IntoIterator for FqxRowCow<'a> {
    type Item = FqxValue;

    type IntoIter = std::vec::IntoIter<FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_values().into_iter()
    }
}

impl<'a> FromIterator<FqxValue> for FqxRowCow<'a> {
    fn from_iter<T: IntoIterator<Item = FqxValue>>(iter: T) -> Self {
        FqxRowCow(Cow::Owned(iter.into_iter().collect()))
    }
}

impl<'a> IntoIterator for &'a FqxRowCow<'a> {
    type Item = &'a FqxValue;

    type IntoIter = std::slice::Iter<'a, FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> Extend<FqxValue> for FqxRowCow<'a> {
    fn extend<T: IntoIterator<Item = FqxValue>>(&mut self, iter: T) {
        self.0.to_mut().extend(iter.into_iter())
    }
}

// ================================================================================================
// FqxRowCow: Index<usize>
// No boundary check!
// ================================================================================================

impl<'a> Index<usize> for FqxRowCow<'a> {
    type Output = FqxValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> IndexMut<usize> for FqxRowCow<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.to_mut().get_mut(index).unwrap()
    }
}

macro_rules! impl_index_range {
    () => {
        impl<'a> Index<RangeFull> for FqxRowCow<'a> {
            type Output = [FqxValue];

            fn index(&self, index: RangeFull) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<'a> IndexMut<RangeFull> for FqxRowCow<'a> {
            fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
                self.0.to_mut().get_mut(index).unwrap()
            }
        }
    };
    ($t:ident) => {
        impl<'a> Index<$t<usize>> for FqxRowCow<'a> {
            type Output = [FqxValue];

            fn index(&self, index: $t<usize>) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<'a> IndexMut<$t<usize>> for FqxRowCow<'a> {
            fn index_mut(&mut self, index: $t<usize>) -> &mut Self::Output {
                self.0.to_mut().get_mut(index).unwrap()
            }
        }
    };
}

impl_index_range!();
impl_index_range!(Range);
impl_index_range!(RangeFrom);
impl_index_range!(RangeTo);
impl_index_range!(RangeToInclusive);
impl_index_range!(RangeInclusive);
