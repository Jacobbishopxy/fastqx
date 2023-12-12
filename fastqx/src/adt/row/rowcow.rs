//! file: rowcow.rs
//! author: Jacob Xie
//! date: 2023/12/08 15:56:31 Friday
//! brief:

use std::borrow::Cow;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, RangeFrom, RangeFull,
    RangeInclusive, RangeTo, RangeToInclusive, Rem, RemAssign, Sub, SubAssign,
};

use itertools::{EitherOrBoth, Itertools};
use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use crate::adt::util::{slice_cow, takes_cow};
use crate::adt::{FqxRow, FqxValue, FqxValueType, FromTo, RowProps, SeqSlice};

// ================================================================================================
// FqxRowCow
// ================================================================================================

#[derive(
    RefCast, Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord,
)]
#[repr(transparent)]
pub struct FqxRowCow<'a>(pub(crate) Cow<'a, [FqxValue]>);

impl<'a> FqxRowCow<'a> {
    pub fn new(d: Vec<FqxValue>) -> Self {
        Self(Cow::Owned(d))
    }

    pub fn to_mut(&mut self) -> &mut Vec<FqxValue> {
        self.0.to_mut()
    }
}
// ================================================================================================
// Conversions
// ================================================================================================

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
    fn nulls(len: usize) -> Self {
        Self(Cow::Owned(vec![FqxValue::Null; len]))
    }

    fn get(&self, idx: usize) -> Option<&FqxValue> {
        self.0.get(idx)
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut FqxValue> {
        self.to_mut().get_mut(idx)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn types(&self) -> Vec<FqxValueType> {
        self.iter().map(FqxValueType::from).collect()
    }

    fn to_values(self) -> Vec<FqxValue> {
        self.0.to_vec()
    }

    fn from_values(d: Vec<FqxValue>) -> Self {
        Self::new(d)
    }

    fn iter_owned(self) -> std::vec::IntoIter<FqxValue> {
        self.into_iter()
    }

    fn iter(&self) -> std::slice::Iter<'_, FqxValue> {
        self.into_iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, FqxValue> {
        self.to_mut().into_iter()
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn sub(&self, rhs: &Self) -> Self {
        self - rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }

    fn div(&self, rhs: &Self) -> Self {
        self / rhs
    }

    fn rem(&self, rhs: &Self) -> Self {
        self % rhs
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

impl<'a> IntoIterator for &'a mut FqxRowCow<'a> {
    type Item = &'a mut FqxValue;

    type IntoIter = std::slice::IterMut<'a, FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_mut().iter_mut()
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
        self.to_mut().get_mut(index).unwrap()
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
                self.to_mut().get_mut(index).unwrap()
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
                self.to_mut().get_mut(index).unwrap()
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

// ================================================================================================
// Arithmetic: FqxRowCow
// ================================================================================================

macro_rules! impl_arith_for_row {
    ($t:ident, $tf:tt, $ta:ident, $taf:tt, $op:tt, $opa:tt) => {
        impl<'a> $t for FqxRowCow<'a> {
            type Output = FqxRowCow<'a>;

            fn $tf(self, rhs: Self) -> Self::Output {
                let inner = self
                    .into_iter()
                    .zip_longest(rhs.into_iter())
                    .map(|pair| match pair {
                        itertools::EitherOrBoth::Both(l, r) => l $op r,
                        _ => FqxValue::Null,
                    })
                    .collect();

                FqxRowCow(inner)
            }
        }

        impl<'a> $ta for FqxRowCow<'a> {
            fn $taf(&mut self, rhs: Self) {
                self.to_mut()
                    .into_iter()
                    .zip_longest(rhs.into_iter())
                    .for_each(move |pair| match pair {
                        EitherOrBoth::Both(l, r) => *l $opa r,
                        _ => {}
                    })
            }
        }

        impl<'a> $t for &FqxRowCow<'a> {
            type Output = FqxRowCow<'a>;

            fn $tf(self, rhs: Self) -> Self::Output {
                let inner = self
                    .into_iter()
                    .zip_longest(rhs.into_iter())
                    .map(|pair| match pair {
                        itertools::EitherOrBoth::Both(l, r) => l $op r,
                        _ => FqxValue::Null,
                    })
                    .collect();

                FqxRowCow(inner)
            }
        }
    };
}

impl_arith_for_row!(Add, add, AddAssign, add_assign, +, +=);
impl_arith_for_row!(Sub, sub, SubAssign, sub_assign, -, -=);
impl_arith_for_row!(Mul, mul, MulAssign, mul_assign, *, *=);
impl_arith_for_row!(Div, div, DivAssign, div_assign, /, /=);
impl_arith_for_row!(Rem, rem, RemAssign, rem_assign, %, %=);
