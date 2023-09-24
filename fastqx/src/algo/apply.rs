//! file: apply.rs
//! author: Jacob Xie
//! date: 2023/09/23 18:34:46 Saturday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::algo::{FqxGroup, FqxGroupMut, FqxSlice};

// ================================================================================================
// AlgoApply & AlgoApplyMut
// ================================================================================================

pub trait AlgoApply {
    type Ret;

    fn apply(&self, apply_fn: &dyn Fn(&FqxRow) -> FqxRow) -> Self::Ret;

    fn try_apply(&self, apply_fn: &dyn Fn(&FqxRow) -> Result<FqxRow>) -> Result<Self::Ret>;
}

pub trait AlgoApplyMut {
    fn apply(&mut self, apply_fn: &dyn Fn(&mut FqxRow));

    fn try_apply(&mut self, apply_fn: &dyn Fn(&mut FqxRow) -> Result<()>) -> Result<()>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl AlgoApply for FqxData {
    type Ret = Vec<FqxRow>;

    fn apply(&self, apply_fn: &dyn Fn(&FqxRow) -> FqxRow) -> Self::Ret {
        self.iter_ref().map(apply_fn).collect::<Vec<_>>()
    }

    fn try_apply(&self, apply_fn: &dyn Fn(&FqxRow) -> Result<FqxRow>) -> Result<Self::Ret> {
        self.iter_ref().map(apply_fn).collect::<Result<Vec<_>>>()
    }
}

impl AlgoApplyMut for FqxData {
    fn apply(&mut self, apply_fn: &dyn Fn(&mut FqxRow)) {
        self.iter_mut().for_each(apply_fn);
    }

    fn try_apply(&mut self, apply_fn: &dyn Fn(&mut FqxRow) -> Result<()>) -> Result<()> {
        self.iter_mut().try_for_each(apply_fn)?;

        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl AlgoApply for FqxSlice {
    type Ret = Vec<FqxRow>;

    fn apply(&self, apply_fn: &dyn Fn(&FqxRow) -> FqxRow) -> Self::Ret {
        self.0
            .iter()
            .map(|r| apply_fn(r.as_ref()))
            .collect::<Vec<_>>()
    }

    fn try_apply(&self, apply_fn: &dyn Fn(&FqxRow) -> Result<FqxRow>) -> Result<Self::Ret> {
        self.0
            .iter()
            .map(|r| apply_fn(r.as_ref()))
            .collect::<Result<Vec<_>>>()
    }
}

impl AlgoApplyMut for FqxSlice {
    fn apply(&mut self, apply_fn: &dyn Fn(&mut FqxRow)) {
        self.0.iter_mut().for_each(|r| apply_fn(r.as_mut()))
    }

    fn try_apply(&mut self, apply_fn: &dyn Fn(&mut FqxRow) -> Result<()>) -> Result<()> {
        self.0.iter_mut().try_for_each(|r| apply_fn(r.as_mut()))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> AlgoApply for FqxGroup<'a> {
    type Ret = HashMap<FqxValue, Vec<FqxRow>>;

    fn apply(&self, apply_fn: &dyn Fn(&FqxRow) -> FqxRow) -> Self::Ret {
        let mut res = HashMap::new();

        for (k, g) in self.0.iter() {
            let v = g.iter().map(|r| apply_fn(*r)).collect::<Vec<_>>();
            res.insert(k.clone(), v);
        }

        res
    }

    fn try_apply(&self, apply_fn: &dyn Fn(&FqxRow) -> Result<FqxRow>) -> Result<Self::Ret> {
        let mut res = HashMap::new();

        for (k, g) in self.0.iter() {
            let v = g.iter().map(|r| apply_fn(*r)).collect::<Result<Vec<_>>>()?;
            res.insert(k.clone(), v);
        }

        Ok(res)
    }
}

impl<'a> AlgoApplyMut for FqxGroupMut<'a> {
    fn apply(&mut self, apply_fn: &dyn Fn(&mut FqxRow)) {
        for (_, g) in self.0.iter_mut() {
            g.iter_mut().for_each(|r| apply_fn(*r));
        }
    }

    fn try_apply(&mut self, apply_fn: &dyn Fn(&mut FqxRow) -> Result<()>) -> Result<()> {
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
        row.apply(2, &|r| {
            *r *= 2.into();
            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn apply_self_success() {
        let mut data = DATA.clone();

        let res = (&mut data).try_apply(&apy);
        println!("{:?}", data);

        assert!(res.is_ok());
    }

    #[test]
    fn apply_group_success() {
        let mut data = DATA.clone();

        let res = data.group_by_mut(0).unwrap().try_apply(&apy);
        assert!(res.is_ok());
    }
}
