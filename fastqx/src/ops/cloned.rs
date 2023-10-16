//! file: cloned.rs
//! author: Jacob Xie
//! date: 2023/10/08 19:42:15 Sunday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::ops::{FqxDataRef, FqxGroup, FqxRowSelect};

// ================================================================================================
// OpCloned
// ================================================================================================

pub trait OpCloned {
    type Ret;

    fn cloned(self) -> Self::Ret;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<T, V> OpCloned for FqxGroup<T>
where
    for<'b> &'b T: IntoIterator<Item = &'b V>,
    V: Into<FqxRow> + Clone,
{
    type Ret = FqxGroup<Vec<FqxRow>>;

    fn cloned(self) -> Self::Ret {
        let inner = (&self.0)
            .into_iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.into_iter().cloned().map(|e| e.into()).collect(),
                )
            })
            .collect::<HashMap<_, _>>();

        FqxGroup(inner)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxRowSelect<A> & Vec<FqxRowSelect<A>>

impl<A> OpCloned for FqxRowSelect<A>
where
    A: Into<FqxValue> + Clone,
{
    type Ret = FqxRowSelect<FqxValue>;

    fn cloned(self) -> Self::Ret {
        FqxRowSelect(self.0.iter().cloned().map(|e| e.into()).collect())
    }
}

impl<A> OpCloned for Vec<FqxRowSelect<A>>
where
    A: Into<FqxValue> + Clone,
{
    type Ret = Vec<FqxRowSelect<FqxValue>>;

    fn cloned(self) -> Self::Ret {
        self.into_iter().map(OpCloned::cloned).collect()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> OpCloned for FqxDataRef<'a> {
    type Ret = FqxData;

    fn cloned(self) -> Self::Ret {
        FqxData {
            columns: self.columns.into_iter().map(Clone::clone).collect(),
            types: self.types.into_iter().map(Clone::clone).collect(),
            data: self.data.into_iter().map(FqxRow::from).collect(),
        }
    }
}
