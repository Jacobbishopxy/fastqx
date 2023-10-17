//! file: select.rs
//! author: Jacob Xie
//! date: 2023/09/25 15:16:03 Monday
//! brief:

use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use ref_cast::RefCast;

use crate::adt::{FqxData, FqxRow, FqxRowAbstract, FqxRowLike, FqxValue};
use crate::ops::{FqxDataRef, FqxIdx};

// ================================================================================================
// FqxSelect
// ================================================================================================

#[derive(RefCast, Debug, Default, Clone)]
#[repr(transparent)]
pub struct FqxRowSelect<A>(pub(crate) Vec<A>)
where
    A: Into<FqxValue>;

pub type FqxRowRef<'a> = FqxRowSelect<&'a FqxValue>;

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

impl<A> Into<FqxRowAbstract<Vec<A>, A>> for FqxRowSelect<A>
where
    A: Into<FqxValue>,
{
    fn into(self) -> FqxRowAbstract<Vec<A>, A> {
        FqxRowAbstract(self.0)
    }
}

impl<A> From<FqxRowAbstract<Vec<A>, A>> for FqxRowSelect<A>
where
    A: Into<FqxValue>,
{
    fn from(value: FqxRowAbstract<Vec<A>, A>) -> Self {
        FqxRowSelect(value.0)
    }
}

