//! file: group.rs
//! author: Jacob Xie
//! date: 2023/09/22 20:06:22 Friday
//! brief:

use std::collections::HashMap;

use itertools::Itertools;
use ref_cast::RefCast;

use crate::adt::*;
use crate::ops::{FqxRowSelect, FqxSlice};

// ================================================================================================
// OpGroup
// ================================================================================================

pub trait OpGroup<K, I>
where
    K: PartialEq,
{
    type Ret<A>;

    fn group_by<F>(self, f: F) -> Self::Ret<I>
    where
        F: Fn(&I) -> K;
}

// ================================================================================================
// FqxGroup
// ================================================================================================

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct FqxGroup<A>(pub(crate) HashMap<Vec<FqxValue>, A>);

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxData

impl OpGroup<Vec<FqxValue>, FqxRow> for FqxData {
    type Ret<A> = FqxGroup<Vec<A>>;

    fn group_by<F>(self, f: F) -> Self::Ret<FqxRow>
    where
        F: Fn(&FqxRow) -> Vec<FqxValue>,
    {
        let mut res = HashMap::new();
        self.iter_owned()
            .group_by(f)
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        FqxGroup(res)
    }
}

impl<'a> OpGroup<Vec<FqxValue>, &'a FqxRow> for &'a FqxData {
    type Ret<A> = FqxGroup<Vec<A>>;

    fn group_by<F>(self, f: F) -> Self::Ret<&'a FqxRow>
    where
        F: Fn(&&'a FqxRow) -> Vec<FqxValue>,
    {
        let mut res = HashMap::new();
        self.iter()
            .group_by(f)
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        FqxGroup(res)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSlice

impl<'a> OpGroup<Vec<FqxValue>, &'a FqxRow> for &'a FqxSlice {
    type Ret<A> = FqxGroup<Vec<A>>;

    fn group_by<F>(self, f: F) -> Self::Ret<&'a FqxRow>
    where
        F: Fn(&&'a FqxRow) -> Vec<FqxValue>,
    {
        let mut res = HashMap::new();
        self.0
            .iter()
            .group_by(f)
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        FqxGroup(res)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSelect

impl OpGroup<Vec<FqxValue>, FqxRowSelect<FqxValue>> for Vec<FqxRowSelect<FqxValue>> {
    type Ret<A> = FqxGroup<Vec<A>>;

    fn group_by<F>(self, f: F) -> Self::Ret<FqxRowSelect<FqxValue>>
    where
        F: Fn(&FqxRowSelect<FqxValue>) -> Vec<FqxValue>,
    {
        let mut res = HashMap::new();
        self.into_iter()
            .group_by(f)
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        FqxGroup(res)
    }
}

impl<'a> OpGroup<Vec<FqxValue>, &'a FqxRowSelect<&'a FqxValue>>
    for &'a Vec<FqxRowSelect<&'a FqxValue>>
{
    type Ret<A> = FqxGroup<Vec<A>>;

    fn group_by<F>(self, f: F) -> Self::Ret<&'a FqxRowSelect<&'a FqxValue>>
    where
        F: Fn(&&'a FqxRowSelect<&'a FqxValue>) -> Vec<FqxValue>,
    {
        let mut res = HashMap::new();
        self.iter()
            .group_by(f)
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        FqxGroup(res)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_group_by {

    use super::*;

    #[test]
    fn group_by_success() {
        let d = FqxData::new(
            &["c1", "c2", "c3"],
            vec![FqxValueType::F32, FqxValueType::String, FqxValueType::I32],
            vec![
                vec![
                    FqxValue::F32(1.1),
                    FqxValue::String("x".to_string()),
                    FqxValue::I32(1),
                ],
                vec![
                    FqxValue::F32(2.1),
                    FqxValue::String("y".to_string()),
                    FqxValue::I32(2),
                ],
                vec![
                    FqxValue::F32(1.1),
                    FqxValue::String("z".to_string()),
                    FqxValue::I32(1),
                ],
            ],
        )
        .unwrap();

        let foo = (&d).group_by(|r| vec![r[0].clone()]);
        println!("{:?}", foo);

        let foo = d.group_by(|r| vec![r[0].clone()]);
        println!("{:?}", foo);
    }
}
