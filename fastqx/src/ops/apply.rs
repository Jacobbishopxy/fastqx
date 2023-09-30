//! file: apply.rs
//! author: Jacob Xie
//! date: 2023/09/23 18:34:46 Saturday
//! brief:

use anyhow::Result;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::ops::{FqxRowSelect, FqxSlice};

// ================================================================================================
// OpApply & OpApplyMut
// ================================================================================================

pub trait OpApply<I> {
    fn apply<R, O, F>(self, f: F) -> R
    where
        F: Fn(I) -> O,
        R: FromIterator<O>;

    fn try_apply<R, O, F>(self, f: F) -> Result<R>
    where
        F: Fn(I) -> Result<O>,
        R: FromIterator<O>;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxData

impl OpApply<FqxRow> for FqxData {
    fn apply<R, O, F>(self, f: F) -> R
    where
        F: Fn(FqxRow) -> O,
        R: FromIterator<O>,
    {
        self.iter_owned().map(f).collect::<R>()
    }

    fn try_apply<R, O, F>(self, f: F) -> Result<R>
    where
        F: Fn(FqxRow) -> Result<O>,
        R: FromIterator<O>,
    {
        self.iter_owned().map(f).collect::<Result<R>>()
    }
}

impl<'a> OpApply<&'a FqxRow> for &'a FqxData {
    fn apply<R, O, F>(self, f: F) -> R
    where
        F: Fn(&'a FqxRow) -> O,
        R: FromIterator<O>,
    {
        self.iter().map(f).collect::<R>()
    }

    fn try_apply<R, O, F>(self, f: F) -> Result<R>
    where
        F: Fn(&'a FqxRow) -> Result<O>,
        R: FromIterator<O>,
    {
        self.iter().map(f).collect::<Result<R>>()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSlice

impl<'a> OpApply<&'a FqxRow> for &'a FqxSlice {
    fn apply<R, O, F>(self, f: F) -> R
    where
        F: Fn(&'a FqxRow) -> O,
        R: FromIterator<O>,
    {
        self.0.iter().map(f).collect::<R>()
    }

    fn try_apply<R, O, F>(self, f: F) -> Result<R>
    where
        F: Fn(&'a FqxRow) -> Result<O>,
        R: FromIterator<O>,
    {
        self.0.iter().map(f).collect::<Result<R>>()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Vec<FqxRowSelect>

impl OpApply<FqxRowSelect<FqxValue>> for Vec<FqxRowSelect<FqxValue>> {
    fn apply<R, O, F>(self, f: F) -> R
    where
        F: Fn(FqxRowSelect<FqxValue>) -> O,
        R: FromIterator<O>,
    {
        self.into_iter().map(f).collect::<R>()
    }

    fn try_apply<R, O, F>(self, f: F) -> Result<R>
    where
        F: Fn(FqxRowSelect<FqxValue>) -> Result<O>,
        R: FromIterator<O>,
    {
        self.into_iter().map(f).collect::<Result<R>>()
    }
}

impl<'a> OpApply<FqxRowSelect<&'a FqxValue>> for Vec<FqxRowSelect<&'a FqxValue>> {
    fn apply<R, O, F>(self, f: F) -> R
    where
        F: Fn(FqxRowSelect<&'a FqxValue>) -> O,
        R: FromIterator<O>,
    {
        self.into_iter().map(f).collect::<R>()
    }

    fn try_apply<R, O, F>(self, f: F) -> Result<R>
    where
        F: Fn(FqxRowSelect<&'a FqxValue>) -> Result<O>,
        R: FromIterator<O>,
    {
        self.into_iter().map(f).collect::<Result<R>>()
    }
}

impl<'a> OpApply<&'a FqxRowSelect<&'a FqxValue>> for &'a [FqxRowSelect<&'a FqxValue>] {
    fn apply<R, O, F>(self, f: F) -> R
    where
        F: Fn(&'a FqxRowSelect<&'a FqxValue>) -> O,
        R: FromIterator<O>,
    {
        self.iter().map(f).collect::<R>()
    }

    fn try_apply<R, O, F>(self, f: F) -> Result<R>
    where
        F: Fn(&'a FqxRowSelect<&'a FqxValue>) -> Result<O>,
        R: FromIterator<O>,
    {
        self.iter().map(f).collect::<Result<R>>()
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
    use crate::ops::OpSelect;

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

    #[test]
    fn apply_select_success() {
        let data = DATA.clone();

        let select = (&data).select(&[0, 2]);
        let foo = select.apply::<Vec<_>, _, _>(|s| s.0[0]);
        println!("{:?}", foo);

        let select = data.select(&[0, 2]);
        let foo = select.apply::<Vec<_>, _, _>(|s| s.0[0].clone());
        println!("{:?}", foo);
    }
}
