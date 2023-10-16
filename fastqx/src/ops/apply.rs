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
        let foo = (&data).apply(|r| r[2].clone() * 2.into());
        println!("{:?}", foo);

        // FqxData
        let foo = data.apply(|r| r[2].clone() * 2.into());
        println!("{:?}", foo);
    }

    #[test]
    fn apply_slice_success() {
        let data = DATA.clone();

        // &FqxSlice
        let slice = &data[1..3];
        let foo = slice.apply(|r| r[2].clone() + 10.into());

        println!("{:?}", foo);
    }

    #[test]
    fn apply_select_success() {
        let data = DATA.clone();

        // Vec<FqxRowSelect<&FqxValue>>
        let select = (&data).select([0, 2].as_slice());
        let foo = select.apply(|s| s[0].to_owned() + s[1].to_owned());
        println!("{:?}", foo);

        // Vec<FqxRowSelect<FqxValue>>
        let select = data.select([0, 2].as_slice());
        let foo = select.apply(|s| (s[0], s[2]));
        println!("{:?}", foo);
    }
}
