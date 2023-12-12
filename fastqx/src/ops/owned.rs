//! file: owned.rs
//! author: Jacob Xie
//! date: 2023/10/08 19:42:15 Sunday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxD, FqxData, FqxDataCow, FqxRow};
use crate::ops::FqxGroup;

// ================================================================================================
// OpOwned
// ================================================================================================

pub trait OpOwned {
    type Ret;

    fn to_owned(self) -> Self::Ret;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<U> OpOwned for FqxGroup<U>
where
    Self: Sized,
    U: FqxD,
{
    type Ret = FqxGroup<U>;

    fn to_owned(self) -> Self::Ret {
        let inner = self
            .0
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect::<HashMap<_, _>>();

        FqxGroup(inner)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl OpOwned for FqxData {
    type Ret = FqxData;

    fn to_owned(self) -> Self::Ret {
        self
    }
}

impl<'a> OpOwned for FqxDataCow<'a> {
    type Ret = FqxData;

    fn to_owned(self) -> Self::Ret {
        FqxData {
            columns: self.columns.into_iter().map(Clone::clone).collect(),
            types: self.types.into_iter().map(Clone::clone).collect(),
            data: self.data.into_iter().map(FqxRow::from).collect(),
        }
    }
}
