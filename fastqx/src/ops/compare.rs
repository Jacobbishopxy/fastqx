//! file: compare.rs
//! author: Jacob Xie
//! date: 2023/09/30 23:59:57 Saturday
//! brief:

use itertools::{EitherOrBoth, Itertools};

use crate::adt::{FqxData, FqxRowAbstract, FqxRowLike, FqxValue};

// ================================================================================================
// OpCompare
// ================================================================================================

pub trait OpCompare<I> {
    type Ret;

    fn equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<I>;

    fn not_equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<I>;

    fn gt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<I>;

    fn gt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<I>;

    fn lt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<I>;

    fn lt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<I>;
}

// ================================================================================================
// Impl
// ================================================================================================

macro_rules! compare_value {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        (&$lhs.0)
            .into_iter()
            .map(|lhs| lhs.as_ref() $op $rhs.as_ref())
            .collect()
    };
}

impl<'a, I, V> OpCompare<FqxValue> for &'a FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    for<'b> &'b I: IntoIterator<Item = &'b V>,
    V: Into<FqxValue> + AsRef<FqxValue>,
{
    type Ret = Vec<bool>;

    fn equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_value!(self, rhs, ==)
    }

    fn not_equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_value!(self, rhs, !=)
    }

    fn gt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_value!(self, rhs, >)
    }

    fn gt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_value!(self, rhs, >=)
    }

    fn lt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_value!(self, rhs, <)
    }

    fn lt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_value!(self, rhs, <=)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! compare_row_abs {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        (&$lhs.0)
            .into_iter()
            .zip_longest((&$rhs.as_ref().0).into_iter())
            .map(|pair| match pair {
                EitherOrBoth::Both(l, r) => l.as_ref() $op r.as_ref(),
                _ => false,
            })
            .collect()
    };
}

impl<'a, LI, LV, RI, RV> OpCompare<FqxRowAbstract<LI, LV>> for &'a FqxRowAbstract<RI, RV>
where
    LI: IntoIterator<Item = LV>,
    for<'b> &'b LI: IntoIterator<Item = &'b LV>,
    RI: IntoIterator<Item = RV>,
    for<'b> &'b RI: IntoIterator<Item = &'b RV>,
    LV: Into<FqxValue> + AsRef<FqxValue>,
    RV: Into<FqxValue> + AsRef<FqxValue>,
{
    type Ret = Vec<bool>;

    fn equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<LI, LV>>,
    {
        compare_row_abs!(self, rhs, ==)
    }

    fn not_equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<LI, LV>>,
    {
        compare_row_abs!(self, rhs, !=)
    }

    fn gt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<LI, LV>>,
    {
        compare_row_abs!(self, rhs, >)
    }

    fn gt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<LI, LV>>,
    {
        compare_row_abs!(self, rhs, >=)
    }

    fn lt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<LI, LV>>,
    {
        compare_row_abs!(self, rhs, <)
    }

    fn lt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<LI, LV>>,
    {
        compare_row_abs!(self, rhs, <=)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! compare_val_fqxdata {
    ($lhs:expr, $rhs:expr, $op:ident) => {
        $lhs.iter()
            .map(|row| row.as_abstract_ref().$op($rhs.as_ref()))
            .collect()
    };
}

impl<'a> OpCompare<FqxValue> for FqxData {
    type Ret = Vec<Vec<bool>>;

    fn equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_val_fqxdata!(self, rhs, equal)
    }

    fn not_equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_val_fqxdata!(self, rhs, not_equal)
    }

    fn gt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_val_fqxdata!(self, rhs, gt)
    }

    fn gt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_val_fqxdata!(self, rhs, gt_eq)
    }

    fn lt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_val_fqxdata!(self, rhs, lt)
    }

    fn lt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        compare_val_fqxdata!(self, rhs, lt_eq)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! compare_row_fqxdata {
    ($lhs:expr, $rhs:expr, $op:ident) => {
        $lhs.iter()
            .map(|row| row.as_abstract_ref().$op($rhs.as_ref()))
            .collect()
    };
}

impl<'a, I, V> OpCompare<FqxRowAbstract<I, V>> for FqxData
where
    I: IntoIterator<Item = V>,
    for<'b> &'b I: IntoIterator<Item = &'b V>,
    V: Into<FqxValue> + AsRef<FqxValue>,
{
    type Ret = Vec<Vec<bool>>;

    fn equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<I, V>>,
    {
        compare_row_fqxdata!(self, rhs, equal)
    }

    fn not_equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<I, V>>,
    {
        compare_row_fqxdata!(self, rhs, not_equal)
    }

    fn gt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<I, V>>,
    {
        compare_row_fqxdata!(self, rhs, gt)
    }

    fn gt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<I, V>>,
    {
        compare_row_fqxdata!(self, rhs, gt_eq)
    }

    fn lt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<I, V>>,
    {
        compare_row_fqxdata!(self, rhs, lt)
    }

    fn lt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxRowAbstract<I, V>>,
    {
        compare_row_fqxdata!(self, rhs, lt_eq)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_compare {
    use once_cell::sync::Lazy;

    use super::*;

    use crate::adt::{FqxRow, FqxValueType};
    use crate::ops::FqxRowSelect;

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
    fn value_absrow_cmp_success() {
        let a1 = FqxRow(vec![
            FqxValue::F32(0.1),                 // false
            FqxValue::I16(1),                   // false
            FqxValue::String("ha".to_string()), // false
        ]);

        let res = a1.as_abstract_ref().lt_eq(FqxValue::F32(0.01));
        println!("{:?}", res);
    }

    #[test]
    fn absrow_cmp_success() {
        let a1 = FqxRow(vec![
            FqxValue::F32(0.2),
            FqxValue::I16(2),
            FqxValue::String("ha".to_string()),
        ]);

        let (v1, v2, v3) = (
            FqxValue::F32(0.1),
            FqxValue::I16(3),
            FqxValue::String("ha".to_string()),
        );

        let a2 = FqxRowSelect(vec![&v1, &v2, &v3]);

        let res = a1.as_abstract_ref().gt(a2);
        println!("{:?}", res);
    }

    #[test]
    fn value_fqxdata_cmp_success() {
        let data = DATA.clone();

        let res = data.gt(FqxValue::I32(0));
        println!("{:?}", res);
    }

    #[test]
    fn fqxdata_cmp_success() {
        let data = DATA.clone();

        let row = FqxRow(vec![
            FqxValue::I16(2),
            FqxValue::String("ha".to_string()),
            FqxValue::F32(0.2),
        ]);

        let res = data.gt(row.as_abstract_ref());
        println!("{:?}", res);
    }
}
