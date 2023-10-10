//! file: merge.rs
//! author: Jacob Xie
//! date: 2023/10/10 09:11:09 Tuesday
//! brief:

use itertools::Itertools;

use crate::adt::{FqxRowAbstract, FqxValue};

// ================================================================================================
// OpMerge
// ================================================================================================

pub trait OpMerge<T> {
    type Item;
    type Ret<A>;

    fn merge_by<F>(self, other: Self, f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<I, V, T, E> OpMerge<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
    // TODO: need a new trait that includes `FromIterator<E>` and "from schema", e.g. `FqxData`
    T: FromIterator<E>,
{
    type Item = E;

    type Ret<A> = T;

    fn merge_by<F>(self, other: Self, f: F) -> Self
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        Itertools::merge_by(self.into_iter(), other.into_iter(), f).collect()
    }
}
