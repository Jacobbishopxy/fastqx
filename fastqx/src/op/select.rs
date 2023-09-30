//! file: select.rs
//! author: Jacob Xie
//! date: 2023/09/25 15:16:03 Monday
//! brief:

use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use ref_cast::RefCast;

use crate::adt::{FqxData, FqxRow, FqxRowAbstract, FqxValue};
use crate::op::FqxSlice;

// ================================================================================================
// OpSelect
// ================================================================================================

pub trait OpSelect<I> {
    type Ret<A>;

    fn select(self, indices: &[usize]) -> Self::Ret<I>;
}

// ================================================================================================
// FqxSelect
// ================================================================================================

#[derive(RefCast, Debug, Clone)]
#[repr(transparent)]
pub struct FqxRowSelect<A>(pub(crate) Vec<A>)
where
    A: Into<FqxValue>;

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxRowAbstract

impl<A> AsRef<FqxRowAbstract<Vec<A>, A>> for FqxRowSelect<A>
where
    A: Into<FqxValue>,
{
    fn as_ref(&self) -> &FqxRowAbstract<Vec<A>, A> {
        FqxRowAbstract::ref_cast(&self.0)
    }
}

impl<A> AsMut<FqxRowAbstract<Vec<A>, A>> for FqxRowSelect<A>
where
    A: Into<FqxValue>,
{
    fn as_mut(&mut self) -> &mut FqxRowAbstract<Vec<A>, A> {
        FqxRowAbstract::ref_cast_mut(&mut self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl From<FqxRowSelect<FqxValue>> for FqxRow {
    fn from(value: FqxRowSelect<FqxValue>) -> Self {
        FqxRow(value.0)
    }
}

impl<'a> From<FqxRowSelect<&'a FqxValue>> for FqxRow {
    fn from(value: FqxRowSelect<&'a FqxValue>) -> Self {
        FqxRow(value.0.into_iter().cloned().collect())
    }
}

impl From<FqxRow> for FqxRowSelect<FqxValue> {
    fn from(value: FqxRow) -> Self {
        FqxRowSelect(value.0)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl IntoIterator for FqxRowSelect<FqxValue> {
    type Item = FqxValue;

    type IntoIter = std::vec::IntoIter<FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<FqxValue> for FqxRowSelect<FqxValue> {
    fn from_iter<T: IntoIterator<Item = FqxValue>>(iter: T) -> Self {
        FqxRowSelect(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for FqxRowSelect<&'a FqxValue> {
    type Item = &'a FqxValue;

    type IntoIter = std::vec::IntoIter<&'a FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Index & IndexMut

impl<A> Index<usize> for FqxRowSelect<A>
where
    A: Into<FqxValue>,
{
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<A> IndexMut<usize> for FqxRowSelect<A>
where
    A: Into<FqxValue>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! impl_index_range {
    () => {
        impl<A> Index<RangeFull> for FqxRowSelect<A>
        where
            A: Into<FqxValue>,
        {
            type Output = [A];

            fn index(&self, index: RangeFull) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<A> IndexMut<RangeFull> for FqxRowSelect<A>
        where
            A: Into<FqxValue>,
        {
            fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
    ($t:ident) => {
        impl<A> Index<$t<usize>> for FqxRowSelect<A>
        where
            A: Into<FqxValue>,
        {
            type Output = [A];

            fn index(&self, index: $t<usize>) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<A> IndexMut<$t<usize>> for FqxRowSelect<A>
        where
            A: Into<FqxValue>,
        {
            fn index_mut(&mut self, index: $t<usize>) -> &mut Self::Output {
                &mut self.0[index]
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
// Impl
// ================================================================================================

impl OpSelect<FqxRowSelect<FqxValue>> for FqxRow {
    type Ret<A> = A;

    fn select(mut self, indices: &[usize]) -> Self::Ret<FqxRowSelect<FqxValue>> {
        let s = indices
            .iter()
            .filter_map(|i| {
                let mut d = FqxValue::Null;
                self.0.get_mut(*i).map(|v| {
                    std::mem::swap(&mut d, v);
                    d
                })
            })
            .collect();
        FqxRowSelect(s)
    }
}

impl<'a> OpSelect<FqxRowSelect<&'a FqxValue>> for &'a FqxRow {
    type Ret<A> = A;

    fn select(self, indices: &[usize]) -> Self::Ret<FqxRowSelect<&'a FqxValue>> {
        let s = indices.iter().filter_map(|i| self.0.get(*i)).collect();
        FqxRowSelect(s)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl OpSelect<FqxRowSelect<FqxValue>> for FqxData {
    type Ret<A> = Vec<A>;

    fn select(self, indices: &[usize]) -> Self::Ret<FqxRowSelect<FqxValue>> {
        self.iter_owned()
            .map(|r| r.select(indices))
            .collect::<Vec<_>>()
    }
}

impl<'a> OpSelect<FqxRowSelect<&'a FqxValue>> for &'a FqxData {
    type Ret<A> = Vec<A>;

    fn select(self, indices: &[usize]) -> Self::Ret<FqxRowSelect<&'a FqxValue>> {
        self.iter().map(|r| r.select(indices)).collect::<Vec<_>>()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> OpSelect<FqxRowSelect<&'a FqxValue>> for &'a FqxSlice {
    type Ret<A> = Vec<A>;

    fn select(self, indices: &[usize]) -> Self::Ret<FqxRowSelect<&'a FqxValue>> {
        self.0.iter().map(|r| r.select(indices)).collect::<Vec<_>>()
    }
}

#[test]
fn tmp() {
    let foo = FqxRowSelect(vec![
        FqxValue::Null,
        FqxValue::I16(0),
        FqxValue::String("ha".to_string()),
    ]);

    println!("{:?}", &foo.as_ref()[0]);

    let foo = FqxRowAbstract(vec![
        FqxValue::Null,
        FqxValue::I16(0),
        FqxValue::String("ha".to_string()),
    ]);

    println!("{:?}", &foo[0..2]);
}
