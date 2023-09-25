//! file: group.rs
//! author: Jacob Xie
//! date: 2023/09/22 20:06:22 Friday
//! brief:

use std::collections::HashMap;

use itertools::Itertools;
use ref_cast::RefCast;

use crate::adt::*;

// ================================================================================================
// AlgoGroup
// ================================================================================================

pub trait AlgoGroup<'a>
where
    Self: 'a,
{
    type IterItem;
    type Key: PartialEq;
    type Ret;

    fn group_by<F>(&'a self, f: F) -> Self::Ret
    where
        F: Fn(Self::IterItem) -> Self::Key;
}

// ================================================================================================
// FqxGroup
// ================================================================================================

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct FqxGroup<'a>(pub(crate) HashMap<FqxValue, Vec<&'a FqxRow>>);

// ================================================================================================
// Impl
// ================================================================================================

impl<'a> AlgoGroup<'a> for FqxData {
    type IterItem = &'a FqxRow;
    type Key = FqxValue;
    type Ret = FqxGroup<'a>;

    fn group_by<F>(&'a self, f: F) -> Self::Ret
    where
        F: Fn(Self::IterItem) -> Self::Key,
    {
        let mut res = HashMap::new();
        self.iter()
            .group_by(|k| f(*k))
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

        let foo = d.group_by(|r| r[0].clone());

        println!("{:?}", foo);
    }
}
