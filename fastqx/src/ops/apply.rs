//! file: apply.rs
//! author: Jacob Xie
//! date: 2023/09/23 18:34:46 Saturday
//! brief:

use anyhow::Result;

use crate::adt::{FqxRowAbstract, FqxValue};

// ================================================================================================
// OpApply & OpApplyMut
// ================================================================================================

pub trait OpApply<T> {
    type Item;

    fn apply<O, F>(self, f: F) -> Vec<O>
    where
        F: Fn(Self::Item) -> O;

    fn try_apply<O, F>(self, f: F) -> Result<Vec<O>>
    where
        F: Fn(Self::Item) -> Result<O>;
}

// ================================================================================================
// Impl
// Generic T
// Include `FqxData`, `&FqxData`, `Vec<FqxRow>`, `Vec<FqxRowSelect<&FqxValue>>` and etc.
// ================================================================================================

impl<I, V, T, E> OpApply<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    fn apply<O, F>(self, f: F) -> Vec<O>
    where
        F: Fn(Self::Item) -> O,
    {
        self.into_iter().map(|r| f(r)).collect::<Vec<_>>()
    }

    fn try_apply<O, F>(self, f: F) -> Result<Vec<O>>
    where
        F: Fn(Self::Item) -> Result<O>,
    {
        self.into_iter().map(|r| f(r)).collect::<Result<Vec<_>>>()
    }
}

impl<'a, I, V, T, E> OpApply<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>> + 'a,
{
    type Item = &'a E;

    fn apply<O, F>(self, f: F) -> Vec<O>
    where
        F: Fn(Self::Item) -> O,
    {
        self.into_iter().map(|r| f(r)).collect::<Vec<_>>()
    }

    fn try_apply<O, F>(self, f: F) -> Result<Vec<O>>
    where
        F: Fn(Self::Item) -> Result<O>,
    {
        self.into_iter().map(|r| f(r)).collect::<Result<Vec<_>>>()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_apply {
    use super::*;
    use crate::ops::OpSelect;

    use crate::mock::data::D1;

    #[test]
    fn apply_self_success() {
        let data = D1.clone();

        // &FqxData
        let foo = (&data).apply(|r| r[2].clone() * 2.into());
        println!("{:?}", foo);

        // FqxData
        let foo = data.apply(|r| r[2].clone() * 2.into());
        println!("{:?}", foo);
    }

    #[test]
    fn apply_slice_success() {
        let data = D1.clone();

        // &FqxSlice
        let slice = &data[1..3];
        let foo = slice.apply(|r| r[2].clone() + 10.into());

        println!("{:?}", foo);
    }

    #[test]
    fn apply_select_success() {
        let data = D1.clone();

        // Vec<FqxRowSelect<&FqxValue>>
        let select = (&data).select([0, 2].as_slice());
        let foo = select.apply(|s| s[0].to_owned() + s[1].to_owned());
        println!("{:?}", foo);

        // Vec<FqxRowSelect<FqxValue>>
        let select = data.select([0, 2, 1].as_slice());
        let foo = select.apply(|s| (s[0], s[2]));
        println!("{:?}", foo);
    }
}
