//! file: fold.rs
//! author: Jacob Xie
//! date: 2023/09/24 18:50:53 Sunday
//! brief:

use anyhow::Result;

use crate::adt::{FqxData, FqxRow};
use crate::algo::FqxSlice;

// ================================================================================================
// AlgoFold
// ================================================================================================

pub trait AlgoFold<'a>
where
    Self: 'a,
{
    type IterItem;
    type Ret<A>;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> A;

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> Result<A>;
}

pub trait AlgoFoldMut<'a>
where
    Self: 'a,
{
    type IterItem;
    type Ret<A>;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> A;

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        A: Clone,
        F: Fn(A, Self::IterItem) -> Result<A>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<'a> AlgoFold<'a> for FqxData {
    type IterItem = &'a FqxRow;
    type Ret<A> = A;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, Self::IterItem) -> A,
    {
        self.iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        self.iter().try_fold(accumulator, f)
    }
}

impl<'a> AlgoFoldMut<'a> for FqxData {
    type IterItem = &'a mut FqxRow;
    type Ret<A> = A;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, Self::IterItem) -> A,
    {
        self.iter_mut().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        self.iter_mut().try_fold(accumulator, f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> AlgoFold<'a> for FqxSlice {
    type IterItem = &'a FqxRow;
    type Ret<A> = A;

    fn fold<A, F>(&'a self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, Self::IterItem) -> A,
    {
        self.0.iter().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        self.0.iter().try_fold(accumulator, f)
    }
}

impl<'a> AlgoFoldMut<'a> for FqxSlice {
    type IterItem = &'a mut FqxRow;
    type Ret<A> = A;

    fn fold<A, F>(&'a mut self, accumulator: A, f: F) -> Self::Ret<A>
    where
        F: Fn(A, Self::IterItem) -> A,
    {
        self.0.iter_mut().fold(accumulator, f)
    }

    fn try_fold<A, F>(&'a mut self, accumulator: A, f: F) -> Result<Self::Ret<A>>
    where
        F: Fn(A, Self::IterItem) -> Result<A>,
    {
        self.0.iter_mut().try_fold(accumulator, f)
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
}
