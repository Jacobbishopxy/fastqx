//! file: reduce.rs
//! author: Jacob Xie
//! date: 2023/09/25 17:16:50 Monday
//! brief:

use std::collections::HashMap;

use anyhow::Result;

use crate::adt::{FqxData, FqxRow, FqxValue};
use crate::op::{FqxGroup, FqxRowSelect, FqxSlice};

// ================================================================================================
// OpReduce
// ================================================================================================

pub trait OpReduce<I> {
    type Ret<A>;

    fn reduce<F>(self, f: F) -> Self::Ret<I>
    where
        F: FnMut(I, I) -> I;

    fn try_reduce<F>(self, f: F) -> Result<Self::Ret<I>>
    where
        F: FnMut(I, I) -> Result<I>;
}

// ================================================================================================
// OpReduceFqxRow
// specific type auto implement
// ================================================================================================

pub trait OpReduceFqxRow
where
    Self: OpReduce<FqxRow, Ret<FqxRow> = Option<FqxRow>>,
    Self: Sized,
{
    fn reduce_fqx_row<F>(self, mut f: F) -> Option<FqxRow>
    where
        F: FnMut(FqxValue, FqxValue) -> FqxValue,
    {
        self.reduce(|pr, cr| {
            let inner =
                pr.0.into_iter()
                    .zip(cr.0.into_iter())
                    .map(|(p, c)| f(p, c))
                    .collect::<Vec<_>>();
            FqxRow(inner)
        })
    }
}

impl OpReduceFqxRow for FqxData {}

impl<'a> OpReduceFqxRow for &'a FqxData {}

impl<'a> OpReduceFqxRow for &'a FqxSlice {}

// ================================================================================================
// Impl
// ================================================================================================

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxData

impl OpReduce<FqxRow> for FqxData {
    type Ret<A> = Option<A>;

    fn reduce<F>(self, f: F) -> Self::Ret<FqxRow>
    where
        F: FnMut(FqxRow, FqxRow) -> FqxRow,
    {
        self.into_iter().reduce(f)
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<FqxRow>>
    where
        F: FnMut(FqxRow, FqxRow) -> Result<FqxRow>,
    {
        // try_reduce is not stable
        let mut iter = self.iter_owned();
        let mut res = None;
        if let Some(fst) = iter.next() {
            res = iter.try_fold(fst, |acc, r| f(acc, r).ok());
        }

        Ok(res)
    }
}

impl<'a> OpReduce<FqxRow> for &'a FqxData {
    type Ret<A> = Option<A>;

