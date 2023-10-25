//! file: abst.rs
//! author: Jacob Xie
//! date: 2023/10/09 09:06:36 Monday
//! brief:

use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use crate::adt::FqxValue;

// ================================================================================================
// FqxRowAbstract & FqxRowLike
// ================================================================================================

#[derive(RefCast, Debug, Clone, Serialize, Deserialize)]
#[repr(transparent)]
pub struct FqxRowAbstract<I, V>(pub(crate) I)
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>;

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a, I, V> AsRef<FqxRowAbstract<I, V>> for &'a FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
{
    fn as_ref(&self) -> &FqxRowAbstract<I, V> {
        &self
    }
}

impl<I, V, E> FromIterator<E> for FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    I: FromIterator<E>,
    V: Into<FqxValue>,
    E: Into<FqxRowAbstract<I, V>>,
{
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        FqxRowAbstract(iter.into_iter().collect())
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait FqxRowLike<I, V>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
{
    fn from_abstract(a: FqxRowAbstract<I, V>) -> Self;

    fn to_abstract(self) -> FqxRowAbstract<I, V>;

    fn as_abstract_ref(&self) -> &FqxRowAbstract<I, V>;

    fn as_abstract_mut(&mut self) -> &mut FqxRowAbstract<I, V>;
}

// ================================================================================================
// Index
// ================================================================================================

impl<I, V> Index<usize> for FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    I: Index<usize, Output = V>,
    V: Into<FqxValue>,
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<I, V> IndexMut<usize> for FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    I: IndexMut<usize, Output = V>,
    V: Into<FqxValue>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! impl_index_range_for_abs {
    () => {
        impl<I, V> Index<RangeFull> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V>,
            I: Index<usize, Output = V> + Index<RangeFull, Output = [V]>,
            V: Into<FqxValue>,
        {
            type Output = [V];

            fn index(&self, index: RangeFull) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<I, V> IndexMut<RangeFull> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V>,
            I: IndexMut<usize, Output = V> + IndexMut<RangeFull, Output = [V]>,
            V: Into<FqxValue>,
        {
            fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
    ($t:ident) => {
        impl<I, V> Index<$t<usize>> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V>,
            I: Index<usize, Output = V> + Index<$t<usize>, Output = [V]>,
            V: Into<FqxValue>,
        {
            type Output = [V];

            fn index(&self, index: $t<usize>) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<I, V> IndexMut<$t<usize>> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V>,
            I: IndexMut<usize, Output = V> + IndexMut<$t<usize>, Output = [V]>,
            V: Into<FqxValue>,
        {
            fn index_mut(&mut self, index: $t<usize>) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
}

impl_index_range_for_abs!();
impl_index_range_for_abs!(Range);
impl_index_range_for_abs!(RangeFrom);
impl_index_range_for_abs!(RangeTo);
impl_index_range_for_abs!(RangeToInclusive);
impl_index_range_for_abs!(RangeInclusive);
