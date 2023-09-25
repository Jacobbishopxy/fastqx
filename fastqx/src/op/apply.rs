//! file: apply.rs
//! author: Jacob Xie
//! date: 2023/09/23 18:34:46 Saturday
//! brief:

use anyhow::Result;

use crate::adt::{FqxData, FqxRow};
use crate::op::{FqxRowSelect, FqxSlice};

// ================================================================================================
// OpApply & OpApplyMut
// ================================================================================================

pub trait OpApply<'a, II> {
    fn apply<R, I, F>(&'a self, f: F) -> R
    where
        F: Fn(II) -> I,
        R: FromIterator<I>;

    fn try_apply<R, I, F>(&'a self, f: F) -> Result<R>
    where
        F: Fn(II) -> Result<I>,
        R: FromIterator<I>;
}

pub trait OpApplyMut<'a, II> {
    fn apply<I, F>(&'a mut self, f: F)
    where
        F: FnMut(II);

    fn try_apply<I, F>(&'a mut self, f: F) -> Result<()>
    where
        F: FnMut(II) -> Result<()>;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxData

impl<'a> OpApply<'a, &'a FqxRow> for FqxData {
    fn apply<R, I, F>(&'a self, f: F) -> R
    where
        F: Fn(&'a FqxRow) -> I,
        R: FromIterator<I>,
    {
        self.iter().map(f).collect::<R>()
    }

    fn try_apply<R, I, F>(&'a self, f: F) -> Result<R>
    where
        F: Fn(&'a FqxRow) -> Result<I>,
        R: FromIterator<I>,
    {
        self.iter().map(f).collect::<Result<R>>()
    }
}

impl<'a> OpApplyMut<'a, &'a mut FqxRow> for FqxData {
    fn apply<I, F>(&'a mut self, f: F)
    where
        F: FnMut(&'a mut FqxRow),
    {
        self.iter_mut().for_each(f)
    }

    fn try_apply<I, F>(&'a mut self, f: F) -> Result<()>
    where
        F: FnMut(&'a mut FqxRow) -> Result<()>,
    {
        self.iter_mut().try_for_each(f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSlice

impl<'a> OpApply<'a, &'a FqxRow> for FqxSlice {
    fn apply<R, I, F>(&'a self, f: F) -> R
    where
        F: Fn(&'a FqxRow) -> I,
        R: FromIterator<I>,
    {
        self.0.iter().map(f).collect::<R>()
    }

    fn try_apply<R, I, F>(&'a self, f: F) -> Result<R>
    where
        F: Fn(&'a FqxRow) -> Result<I>,
        R: FromIterator<I>,
    {
        self.0.iter().map(f).collect::<Result<R>>()
    }
}

impl<'a> OpApplyMut<'a, &'a mut FqxRow> for FqxSlice {
    fn apply<I, F>(&'a mut self, f: F)
    where
        F: FnMut(&'a mut FqxRow),
    {
        self.0.iter_mut().for_each(f)
    }

    fn try_apply<I, F>(&'a mut self, f: F) -> Result<()>
    where
        F: FnMut(&'a mut FqxRow) -> Result<()>,
    {
        self.0.iter_mut().try_for_each(f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Vec<FqxRowSelect>

impl<'a> OpApply<'a, &'a FqxRowSelect<'a>> for Vec<FqxRowSelect<'a>> {
    fn apply<R, I, F>(&'a self, f: F) -> R
    where
        F: Fn(&'a FqxRowSelect<'a>) -> I,
        R: FromIterator<I>,
    {
        self.iter().map(f).collect::<R>()
    }

    fn try_apply<R, I, F>(&'a self, f: F) -> Result<R>
    where
        F: Fn(&'a FqxRowSelect<'a>) -> Result<I>,
        R: FromIterator<I>,
    {
        self.iter().map(f).collect::<Result<R>>()
    }
}

impl<'a> OpApplyMut<'a, &'a mut FqxRowSelect<'a>> for Vec<FqxRowSelect<'a>> {
    fn apply<I, F>(&'a mut self, f: F)
    where
        F: FnMut(&'a mut FqxRowSelect<'a>),
    {
        self.iter_mut().for_each(f)
    }

    fn try_apply<I, F>(&'a mut self, f: F) -> Result<()>
    where
        F: FnMut(&'a mut FqxRowSelect<'a>) -> Result<()>,
    {
        self.iter_mut().try_for_each(f)
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
    use crate::op::OpSelect;

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

    #[test]
    fn apply_select_success() {
        let data = DATA.clone();

        let select = data.select(&[0, 2]);

        let foo = select.apply::<Vec<_>, _, _>(|s| s.0[1]);

        println!("{:?}", foo);
    }
}
