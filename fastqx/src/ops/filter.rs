//! file: filter.rs
//! author: Jacob Xie
//! date: 2023/10/08 20:51:52 Sunday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxRowAbstract, FqxValue};
use crate::ops::FqxGroup;

// ================================================================================================
// OpFilter
// ================================================================================================

pub trait OpFilter<T> {
    type Item;
    type Ret<A>;

    fn filter<F>(self, f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// Generic T

impl<I, V, T, E> OpFilter<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = Vec<A>;

    fn filter<F>(self, f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Iterator::filter(self.into_iter(), f).collect()
    }
}

impl<'a, I, V, T, E> OpFilter<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: Into<FqxRowAbstract<I, V>> + 'a,
{
    type Item = &'a E;

    type Ret<A> = Vec<A>;

    fn filter<F>(self, f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Iterator::filter(self.into_iter(), f).collect()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<I, V, T, E> OpFilter<FqxRowAbstract<I, V>> for FqxGroup<T>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = HashMap<Vec<FqxValue>, Vec<A>>;

    fn filter<F>(self, mut f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = Iterator::filter(v.into_iter(), &mut f).collect();
            res.insert(k, a);
        }

        res
    }
}

impl<'a, I, V, T, E> OpFilter<&'a FqxRowAbstract<I, V>> for &'a FqxGroup<T>
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>> + 'a,
{
    type Item = &'a E;

    type Ret<A> = HashMap<Vec<FqxValue>, Vec<A>>;

    fn filter<F>(self, mut f: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = Iterator::filter(v.into_iter(), &mut f).collect();
            res.insert(k.clone(), a);
        }

        res
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use super::*;
    use crate::adt::*;
    use crate::ops::{OpGroup, OpSelect};

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
    fn filter_self_success() {
        let data = DATA.clone();

        let foo = (&data).filter(|r| r[0] == FqxValue::I32(2));
        println!("{:?}", foo);

        let foo = data.filter(|r| r[0] == FqxValue::I32(2));
        println!("{:?}", foo);
    }

    #[test]
    fn filter_slice_success() {
        let data = DATA.clone();

        let slice = &data[..];

        let foo = slice.filter(|r| r[0] == FqxValue::I64(1));

        println!("{:?}", foo);
    }

    #[test]
    fn filter_group_success() {
        let data = DATA.clone();

        let foo = (&data).group_by(|r| vec![r[0].clone()]);
        let foo = foo.filter(|r| r[0] == FqxValue::I64(2));
        println!("{:?}", foo);

        let foo = data
            .group_by(|r| vec![r[0].clone()])
            .filter(|r| r[0] == FqxValue::I64(2));
        println!("{:?}", foo);
    }

    #[test]
    fn filter_selected_success() {
        let data = DATA.clone();

        let foo = (&data)
            .select(&[0, 1])
            .filter(|r| r[0] == &FqxValue::I64(2));
        println!("{:?}", foo);

        let foo = data.select(&[0, 1]).filter(|r| r[0] == FqxValue::I64(2));
        println!("{:?}", foo);
    }

    #[test]
    fn filter_selected_group_success() {
        let data = DATA.clone();

        let foo = (&data).select(&[0, 1]).group_by(|r| vec![r[0].clone()]);
        let foo = foo.filter(|r| r[0] == &FqxValue::I64(2));
        println!("{:?}", foo);

        let foo = data
            .select(&[0, 1])
            .group_by(|r| vec![r[0].clone()])
            .filter(|r| r[0] == FqxValue::I64(2));
        println!("{:?}", foo);
    }
}
