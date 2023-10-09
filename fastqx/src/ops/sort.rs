//! file: sort.rs
//! author: Jacob Xie
//! date: 2023/10/09 19:45:55 Monday
//! brief:

use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::adt::{FqxRowAbstract, FqxValue};
use crate::ops::FqxGroup;

// ================================================================================================
// OpSort
// ================================================================================================

pub trait OpSort<T> {
    type Item;
    type Ret<A>;

    fn sorted_by<F>(self, cmp: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

fn bool_to_ordering(b: bool) -> Ordering {
    if b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Generic T

impl<I, V, T, E> OpSort<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = Vec<Self::Item>;

    fn sorted_by<F>(self, mut cmp: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        Itertools::sorted_by(self.into_iter(), |p, c| bool_to_ordering(cmp(p, c))).collect()
    }
}

impl<'a, I, V, T, E> OpSort<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: Into<FqxRowAbstract<I, V>> + 'a,
{
    type Item = &'a E;

    type Ret<A> = Vec<Self::Item>;

    fn sorted_by<F>(self, mut cmp: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        Itertools::sorted_by(self.into_iter(), |p, c| bool_to_ordering(cmp(p, c))).collect()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<I, V, T, E> OpSort<FqxRowAbstract<I, V>> for FqxGroup<T>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = HashMap<Vec<FqxValue>, Vec<Self::Item>>;

    fn sorted_by<F>(self, mut cmp: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a =
                Itertools::sorted_by(v.into_iter(), |p, c| bool_to_ordering(cmp(p, c))).collect();
            res.insert(k, a);
        }

        res
    }
}

impl<'a, I, V, T, E> OpSort<&'a FqxRowAbstract<I, V>> for &'a FqxGroup<T>
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: Into<FqxRowAbstract<I, V>> + 'a,
{
    type Item = &'a E;

    type Ret<A> = HashMap<Vec<FqxValue>, Vec<Self::Item>>;

    fn sorted_by<F>(self, mut cmp: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a =
                Itertools::sorted_by(v.into_iter(), |p, c| bool_to_ordering(cmp(p, c))).collect();
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
    use crate::ops::{OpCloned, OpGroup, OpSelect};

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
    fn sort_self_success() {
        let data = DATA.clone();

        let foo = (&data).sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);

        let foo = data.sorted_by(|p, c| p[0] < c[0]);
        println!("{:?}", foo);
    }

    #[test]
    fn sort_slice_success() {
        let data = DATA.clone();

        let slice = &data[..];

        let foo = slice.sorted_by(|p, c| p[0] > c[0]);

        println!("{:?}", foo);
    }

    #[test]
    fn sort_group_success() {
        let data = DATA.clone();

        let foo = (&data)
            .group_by(|r| vec![r[0].clone()])
            .cloned()
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);

        let foo = data
            .group_by(|r| vec![r[0].clone()])
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);
    }

    #[test]
    fn sort_selected_success() {
        let data = DATA.clone();

        let foo = (&data)
            .select(&[0, 1])
            .cloned()
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);

        let foo = data.select(&[0, 1]).sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);
    }

    #[test]
    fn sort_selected_group_success() {
        let data = DATA.clone();

        let foo = data
            .select(&[0, 1])
            .group_by(|r| vec![r[0].clone()])
            .sorted_by(|p, c| p[0] > c[0]);
        println!("{:?}", foo);
    }
}
