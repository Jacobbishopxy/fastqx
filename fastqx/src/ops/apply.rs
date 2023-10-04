//! file: apply.rs
//! author: Jacob Xie
//! date: 2023/09/23 18:34:46 Saturday
//! brief:

use anyhow::Result;

use crate::adt::{FqxRow, FqxRowAbstract, FqxValue};
use crate::ops::FqxSlice;

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
// FqxSlice
// Since `FqxSlice` is not Sized, we can't treat it as generic T implementation.

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
// Generic T
// Include `FqxData`, `&FqxData`, `Vec<FqxRow>`, `Vec<FqxRowSelect<&FqxValue>>` and etc.

impl<I, V, T, A> OpApply<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = A>,
    A: Into<FqxRowAbstract<I, V>>,
{
    fn apply<R, O, F>(self, f: F) -> R
    where
        F: Fn(FqxRowAbstract<I, V>) -> O,
        R: FromIterator<O>,
    {
        self.into_iter().map(|r| f(r.into())).collect::<R>()
    }

    fn try_apply<R, O, F>(self, f: F) -> Result<R>
    where
        F: Fn(FqxRowAbstract<I, V>) -> Result<O>,
        R: FromIterator<O>,
    {
        self.into_iter().map(|r| f(r.into())).collect::<Result<R>>()
    }
}

impl<'a, I, V, T, A> OpApply<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    for<'b> &'b T: IntoIterator<Item = &'b A>,
    A: AsRef<FqxRowAbstract<I, V>>,
{
    fn apply<R, O, F>(self, f: F) -> R
    where
        F: Fn(&'a FqxRowAbstract<I, V>) -> O,
        R: FromIterator<O>,
    {
        self.into_iter().map(|r| f(r.as_ref())).collect::<R>()
    }

    fn try_apply<R, O, F>(self, f: F) -> Result<R>
    where
        F: Fn(&'a FqxRowAbstract<I, V>) -> Result<O>,
        R: FromIterator<O>,
    {
        self.into_iter()
            .map(|r| f(r.as_ref()))
            .collect::<Result<R>>()
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

        // &FqxData
        let foo = (&data).apply::<Vec<_>, _, _>(|r| r[2].clone() * 2.into());
        println!("{:?}", foo);

        // FqxData
        let foo = data.apply::<Vec<_>, _, _>(|r| r[2].clone() * 2.into());
        println!("{:?}", foo);
    }

    #[test]
    fn apply_slice_success() {
        let data = DATA.clone();

        // &FqxSlice
        let slice = &data[1..3];
        let foo = slice.apply::<Vec<_>, _, _>(|r| r[2].clone() + 10.into());

        println!("{:?}", foo);
    }

    #[test]
    fn apply_select_success() {
        let data = DATA.clone();

        // Vec<FqxRowSelect<&FqxValue>>
        let select = (&data).select(&[0, 2]);
        let foo = select.apply::<Vec<_>, _, _>(|s| s.0[0]);
        println!("{:?}", foo);

        // Vec<FqxRowSelect<FqxValue>>
        let select = data.select(&[0, 2]);
        let foo = select.apply::<Vec<_>, _, _>(|s| s.0[0].clone());
        println!("{:?}", foo);
    }
}
