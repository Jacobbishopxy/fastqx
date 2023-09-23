//! file: apply.rs
//! author: Jacob Xie
//! date: 2023/09/23 18:34:46 Saturday
//! brief:

use anyhow::Result;

use crate::adt::{FqxData, FqxRow};
use crate::algo::FqxGroupMut;

// ================================================================================================
// AlgoApply
// ================================================================================================

pub trait AlgoApply: Sized {
    fn apply_mut(&mut self, apply_fn: &dyn (Fn(&mut FqxRow) -> Result<()>)) -> Result<()>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl AlgoApply for FqxData {
    fn apply_mut(&mut self, apply_fn: &dyn (Fn(&mut FqxRow) -> Result<()>)) -> Result<()> {
        self.iter_mut().try_for_each(|r| apply_fn(r))?;

        Ok(())
    }
}

impl<'a> AlgoApply for FqxGroupMut<'a> {
    fn apply_mut(&mut self, apply_fn: &dyn (Fn(&mut FqxRow) -> Result<()>)) -> Result<()> {
        for (_, g) in self.0.iter_mut() {
            g.iter_mut().try_for_each(|r| apply_fn(*r))?;
        }

        Ok(())
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_apply {
    use once_cell::sync::Lazy;

    use super::*;
    use crate::adt::*;

    static DATA: Lazy<FqxData> = Lazy::new(|| {
        FqxData::new(
            vec![String::from("c1"), String::from("c2"), String::from("c3")],
            vec![FqxValueType::I32, FqxValueType::String, FqxValueType::F32],
            vec![
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("A")),
                    FqxValue::F32(2.1),
                ],
                vec![
                    FqxValue::I32(2),
                    FqxValue::String(String::from("B")),
                    FqxValue::F32(1.3),
                ],
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("C")),
                    FqxValue::F32(3.2),
                ],
            ],
        )
        .unwrap()
    });

    fn apy(row: &mut FqxRow) -> Result<()> {
        row.apply(2, &|_r| {
            // *r = r * 2;
            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn apply_self_success() {
        let mut data = DATA.clone();

        let res = data.apply_mut(&apy);
        println!("{:?}", data);

        assert!(res.is_ok());
    }

    #[test]
    fn apply_group_success() {
        let mut data = DATA.clone();

        let res = data.group_by_mut(0).unwrap().apply_mut(&apy);
        assert!(res.is_ok());
    }
}
