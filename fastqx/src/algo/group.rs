//! file: group.rs
//! author: Jacob Xie
//! date: 2023/09/22 20:06:22 Friday
//! brief:

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::adt::*;

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
    };
}

impl FqxData {
    pub fn group_by(&self, key_idx: usize) -> Result<HashMap<FqxValue, Vec<&FqxRow>>> {
        guard!(self, key_idx);

        let mut map = HashMap::new();
        self.iter_ref()
            .group_by(|r| r[key_idx].clone())
            .into_iter()
            .for_each(|(k, g)| map.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        Ok(map)
    }

    pub fn group_by_mut(&mut self, key_idx: usize) -> Result<HashMap<FqxValue, Vec<&mut FqxRow>>> {
        guard!(self, key_idx);

        let mut map = HashMap::new();
        self.iter_mut()
            .group_by(|r| r[key_idx].clone())
            .into_iter()
            .for_each(|(k, g)| map.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        Ok(map)
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
