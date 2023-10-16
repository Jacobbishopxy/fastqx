//! file: filter.rs
//! author: Jacob Xie
//! date: 2023/10/08 20:51:52 Sunday
//! brief:

use std::collections::HashMap;

use crate::adt::{FqxD, PhantomU};
use crate::ops::FqxGroup;

// ================================================================================================
// OpFilter
// ================================================================================================

pub trait OpFilter<T>
where
    Self: Sized,
{
    type Item;

    fn filter<F>(self, f: F) -> Self
    where
        F: FnMut(&Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// Generic T

impl<U, C, T, I, E> OpFilter<PhantomU<C, T, I, E>> for U
where
    Self: Sized,
    U: FqxD<C, T, I, E>,
    C: Clone,
    T: Clone,
    I: Default + Clone,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    type Item = I;

    fn filter<F>(self, f: F) -> Self
    where
        F: FnMut(&Self::Item) -> bool,
    {
        let (c, t, d) = self.dcst();

        let d = Iterator::filter(d.into_iter(), f).collect();

        U::cst(c, t, d)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup<T>

impl<U, C, T, I, E> OpFilter<FqxGroup<PhantomU<C, T, I, E>>> for FqxGroup<U>
where
    Self: Sized,
    U: FqxD<C, T, I, E>,
    C: Clone,
    T: Clone,
    I: Default + Clone,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    type Item = I;

    fn filter<F>(self, mut f: F) -> Self
    where
        F: FnMut(&Self::Item) -> bool,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let (c, t, d) = v.dcst();
            let d = Iterator::filter(d.into_iter(), &mut f).collect();
            res.insert(k, U::cst(c, t, d));
        }

        FqxGroup(res)
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

        let foo = data.rf().filter(|r| r[0] == &FqxValue::I32(2));
        println!("{:?}", foo);

        let foo = data.filter(|r| r[0] == FqxValue::I32(2));
        println!("{:?}", foo);
    }

    #[test]
    fn filter_group_success() {
        let data = DATA.clone();

        let foo = data.rf().group_by(|r| vec![r[0].clone()]);
        let foo = foo.filter(|r| r[0] == &FqxValue::I64(2));
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
            .select([0, 1].as_slice())
            .filter(|r| r[0] == &FqxValue::I64(2));
        println!("{:?}", foo);

        let foo = data
            .select([0, 1].as_slice())
            .filter(|r| r[0] == &FqxValue::I64(2));
        println!("{:?}", foo);
    }

    #[test]
    fn filter_selected_group_success() {
        let data = DATA.clone();

        let foo = (&data)
            .select([0, 1].as_slice())
            .group_by(|r| vec![r[0].clone()]);
        let foo = foo.filter(|r| r[0] == &FqxValue::I64(2));
        println!("{:?}", foo);

        let foo = data
            .select([0, 1].as_slice())
            .group_by(|r| vec![r[0].clone()])
            .filter(|r| r[0] == &FqxValue::I64(2));
        println!("{:?}", foo);
    }
}
