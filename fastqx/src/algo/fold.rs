//! file: fold.rs
//! author: Jacob Xie
//! date: 2023/09/24 18:50:53 Sunday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::algo::{FqxGroup, FqxSlice};

// ================================================================================================
// AlgoFold
// ================================================================================================

pub trait AlgoFold<'a, II>
where
    Self: 'a,
{
    type Ret<A>;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: Fn(A, II) -> A;

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: Fn(A, II) -> Result<A>;
}

pub trait AlgoFoldMut<'a, II>
where
    Self: 'a,
{
    type Ret<A>;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: Fn(A, II) -> A;

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: Fn(A, II) -> Result<A>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<'a> AlgoFold<'a, &'a FqxRow> for FqxData {
    type Ret<A> = A;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, &'a FqxRow) -> A,
    {
        self.iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, &'a FqxRow) -> Result<A>,
    {
        self.iter().try_fold(accumulator, f)
    }
}

impl<'a> AlgoFoldMut<'a, &'a mut FqxRow> for FqxData {
    type Ret<A> = A;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, &'a mut FqxRow) -> A,
    {
        self.iter_mut().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, &'a mut FqxRow) -> Result<A>,
    {
        self.iter_mut().try_fold(accumulator, f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> AlgoFold<'a, &'a FqxRow> for FqxSlice {
    type Ret<A> = A;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, &'a FqxRow) -> A,
    {
        self.0.iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, &'a FqxRow) -> Result<A>,
    {
        self.0.iter().try_fold(accumulator, f)
    }
}

impl<'a> AlgoFoldMut<'a, &'a mut FqxRow> for FqxSlice {
    type Ret<A> = A;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, &'a mut FqxRow) -> A,
    {
        self.0.iter_mut().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, &'a mut FqxRow) -> Result<A>,
    {
        self.0.iter_mut().try_fold(accumulator, f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> AlgoFold<'a, &'a FqxRow> for FqxGroup<Vec<&'a FqxRow>> {
    type Ret<A> = HashMap<FqxValue, A>;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: Fn(A, &'a FqxRow) -> A,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.iter().fold(accumulator.clone(), |acc, r| f(acc, *r));
            res.insert(k.clone(), a);
        }

        res
    }

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: Fn(A, &'a FqxRow) -> Result<A>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v
                .iter()
                .try_fold(accumulator.clone(), |acc, r| f(acc, *r))?;
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
    use crate::{adt::*, prelude::AlgoGroup};

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

        let foo = data.fold(vec![], |mut acc, r| {
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
    fn fold_self_mut_success() {
        let mut data = DATA.clone();

        let foo = (&mut data).fold(vec![], |mut acc, r| {
            r[2] *= 2.into();
            acc.push(r[2].clone());

            acc
        });
        println!("{:?}", foo);
        println!("{:?}", data);
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
    fn fold_slice_mut_success() {
        let mut data = DATA.clone();

        let slice = &mut data[1..3];

        let foo = slice.fold(vec![], |mut acc, r| {
            r[2] *= 2.into();
            acc.push(r[2].clone());

            acc
        });
        println!("{:?}", foo);
        println!("{:?}", data);
    }

    #[test]
    fn fold_group_success() {
        let data = DATA.clone();

        let foo = data
            .group_by(|r| r[0].clone())
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r[1].to_string());

                acc
            });

        println!("{:?}", foo);
    }
}
