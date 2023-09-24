//! file: apply.rs
//! author: Jacob Xie
//! date: 2023/09/23 18:34:46 Saturday
//! brief:

use anyhow::Result;

use crate::adt::{FqxData, FqxRow};
use crate::algo::FqxSlice;

// ================================================================================================
// AlgoApply & AlgoApplyMut
// ================================================================================================

pub trait AlgoApply<'a> {
    type IterItem;

    fn apply<R, I, F>(&'a self, f: F) -> R
    where
        F: Fn(Self::IterItem) -> I,
        R: FromIterator<I>;

    fn try_apply<R, I, F>(&'a self, f: F) -> Result<R>
    where
        F: Fn(Self::IterItem) -> Result<I>,
        R: FromIterator<I>;
}

pub trait AlgoApplyMut<'a> {
    type IterItem;

    fn apply<I, F>(&'a mut self, f: F)
    where
        F: FnMut(Self::IterItem);

    fn try_apply<I, F>(&'a mut self, f: F) -> Result<()>
    where
        F: FnMut(Self::IterItem) -> Result<()>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<'a> AlgoApply<'a> for FqxData {
    type IterItem = &'a FqxRow;

    fn apply<R, I, F>(&'a self, f: F) -> R
    where
        F: Fn(Self::IterItem) -> I,
        R: FromIterator<I>,
    {
        self.iter().map(f).collect::<R>()
    }

    fn try_apply<R, I, F>(&'a self, f: F) -> Result<R>
    where
        F: Fn(Self::IterItem) -> Result<I>,
        R: FromIterator<I>,
    {
        self.iter().map(f).collect::<Result<R>>()
    }
}

impl<'a> AlgoApplyMut<'a> for FqxData {
    type IterItem = &'a mut FqxRow;

    fn apply<I, F>(&'a mut self, f: F)
    where
        F: FnMut(Self::IterItem),
    {
        self.iter_mut().for_each(f)
    }

    fn try_apply<I, F>(&'a mut self, f: F) -> Result<()>
    where
        F: FnMut(Self::IterItem) -> Result<()>,
    {
        self.iter_mut().try_for_each(f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> AlgoApply<'a> for FqxSlice {
    type IterItem = &'a FqxRow;

    fn apply<R, I, F>(&'a self, f: F) -> R
    where
        F: Fn(Self::IterItem) -> I,
        R: FromIterator<I>,
    {
        self.0.iter().map(f).collect::<R>()
    }

    fn try_apply<R, I, F>(&'a self, f: F) -> Result<R>
    where
        F: Fn(Self::IterItem) -> Result<I>,
        R: FromIterator<I>,
    {
        self.0.iter().map(f).collect::<Result<R>>()
    }
}

impl<'a> AlgoApplyMut<'a> for FqxSlice {
    type IterItem = &'a mut FqxRow;

    fn apply<I, F>(&'a mut self, f: F)
    where
        F: FnMut(Self::IterItem),
    {
        self.0.iter_mut().for_each(f)
    }

    fn try_apply<I, F>(&'a mut self, f: F) -> Result<()>
    where
        F: FnMut(Self::IterItem) -> Result<()>,
    {
        self.0.iter_mut().try_for_each(f)
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

    #[test]
    fn apply_self_success() {
        let data = DATA.clone();

        let foo = data.apply::<Vec<_>, _, _>(|r| r[2].clone() * 2.into());

        println!("{:?}", foo);
    }

    #[test]
    fn apply_slice_success() {
        let data = DATA.clone();

        let slice = &data[1..3];

        let foo = slice.apply::<Vec<_>, _, _>(|r| r[2].clone() + 10.into());

        println!("{:?}", foo);
    }

    fn apy(row: &mut FqxRow) -> Result<()> {
        row[2] = row[2].clone() - 10.into();

        Ok(())
    }

    #[test]
    fn apply_self_mut_success() {
        let mut data = DATA.clone();

        let foo = (&mut data).try_apply::<&mut FqxRow, _>(apy);

        assert!(foo.is_ok());

        println!("{:?}", data);
    }

    #[test]
    fn apply_slice_mut_success() {
        let mut data = DATA.clone();

        let slice = &mut data[1..3];

        let foo = slice.try_apply::<&mut FqxRow, _>(apy);

        assert!(foo.is_ok());

        println!("{:?}", data);
    }
}
