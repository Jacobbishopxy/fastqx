//! file: group.rs
//! author: Jacob Xie
//! date: 2023/09/22 20:06:22 Friday
//! brief:

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use ref_cast::RefCast;

use crate::adt::*;

// ================================================================================================
// FqxGroup
// ================================================================================================

// TODO: refactor

pub trait AlgoGroup<'a>
where
    Self: 'a,
{
    type IterItem;
    type Ret<R>;

    fn group_by<R, K, F>(&'a self, f: F) -> Self::Ret<R>
    where
        F: Fn(Self::IterItem) -> Self::Ret<R>;

    fn try_group_by<R, K, F>(&'a self, f: F) -> Result<Self::Ret<R>>
    where
        F: Fn(Self::IterItem) -> Result<Self::Ret<R>>;
}

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct FqxGroup<'a>(pub(crate) HashMap<FqxValue, Vec<&'a FqxRow>>);

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct FqxGroupMut<'a>(pub(crate) HashMap<FqxValue, Vec<&'a mut FqxRow>>);

// ================================================================================================
// group_by
// ================================================================================================

macro_rules! guard {
    ($s:expr, $ki:expr) => {
        let w = $s.width();
        if $ki >= w {
            return Err(anyhow!(format!(
                "Out of range, key_idx: {}, width: {w}",
                $ki
            )));
        }

        if $s.types[$ki].is_float() {
            return Err(anyhow!(
                "the selected column is float, cannot be use as an group_by key"
            ));
        }
    };
}

impl FqxData {
    pub fn group_by(&self, key_idx: usize) -> Result<FqxGroup> {
        guard!(self, key_idx);

        let mut map = HashMap::new();
        self.iter()
            .group_by(|r| r[key_idx].clone())
            .into_iter()
            .for_each(|(k, g)| map.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        Ok(FqxGroup(map))
    }

    pub fn group_by_mut(&mut self, key_idx: usize) -> Result<FqxGroupMut> {
        guard!(self, key_idx);

        let mut map = HashMap::new();
        self.iter_mut()
            .group_by(|r| r[key_idx].clone())
            .into_iter()
            .for_each(|(k, g)| map.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        Ok(FqxGroupMut(map))
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

        let foo = d.group_by(2).unwrap();

        println!("{:?}", foo);
    }
}
