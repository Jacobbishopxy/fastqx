//! file: group.rs
//! author: Jacob Xie
//! date: 2023/09/22 20:06:22 Friday
//! brief:

use std::collections::HashMap;

use itertools::Itertools;
use ref_cast::RefCast;

use crate::adt::*;

// ================================================================================================
// OpGroup
// ================================================================================================

pub trait OpGroup<K, T>
where
    K: PartialEq,
{
    type Item;
    type Ret<A>;

    fn group_by<F>(self, f: F) -> Self::Ret<Self::Item>
    where
        F: Fn(&Self::Item) -> K;
}

// ================================================================================================
// FqxGroup
// ================================================================================================

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct FqxGroup<T>(pub(crate) HashMap<Vec<FqxValue>, T>);

// ================================================================================================
// Impl
// ================================================================================================

impl<I, V, T, E> OpGroup<Vec<FqxValue>, FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = FqxGroup<Vec<A>>;

    fn group_by<F>(self, f: F) -> Self::Ret<Self::Item>
    where
        F: Fn(&Self::Item) -> Vec<FqxValue>,
    {
        let mut res = HashMap::new();
        Itertools::group_by(self.into_iter(), f)
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        FqxGroup(res)
    }
}

impl<'a, I, V, T, E> OpGroup<Vec<FqxValue>, &'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>> + 'a,
{
    type Item = &'a E;

    type Ret<A> = FqxGroup<Vec<A>>;

    fn group_by<F>(self, f: F) -> Self::Ret<Self::Item>
    where
        F: Fn(&Self::Item) -> Vec<FqxValue>,
    {
        let mut res = HashMap::new();
        Itertools::group_by(self.into_iter(), f)
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        FqxGroup(res)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_group_by {
    use once_cell::sync::Lazy;

    use super::*;
    use crate::ops::OpGroup;

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
    fn group_success() {
        let d = DATA.clone();

        let foo = (&d).group_by(|r| vec![r[0].clone()]);
        println!("{:?}", foo);

        let foo = d.group_by(|r| vec![r[0].clone()]);
        println!("{:?}", foo);
    }
}
