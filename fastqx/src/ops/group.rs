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

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T, V> FqxGroup<T>
where
    for<'b> &'b T: IntoIterator<Item = &'b V>,
    V: Into<FqxRow> + Clone,
{
    pub fn cloned(&self) -> FqxGroup<Vec<FqxRow>> {
        let inner = (&self.0)
            .into_iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.into_iter().cloned().map(|e| e.into()).collect(),
                )
            })
            .collect::<HashMap<_, _>>();

        FqxGroup(inner)
    }
}

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

    use super::*;

    #[test]
    fn group_by_success() {
        let d = FqxData::new(
            &["c1", "c2", "c3"],
            vec![FqxValueType::F32, FqxValueType::String, FqxValueType::I32],
            vec![
                vec![
                    FqxValue::F32(1.1),
                    FqxValue::String("x".to_string()),
                    FqxValue::I32(1),
                ],
                vec![
                    FqxValue::F32(2.1),
                    FqxValue::String("y".to_string()),
                    FqxValue::I32(2),
                ],
                vec![
                    FqxValue::F32(1.1),
                    FqxValue::String("z".to_string()),
                    FqxValue::I32(1),
                ],
            ],
        )
        .unwrap();

        let foo = (&d).group_by(|r| vec![r[0].clone()]);
        println!("{:?}", foo);

        let foo = d.group_by(|r| vec![r[0].clone()]);
        println!("{:?}", foo);
    }
}
