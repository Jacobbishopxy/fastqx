//! file: group.rs
//! author: Jacob Xie
//! date: 2023/09/22 20:06:22 Friday
//! brief:

use std::collections::HashMap;

use itertools::Itertools;
use ref_cast::RefCast;

use crate::adt::*;

// ================================================================================================
// OpGroup
// ================================================================================================

pub trait OpGroup<K, T>
where
    Self: Sized,
    K: PartialEq,
{
    type Item;
    type Col;
    type Ret<A>;

    fn group_by_<N>(self, by: &N) -> Self::Ret<Self>
    where
        for<'a> &'a N: IntoIterator<Item = &'a Self::Col>;

    fn group_by_fn_<F>(self, f: F) -> Self::Ret<Self>
    where
        F: Fn(&Self::Item) -> K;
}

// ================================================================================================
// FqxGroup
// ================================================================================================

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct FqxGroup<T>(pub(crate) HashMap<Vec<FqxValue>, T>);

impl<T> FqxGroup<T> {
    pub fn to_hashmap(self) -> HashMap<Vec<FqxValue>, T> {
        self.0
    }
}

// ================================================================================================
// Impl
// ================================================================================================

impl<U> OpGroup<Vec<FqxValue>, U> for U
where
    Self: Sized,
    U: FqxD,
{
    type Item = U::RowT;

    type Col = String;

    type Ret<A> = FqxGroup<A>;

    fn group_by_<N>(self, by: &N) -> Self::Ret<Self>
    where
        for<'a> &'a N: IntoIterator<Item = &'a Self::Col>,
    {
        let pos = self.columns_position(by);
        self.group_by_fn_(|r| {
            pos.iter()
                .filter_map(|&i| r.get(i).cloned())
                .collect::<Vec<_>>()
        })
    }

    fn group_by_fn_<F>(self, f: F) -> Self::Ret<Self>
    where
        F: Fn(&Self::Item) -> Vec<FqxValue>,
    {
        let (c, t, d) = self.dcst();

        let mut res = HashMap::new();
        Itertools::chunk_by(d.into_iter(), f)
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        let res = res
            .into_iter()
            .map(|(k, g)| (k, U::cst(c.clone(), t.clone(), g)))
            .collect::<HashMap<_, _>>();

        FqxGroup(res)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_group_by {
    use crate::ops::mock::data::D5;
    use crate::ops::{OpGroup, OpSelect};

    #[test]
    fn group_success() {
        let d = D5.clone();

        let foo = d.rf().group_by_fn_(|r| vec![r[0].clone()]);
        println!("{:?}", foo);

        let foo = d.group_by_fn_(|r| vec![r[0].clone()]);
        println!("{:?}", foo);
    }

    #[test]
    fn group_success2() {
        let d = D5.clone();

        let by = vec![String::from("col_1")];
        let foo = d.rf().group_by_(&by);
        println!("{:?}", foo);

        let foo = d.group_by_(&by);
        println!("{:?}", foo);
    }
}
