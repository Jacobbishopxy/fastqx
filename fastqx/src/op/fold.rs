//! file: fold.rs
//! author: Jacob Xie
//! date: 2023/09/24 18:50:53 Sunday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::op::{FqxGroup, FqxRowSelect, FqxSlice};

// ================================================================================================
// OpFold
// ================================================================================================

pub trait OpFold<I> {
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
    Self: OpFold<I, Ret<FqxRow> = FqxRow>,
    Self: Sized,
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
{
    fn fold_fqx_row<F>(self, accumulator: FqxRow, mut f: F) -> FqxRow
    where
        F: FnMut(FqxValue, FqxValue) -> FqxValue,
    {
        self.fold(accumulator, |acc, row| {
            let inner = acc
                .into_iter()
                .zip(row.into_iter())
                .map(|(p, c)| f(p, c.into()))
                .collect::<Vec<_>>();
            FqxRow(inner)
        })
    }
}

impl OpFoldFqxRow<FqxRow, FqxValue> for FqxData {}

impl<'a> OpFoldFqxRow<&'a FqxRow, &'a FqxValue> for &'a FqxData {}

impl<'a> OpFoldFqxRow<&'a FqxRow, &'a FqxValue> for &'a FqxSlice {}

impl OpFoldFqxRow<FqxRowSelect<FqxValue>, FqxValue> for Vec<FqxRowSelect<FqxValue>> {}

impl<'a> OpFoldFqxRow<FqxRowSelect<&'a FqxValue>, &'a FqxValue>
    for Vec<FqxRowSelect<&'a FqxValue>>
{
}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxData

impl OpFold<FqxRow> for FqxData {
    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: FnMut(A, FqxRow) -> A,
    {
        self.iter_owned().fold(accumulator, f)
    }

    fn try_fold<A, F>(self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: FnMut(A, FqxRow) -> Result<A>,
    {
        self.iter_owned().try_fold(accumulator, f)
    }
}

impl<'a> OpFold<&'a FqxRow> for &'a FqxData {
    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: FnMut(A, &'a FqxRow) -> A,
    {
        self.iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: FnMut(A, &'a FqxRow) -> Result<A>,
    {
        self.iter().try_fold(accumulator, f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSlice

impl<'a> OpFold<&'a FqxRow> for &'a FqxSlice {
    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: FnMut(A, &'a FqxRow) -> A,
    {
        self.0.iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: FnMut(A, &'a FqxRow) -> Result<A>,
    {
        self.0.iter().try_fold(accumulator, f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup

impl OpFold<FqxRow> for FqxGroup<Vec<FqxRow>> {
    type Ret<A> = HashMap<FqxValue, A>;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, FqxRow) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.into_iter().fold(accumulator.clone(), &mut f);
            res.insert(k, a);
        }

        res
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, FqxRow) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.into_iter().try_fold(accumulator.clone(), &mut f)?;
            res.insert(k, a);
        }

        Ok(res)
    }
}

impl<'a> OpFold<&'a FqxRow> for FqxGroup<Vec<&'a FqxRow>> {
    type Ret<A> = HashMap<FqxValue, A>;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, &'a FqxRow) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.iter().cloned().fold(accumulator.clone(), &mut f);
            res.insert(k.clone(), a);
        }

        res
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, &'a FqxRow) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.iter().cloned().try_fold(accumulator.clone(), &mut f)?;
            res.insert(k.clone(), a);
        }

        Ok(res)
    }
}

impl OpFold<FqxRowSelect<FqxValue>> for FqxGroup<Vec<FqxRowSelect<FqxValue>>> {
    type Ret<A> = HashMap<FqxValue, A>;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, FqxRowSelect<FqxValue>) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.into_iter().fold(accumulator.clone(), &mut f);
            res.insert(k, a);
        }

        res
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, FqxRowSelect<FqxValue>) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.into_iter().try_fold(accumulator.clone(), &mut f)?;
            res.insert(k, a);
        }

        Ok(res)
    }
}

impl<'a> OpFold<&'a FqxRowSelect<&'a FqxValue>> for FqxGroup<&'a Vec<FqxRowSelect<&'a FqxValue>>> {
    type Ret<A> = HashMap<FqxValue, A>;

    fn fold<A, F>(self, accumulator: A, mut f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: FnMut(A, &'a FqxRowSelect<&'a FqxValue>) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.iter().fold(accumulator.clone(), &mut f);
            res.insert(k.clone(), a);
        }

        res
    }

    fn try_fold<A, F>(self, accumulator: A, mut f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, &'a FqxRowSelect<&'a FqxValue>) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.into_iter().try_fold(accumulator.clone(), &mut f)?;
            res.insert(k.clone(), a);
        }

        Ok(res)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSelect

impl OpFold<FqxRowSelect<FqxValue>> for Vec<FqxRowSelect<FqxValue>> {
    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: FnMut(A, FqxRowSelect<FqxValue>) -> A,
    {
        self.into_iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: FnMut(A, FqxRowSelect<FqxValue>) -> Result<A>,
    {
        self.into_iter().try_fold(accumulator, f)
    }
}

impl<'a> OpFold<FqxRowSelect<&'a FqxValue>> for Vec<FqxRowSelect<&'a FqxValue>> {
    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: FnMut(A, FqxRowSelect<&'a FqxValue>) -> A,
    {
        self.into_iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: FnMut(A, FqxRowSelect<&'a FqxValue>) -> Result<A>,
    {
        self.into_iter().try_fold(accumulator, f)
    }
}

impl<'a> OpFold<&'a FqxRowSelect<&'a FqxValue>> for &'a [FqxRowSelect<&'a FqxValue>] {
    type Ret<A> = A;

    fn fold<A, F>(self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: FnMut(A, &'a FqxRowSelect<&'a FqxValue>) -> A,
    {
        self.iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: FnMut(A, &'a FqxRowSelect<&'a FqxValue>) -> Result<A>,
    {
        self.iter().try_fold(accumulator, f)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_fold {
    use once_cell::sync::Lazy;

    use super::*;
    use crate::{adt::*, prelude::OpGroup};

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
            .group_by(|r| r[0].clone())
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);

        let foo = data
            .group_by(|r| r[0].clone())
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });
        println!("{:?}", foo);
    }
}
