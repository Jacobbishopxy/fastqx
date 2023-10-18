//! file: owned.rs
//! author: Jacob Xie
//! date: 2023/10/08 19:42:15 Sunday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxD, FqxData, FqxRow, FqxValue, PhantomU};
use crate::ops::{FqxDataRef, FqxGroup, FqxRowSelect};

// ================================================================================================
// OpCloned
// ================================================================================================

pub trait OpOwned<T> {
    type Ret;

    fn to_owned(self) -> Self::Ret;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<U, C, T, I, E> OpOwned<PhantomU<C, T, I, E>> for FqxGroup<U>
where
    Self: Sized,
    U: FqxD<C, T, I, E> + OpOwned<FqxData, Ret = FqxData>,
    C: Clone,
    T: Clone,
    I: Default + Clone,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    type Ret = FqxGroup<FqxData>;

    fn to_owned(self) -> Self::Ret {
        let inner = self
            .0
            .into_iter()
            .map(|(k, v)| (k.clone(), v.to_owned()))
            .collect::<HashMap<_, _>>();

        FqxGroup(inner)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxRowSelect<A> & Vec<FqxRowSelect<A>>

impl<A> OpOwned<A> for FqxRowSelect<A>
where
    A: Into<FqxValue> + Clone,
{
    type Ret = FqxRowSelect<FqxValue>;

    fn to_owned(self) -> Self::Ret {
        FqxRowSelect(self.0.iter().cloned().map(|e| e.into()).collect())
    }
}

impl<A> OpOwned<A> for Vec<FqxRowSelect<A>>
where
    A: Into<FqxValue> + Clone,
{
    type Ret = Vec<FqxRowSelect<FqxValue>>;

    fn to_owned(self) -> Self::Ret {
        self.into_iter().map(OpOwned::to_owned).collect()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl OpOwned<FqxData> for FqxData {
    type Ret = FqxData;

    fn to_owned(self) -> Self::Ret {
        self
    }
}

impl<'a> OpOwned<FqxData> for FqxDataRef<'a> {
    type Ret = FqxData;

    fn to_owned(self) -> Self::Ret {
        FqxData {
            columns: self.columns.into_iter().map(Clone::clone).collect(),
            types: self.types.into_iter().map(Clone::clone).collect(),
            data: self.data.into_iter().map(FqxRow::from).collect(),
        }
    }
}