    fn reduce<F>(self, f: F) -> Self::Ret<FqxRow>
    where
        F: FnMut(FqxRow, FqxRow) -> FqxRow,
    {
        self.iter().cloned().reduce(f)
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<FqxRow>>
    where
        F: FnMut(FqxRow, FqxRow) -> Result<FqxRow>,
    {
        // try_reduce is not stable
        let mut iter = self.iter().cloned();
        let mut res = None;
        if let Some(fst) = iter.next() {
            res = iter.try_fold(fst, |acc, r| f(acc, r).ok());
        }

        Ok(res)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSlice

impl<'a> OpReduce<FqxRow> for &'a FqxSlice {
    type Ret<A> = Option<A>;

    fn reduce<F>(self, f: F) -> Self::Ret<FqxRow>
    where
        F: FnMut(FqxRow, FqxRow) -> FqxRow,
    {
        self.0.iter().cloned().reduce(f)
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<FqxRow>>
    where
        F: FnMut(FqxRow, FqxRow) -> Result<FqxRow>,
    {
        let mut iter = self.0.iter().cloned();
        let mut res = None;
        if let Some(fst) = iter.next() {
            res = iter.try_fold(fst, |acc, r| f(acc, r).ok());
        }

        Ok(res)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxGroup

impl OpReduce<FqxRow> for FqxGroup<Vec<FqxRow>> {
    type Ret<A> = HashMap<FqxValue, Option<A>>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<FqxRow>
    where
        F: FnMut(FqxRow, FqxRow) -> FqxRow,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let a = v.into_iter().reduce(&mut f);
            res.insert(k, a);
        }

        res
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<FqxRow>>
    where
        F: FnMut(FqxRow, FqxRow) -> Result<FqxRow>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.into_iter() {
            let mut iter = v.into_iter();
            let mut a = None;
            if let Some(fst) = iter.next() {
                a = iter.try_fold(fst, |p, c| f(p, c).ok());
            }
            res.insert(k, a);
        }

        Ok(res)
    }
}

impl<'a> OpReduce<FqxRow> for &'a FqxGroup<Vec<&'a FqxRow>> {
    type Ret<A> = HashMap<FqxValue, Option<A>>;

    fn reduce<F>(self, mut f: F) -> Self::Ret<FqxRow>
    where
        F: FnMut(FqxRow, FqxRow) -> FqxRow,
    {
        let mut res: HashMap<FqxValue, Option<FqxRow>> = HashMap::new();

        for (k, v) in self.0.iter() {
            let a = v.iter().cloned().cloned().reduce(&mut f);
            res.insert(k.clone(), a);
        }

        res
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<FqxRow>>
    where
        F: FnMut(FqxRow, FqxRow) -> Result<FqxRow>,
    {
        let mut res = HashMap::new();

        for (k, v) in self.0.iter() {
            let mut iter = v.iter().cloned().cloned();
            let mut a = None;
            if let Some(fst) = iter.next() {
                a = iter.try_fold(fst, |p, c| f(p, c).ok());
            }
            res.insert(k.clone(), a);
        }

        Ok(res)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FqxSelect

impl OpReduce<FqxRowSelect<FqxValue>> for Vec<FqxRowSelect<FqxValue>> {
    type Ret<A> = Option<A>;

    fn reduce<F>(self, f: F) -> Self::Ret<FqxRowSelect<FqxValue>>
    where
        F: FnMut(FqxRowSelect<FqxValue>, FqxRowSelect<FqxValue>) -> FqxRowSelect<FqxValue>,
    {
        self.into_iter().reduce(f)
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<FqxRowSelect<FqxValue>>>
    where
        F: FnMut(FqxRowSelect<FqxValue>, FqxRowSelect<FqxValue>) -> Result<FqxRowSelect<FqxValue>>,
    {
        // try_reduce is not stable
        let mut iter = self.into_iter();
        let mut res = None;
        if let Some(fst) = iter.next() {
            res = iter.try_fold(fst, |acc, r| f(acc, r).ok());
        }

        Ok(res)
    }
}

impl<'a> OpReduce<FqxRowSelect<FqxValue>> for Vec<FqxRowSelect<&'a FqxValue>> {
    type Ret<A> = Option<A>;

    fn reduce<F>(self, f: F) -> Self::Ret<FqxRowSelect<FqxValue>>
    where
        F: FnMut(FqxRowSelect<FqxValue>, FqxRowSelect<FqxValue>) -> FqxRowSelect<FqxValue>,
    {
        self.into_iter()
            .map(|r| FqxRowSelect(r.0.into_iter().cloned().collect()))
            .reduce(f)
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<FqxRowSelect<FqxValue>>>
    where
        F: FnMut(FqxRowSelect<FqxValue>, FqxRowSelect<FqxValue>) -> Result<FqxRowSelect<FqxValue>>,
    {
        // try_reduce is not stable
        let mut iter = self
            .into_iter()
            .map(|r| FqxRowSelect(r.0.into_iter().cloned().collect()));
        let mut res = None;
        if let Some(fst) = iter.next() {
            res = iter.try_fold(fst, |acc, r| f(acc, r).ok());
        }

        Ok(res)
    }
}

impl<'a> OpReduce<FqxRowSelect<FqxValue>> for &'a [FqxRowSelect<&'a FqxValue>] {
    type Ret<A> = Option<A>;

    fn reduce<F>(self, f: F) -> Self::Ret<FqxRowSelect<FqxValue>>
    where
        F: FnMut(FqxRowSelect<FqxValue>, FqxRowSelect<FqxValue>) -> FqxRowSelect<FqxValue>,
    {
        self.into_iter()
            .map(|r| FqxRowSelect(r.0.iter().cloned().cloned().collect()))
            .reduce(f)
    }

    fn try_reduce<F>(self, mut f: F) -> Result<Self::Ret<FqxRowSelect<FqxValue>>>
    where
        F: FnMut(FqxRowSelect<FqxValue>, FqxRowSelect<FqxValue>) -> Result<FqxRowSelect<FqxValue>>,
    {
        // try_reduce is not stable
        let mut iter = self
            .into_iter()
            .map(|r| FqxRowSelect(r.0.iter().cloned().cloned().collect()));
        let mut res = None;
        if let Some(fst) = iter.next() {
            res = iter.try_fold(fst, |acc, r| f(acc, r).ok());
        }

        Ok(res)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_reduce {
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
    fn reduce_self_success() {
        let data = DATA.clone();

        let foo = (&data).reduce(|p, c| p + c);
        println!("{:?}", foo);

        let foo = data.reduce(|p, c| p + c);
        println!("{:?}", foo);
    }

    #[test]
    fn reduce_slice_success() {
        let data = DATA.clone();

        let slice = &data[..];

        let foo = slice.reduce(|p, c| p + c);

        println!("{:?}", foo);
    }

    #[test]
    fn reduce_group_success() {
        let data = DATA.clone();

        let foo = data.group_by(|r| r[0].clone()).reduce(|p, c| p + c);

        println!("{:?}", foo);
    }
}
