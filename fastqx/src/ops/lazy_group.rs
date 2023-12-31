//! file: lazy_group.rs
//! author: Jacob Xie
//! date: 2023/12/30 22:08:48 Saturday
//! brief:

use std::collections::HashSet;

use itertools::{GroupBy, Itertools};

use crate::adt::*;

/*
Lazy FqxGroup for FqxD trait

fn:

- agg
- all
- apply
- count
- first
- head
- last
- map_groups
- max
- mean
- median
- min
- n_unique
- quantile
- sum
- tail
*/

// ================================================================================================
// FqxLazyGroup
// ================================================================================================

pub struct FqxLazyGroup<'a, D>
where
    D: FqxD,
{
    pub d: &'a D,
    pub(crate) selected_keys: Vec<usize>,
    pub(crate) selected_aggs: Vec<usize>,
}

fn group_fn<'a, D>(row: &'a D::RowT, pos: &[usize]) -> Vec<&'a FqxValue>
where
    D: FqxD,
{
    pos.iter().filter_map(|&i| row.get(i)).collect_vec()
}

impl<'a, D> FqxLazyGroup<'a, D>
where
    D: FqxD,
{
    pub fn select<N, S>(mut self, cols: &N) -> Self
    where
        for<'n> &'n N: IntoIterator<Item = &'n S>,
        S: AsRef<str>,
    {
        let mut selected_aggs = vec![];
        let cols = cols.into_iter().map(|e| e.as_ref()).collect::<HashSet<_>>();

        for (i, c) in self.d.columns().into_iter().enumerate() {
            if cols.contains(c.as_str()) {
                selected_aggs.push(i);
            }
        }

        self.selected_aggs = selected_aggs;
        self
    }

    pub(crate) fn to_group(
        &'a self,
    ) -> GroupBy<
        Vec<&'a FqxValue>,
        std::slice::Iter<'a, D::RowT>,
        impl FnMut(&&'a D::RowT) -> Vec<&'a FqxValue>,
    > {
        self.d
            .iter()
            .group_by(|r| group_fn::<D>(r, &self.selected_keys))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait OpLazyGroup<'a, K, T>
where
    K: PartialEq,
{
    type Item;
    type Ret;

    fn group_by<N, S>(&'a self, by: &N) -> Self::Ret
    where
        for<'n> &'n N: IntoIterator<Item = &'n S>,
        S: AsRef<str>;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a, K, T, U> OpLazyGroup<'a, K, T> for U
where
    K: PartialEq,
    U: FqxD + 'a,
{
    type Item = U::RowT;
    type Ret = FqxLazyGroup<'a, U>;

    fn group_by<N, S>(&'a self, by: &N) -> Self::Ret
    where
        for<'n> &'n N: IntoIterator<Item = &'n S>,
        S: AsRef<str>,
    {
        let mut selected_keys = vec![];
        let mut selected_aggs = vec![];

        let by = by.into_iter().map(|e| e.as_ref()).collect::<HashSet<_>>();
        for (i, c) in self.columns().into_iter().enumerate() {
            if by.contains(c.as_str()) {
                selected_keys.push(i);
            } else {
                selected_aggs.push(i);
            }
        }

        FqxLazyGroup {
            d: &self,
            selected_keys,
            selected_aggs,
        }
    }
}
