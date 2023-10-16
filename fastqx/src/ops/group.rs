//! file: group.rs
//! author: Jacob Xie
//! date: 2023/09/22 20:06:22 Friday
//! brief:

use std::collections::HashMap;

use itertools::Itertools;
use ref_cast::RefCast;

use crate::adt::ab::d::{FqxD, PhantomU};
use crate::adt::*;

// ================================================================================================
// OpGroup
// ================================================================================================

pub trait OpGroup<K, T>
where
    Self: Sized,
    K: PartialEq,
{
    type Item;
    type Ret<A>;

    fn group_by<F>(self, f: F) -> Self::Ret<Self>
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

impl<U, C, T, I, E> OpGroup<Vec<FqxValue>, PhantomU<C, T, I, E>> for U
where
    Self: Sized,
    U: FqxD<C, T, I, E>,
    C: Clone,
    T: Clone,
    I: Default + Clone,
    I: IntoIterator<Item = E> + FromIterator<E>,
{
    type Item = I;

    type Ret<A> = FqxGroup<A>;

    fn group_by<F>(self, f: F) -> Self::Ret<Self>
    where
        F: Fn(&Self::Item) -> Vec<FqxValue>,
    {
        let (c, t, d) = self.dcst();

        let mut res = HashMap::new();
        Itertools::group_by(d.into_iter(), f)
            .into_iter()
            .for_each(|(k, g)| res.entry(k).or_insert(Vec::new()).extend(g.collect_vec()));

        let res = res
            .into_iter()
            .map(|(k, g)| (k, U::cst(c.clone(), t.clone(), g)))
            .collect::<HashMap<_, _>>();

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
    fn group_success() {
        let d = DATA.clone();

        let foo = d.rf().group_by(|r| vec![r[0].clone()]);
        println!("{:?}", foo);

        let foo = d.group_by(|r| vec![r[0].clone()]);
        println!("{:?}", foo);
    }
}