impl<A> FqxRowLike<Vec<A>, A> for FqxRowSelect<A>
where
    A: Into<FqxValue>,
{
    fn from_abstract(a: FqxRowAbstract<Vec<A>, A>) -> Self {
        FqxRowSelect(a.0)
    }

    fn to_abstract(self) -> FqxRowAbstract<Vec<A>, A> {
        FqxRowAbstract(self.0)
    }

    fn as_abstract_ref(&self) -> &FqxRowAbstract<Vec<A>, A> {
        self.as_ref()
    }

    fn as_abstract_mut(&mut self) -> &mut FqxRowAbstract<Vec<A>, A> {
        self.as_mut()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl From<FqxRowSelect<FqxValue>> for FqxRow {
    fn from(value: FqxRowSelect<FqxValue>) -> Self {
        FqxRow(value.0)
    }
}

impl<'a> From<FqxRowRef<'a>> for FqxRow {
    fn from(value: FqxRowRef<'a>) -> Self {
        FqxRow(value.0.into_iter().cloned().collect())
    }
}

impl<'a> From<&'a FqxRow> for FqxRowRef<'a> {
    fn from(value: &'a FqxRow) -> Self {
        FqxRowSelect(value.into_iter().collect())
    }
}

impl From<FqxRow> for FqxRowSelect<FqxValue> {
    fn from(value: FqxRow) -> Self {
        FqxRowSelect(value.0)
    }
}

impl From<Vec<FqxValue>> for FqxRowSelect<FqxValue> {
    fn from(value: Vec<FqxValue>) -> Self {
        FqxRowSelect(value)
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

impl<'a> IntoIterator for &'a mut FqxRowSelect<FqxValue> {
    type Item = &'a mut FqxValue;

    type IntoIter = std::slice::IterMut<'a, FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl FromIterator<FqxValue> for FqxRowSelect<FqxValue> {
    fn from_iter<T: IntoIterator<Item = FqxValue>>(iter: T) -> Self {
        FqxRowSelect(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for FqxRowRef<'a> {
    type Item = &'a FqxValue;

    type IntoIter = std::vec::IntoIter<&'a FqxValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> FromIterator<&'a FqxValue> for FqxRowRef<'a> {
    fn from_iter<T: IntoIterator<Item = &'a FqxValue>>(iter: T) -> Self {
        FqxRowSelect(iter.into_iter().collect())
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> Default for FqxRowRef<'a> {
    fn default() -> Self {
        Self(vec![])
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
// OpSelect
// ================================================================================================

pub trait OpSelect<'a> {
    fn select<I>(&'a self, idx: I) -> FqxDataRef<'a>
    where
        I: FqxIdx<'a>;

    fn take<I>(self, idx: I) -> Self
    where
        I: FqxIdx<'a>;

    fn rf(&'a self) -> FqxDataRef<'a> {
        self.select(..)
    }
}

// ================================================================================================
// Impl
// ================================================================================================

impl<'a> OpSelect<'a> for FqxData {
    fn select<I>(&'a self, idx: I) -> FqxDataRef<'a>
    where
        I: FqxIdx<'a>,
    {
        let d = FqxDataRef {
            columns: self.columns.iter().collect(),
            types: self.types.iter().collect(),
            data: self
                .data
                .iter()
                .map(|r| FqxRowSelect(r.into_iter().collect()))
                .collect(),
        };

        idx.cvt_ref(d)
    }

    fn take<I>(self, idx: I) -> Self
    where
        I: FqxIdx<'a>,
    {
        idx.cvt_own(self)
    }
}

impl<'a> OpSelect<'a> for FqxDataRef<'a> {
    fn select<I>(&'a self, idx: I) -> FqxDataRef<'a>
    where
        I: FqxIdx<'a>,
    {
        // cloning references which points to the original `FqxData`
        let d = FqxDataRef {
            columns: self.columns.iter().cloned().collect(),
            types: self.types.iter().cloned().collect(),
            data: self
                .data
                .iter()
                .map(|r| FqxRowSelect(r.clone().into_iter().collect()))
                .collect(),
        };

        idx.cvt_ref(d)
    }

    fn take<I>(self, idx: I) -> Self
    where
        I: FqxIdx<'a>,
    {
        idx.cvt_ref(self)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_select {
    use super::*;
    use crate::adt::{FqxValue, FqxValueType};

    #[test]
    fn as_abstract_success() {
        let foo = FqxRowSelect(vec![
            FqxValue::Null,
            FqxValue::I16(0),
            FqxValue::String("ha".to_string()),
        ]);

        let bar = foo.as_abstract_ref();

        println!("{:?}", bar[0]);
    }

    #[test]
    fn as_abstract_arith_success() {
        let mut a1 = FqxRowSelect(vec![
            FqxValue::F32(0.1),
            FqxValue::I16(0),
            FqxValue::String("ha".to_string()),
        ]);

        let a2 = FqxRowSelect(vec![
            FqxValue::F32(0.2),
            FqxValue::I16(0),
            FqxValue::String("!".to_string()),
        ]);

        *a1.as_abstract_mut() += a2.to_abstract();

        println!("{:?}", a1);
    }

    use once_cell::sync::Lazy;

    static DATA: Lazy<FqxData> = Lazy::new(|| {
        FqxData::new(
            vec![String::from("c1"), String::from("c2"), String::from("c3")],
            vec![FqxValueType::I32, FqxValueType::String, FqxValueType::F32],
            vec![
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("A")),
                    FqxValue::F32(2.1),
                ],
                vec![
                    FqxValue::I32(2),
                    FqxValue::String(String::from("B")),
                    FqxValue::F32(1.3),
                ],
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("C")),
                    FqxValue::F32(3.2),
                ],
            ],
        )
        .unwrap()
    });

    #[test]
    fn select_success() {
        let data = DATA.clone();

        let refd = data.select(1);
        println!("{:?}", refd);
        let refd = data.select(..);
        println!("{:?}", refd);
        let refd = data.select(1..2);
        println!("{:?}", refd);
        let refd = data.select(1..);
        println!("{:?}", refd);
        let refd = data.select(1..=2);
        println!("{:?}", refd);
        let refd = data.select(..2);
        println!("{:?}", refd);
        let refd = data.select(..=2);
        println!("{:?}", refd);

        println!();

        let refd = data.select((0, 1));
        println!("{:?}", refd);
        let refd = data.select((0, ..));
        println!("{:?}", refd);
        let refd = data.select((0, 1..2));
        println!("{:?}", refd);
        let refd = data.select((0, 1..));
        println!("{:?}", refd);
        let refd = data.select((0, 1..=2));
        println!("{:?}", refd);
        let refd = data.select((0, ..2));
        println!("{:?}", refd);
        let refd = data.select((0, ..=2));
        println!("{:?}", refd);
    }

    #[test]
    fn select_success2() {
        let data = DATA.clone();

        let refd = data.select("c2");
        println!("{:?}", refd);
        let refd = data.select(String::from("c2"));
        println!("{:?}", refd);

        let refd = data.select([2, 0].as_slice());
        println!("{:?}", refd);
        let refd = data.select(vec![2, 0]);
        println!("{:?}", refd);

        let refd = data.select(["c3", "c1"].as_slice());
        println!("{:?}", refd);
        let refd = data.select(vec![String::from("c3"), String::from("c1")]);
        println!("{:?}", refd);
    }

    #[test]
    fn select_select_success() {
        let data = DATA.clone();

        let refd1 = data.select([2, 0].as_slice());
        println!("{:?}", refd1);
        let refd2 = refd1.select(1..);
        println!("{:?}", refd2);
    }
}
