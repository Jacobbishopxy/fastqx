//! file: fold.rs
//! author: Jacob Xie
//! date: 2023/09/24 18:50:53 Sunday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxRow, FqxRowAbstract, FqxValue};
use crate::ops::FqxGroup;

// ================================================================================================
// OpFold
// ================================================================================================

pub trait OpFold<I> {
    fn fold<A, F>(self, accumulator: A, f: F) -> A
    where
        A: Clone,
        F: FnMut(A, I) -> A;

    fn try_fold<A, F>(self, accumulator: A, f: F) -> Result<A>
    where
        A: Clone,
        F: FnMut(A, I) -> Result<A>;
}

// OpFold for `OpGroup`
pub trait OpFoldGroup<I> {
    type Ret<A>;

    fn fold<A, F>(self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, I) -> A;

    fn try_fold<A, F>(self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, I) -> Result<A>;
}

// ================================================================================================
// OnFoldFqxRow
// specified type impl
// ================================================================================================

pub trait OpFoldFqxRow<I, V>
where
    Self: OpFold<I>,
    Self: Sized,
{
    fn fold_fqx_row<F>(self, accumulator: FqxRow, f: F) -> FqxRow
    where
        F: FnMut(FqxValue, FqxValue) -> FqxValue;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<I, V, T, E> OpFoldFqxRow<FqxRowAbstract<I, V>, FqxValue> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    fn fold_fqx_row<F>(self, accumulator: FqxRow, mut f: F) -> FqxRow
    where
        F: FnMut(FqxValue, FqxValue) -> FqxValue,
    {
        self.fold(accumulator, |acc, row| {
            let inner = acc
                .into_iter()
                .zip(row.0.into_iter())
                .map(|(p, c)| f(p, c.into()))
                .collect::<Vec<_>>();
            FqxRow(inner)
        })
    }
}

impl<'a, I, V, T, E> OpFoldFqxRow<&'a FqxRowAbstract<I, V>, &'a FqxValue> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    for<'b> &'b I: IntoIterator<Item = &'b V>,
    V: Into<FqxValue> + 'a,
    for<'b> FqxValue: From<&'b V>,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>>,
{
    fn fold_fqx_row<F>(self, accumulator: FqxRow, mut f: F) -> FqxRow
    where
        F: FnMut(FqxValue, FqxValue) -> FqxValue,
    {
        self.fold(accumulator, |acc, ref row| {
            let inner = acc
                .into_iter()
                .zip((&row.0).into_iter())
                .map(|(p, c)| f(p, c.into()))
                .collect::<Vec<_>>();
            FqxRow(inner)
        })
    }
}

// ================================================================================================
// Impl
// ================================================================================================

impl<I, V, T, E> OpFold<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    fn fold<A, F>(self, accumulator: A, mut f: F) -> A
    where
        A: Clone,
        F: FnMut(A, FqxRowAbstract<I, V>) -> A,
    {
        Iterator::fold(self.into_iter(), accumulator, |acc, r| f(acc, r.into()))
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<A>
    where
        A: Clone,
        F: FnMut(A, FqxRowAbstract<I, V>) -> Result<A>,
    {
        Iterator::try_fold(&mut self.into_iter(), accumulator, |acc, r| {
            f(acc, r.into())
        })
    }
}

impl<'a, I, V, T, E> OpFold<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>>,
{
    fn fold<A, F>(self, accumulator: A, mut f: F) -> A
    where
        A: Clone,
        F: FnMut(A, &'a FqxRowAbstract<I, V>) -> A,
    {
        self.into_iter()
            .fold(accumulator, |acc, r| f(acc, r.as_ref()))
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<A>
    where
        A: Clone,
        F: FnMut(A, &'a FqxRowAbstract<I, V>) -> Result<A>,
    {
        self.into_iter()
            .try_fold(accumulator, |acc, r| f(acc, r.as_ref()))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup

impl<I, V, T, E> OpFoldGroup<FqxRowAbstract<I, V>> for FqxGroup<T>
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Ret<A> = HashMap<Vec<FqxValue>, A>;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, FqxRowAbstract<I, V>) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = Iterator::fold(v.into_iter(), accumulator.clone(), |acc, r| {
                f(acc, r.into())
            });
            res.insert(k, a);
        }

        res
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, FqxRowAbstract<I, V>) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = Iterator::try_fold(&mut v.into_iter(), accumulator.clone(), |acc, r| {
                f(acc, r.into())
            })?;
            res.insert(k, a);
        }

        Ok(res)
    }
}

impl<'a, I, V, T, E> OpFoldGroup<&'a FqxRowAbstract<I, V>> for &'a FqxGroup<T>
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>>,
{
    type Ret<A> = HashMap<Vec<FqxValue>, A>;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, &'a FqxRowAbstract<I, V>) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = Iterator::fold(v.into_iter(), accumulator.clone(), |acc, r| {
                f(acc, r.as_ref())
            });
            res.insert(k.clone(), a);
        }

        res
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, &'a FqxRowAbstract<I, V>) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in (&self.0).into_iter() {
            let a = Iterator::try_fold(&mut v.into_iter(), accumulator.clone(), |acc, r| {
                f(acc, r.as_ref())
            })?;
            res.insert(k.clone(), a);
        }

        Ok(res)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_fold {
    use once_cell::sync::Lazy;

    use super::*;
    use crate::adt::*;
    use crate::ops::OpGroup;
    use crate::prelude::OpSelect;

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
    fn fold_self_success() {
        let data = DATA.clone();

        let foo = (&data).fold(vec![], |mut acc, r| {
            acc.push(r[1].clone());

            acc
        });
        println!("{:?}", foo);

        let foo = data.fold(FqxValue::F32(0f32), |mut acc, r| {
            acc += r[2].clone();

            acc
        });
        println!("{:?}", foo);
    }

    #[test]
    fn fold_slice_success() {
        let data = DATA.clone();

        let slice = &data[1..3];

        let foo = slice.fold(vec![], |mut acc, r| {
            acc.push(r[1].clone());

            acc
        });
        println!("{:?}", foo);
    }

    #[test]
    fn fold_group_success() {
        let data = DATA.clone();

        let foo = (&data)
            .group_by(|r| vec![r[0].clone()])
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);

        let foo = data.group_by(|r| vec![r[0].clone()]).fold(
            String::new(),
            |mut acc, r: FqxRowAbstract<_, _>| {
                acc.push_str(&r[1].to_string());

                acc
            },
        );
        println!("{:?}", foo);
    }

    #[test]
    fn fold_selected_group_success() {
        let data = DATA.clone();

        let foo = (&data)
            .select(&[0, 1])
            .group_by(|r| vec![r[0].clone()])
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);

        let foo = data.select(&[0, 1]).group_by(|r| vec![r[0].clone()]).fold(
            String::new(),
            |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            },
        );
        println!("{:?}", foo);
    }
}
