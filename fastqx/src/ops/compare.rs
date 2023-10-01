//! file: compare.rs
//! author: Jacob Xie
//! date: 2023/09/30 23:59:57 Saturday
//! brief:

use itertools::{EitherOrBoth, Itertools};

use crate::{
    adt::{FqxData, FqxRowAbstract, FqxValue},
    prelude::FqxRowLike,
};

// ================================================================================================
// OpCompare
// ================================================================================================

pub trait OpCompare<I> {
    type Ret;

    fn equal(&self, rhs: I) -> Self::Ret;

    fn not_equal(&self, rhs: I) -> Self::Ret;

    fn gt(&self, rhs: I) -> Self::Ret;

    fn gt_eq(&self, rhs: I) -> Self::Ret;

    fn lt(&self, rhs: I) -> Self::Ret;

    fn lt_eq(&self, rhs: I) -> Self::Ret;
}

// ================================================================================================
// Impl
// ================================================================================================

macro_rules! compare_value {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        (&$lhs.0)
            .into_iter()
            .map(|lhs| lhs.as_ref() $op $rhs)
            .collect()
    };
}

impl<'a, I, V> OpCompare<&'a FqxValue> for &'a FqxRowAbstract<I, V>
where
    I: IntoIterator<Item = V>,
    for<'b> &'b I: IntoIterator<Item = &'b V>,
    V: Into<FqxValue> + AsRef<FqxValue>,
{
    type Ret = Vec<bool>;

    fn equal(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_value!(self, rhs, ==)
    }

    fn not_equal(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_value!(self, rhs, !=)
    }

    fn gt(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_value!(self, rhs, >)
    }

    fn gt_eq(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_value!(self, rhs, >=)
    }

    fn lt(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_value!(self, rhs, <)
    }

    fn lt_eq(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_value!(self, rhs, <=)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! compare_row_abs {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        (&$lhs.0)
            .into_iter()
            .zip_longest((&$rhs.0).into_iter())
            .map(|pair| match pair {
                EitherOrBoth::Both(l, r) => l.as_ref() $op r.as_ref(),
                _ => false,
            })
            .collect()
    };
}

impl<'a, LI, LV, RI, RV> OpCompare<&'a FqxRowAbstract<LI, LV>> for &'a FqxRowAbstract<RI, RV>
where
    LI: IntoIterator<Item = LV>,
    for<'b> &'b LI: IntoIterator<Item = &'b LV>,
    RI: IntoIterator<Item = RV>,
    for<'b> &'b RI: IntoIterator<Item = &'b RV>,
    LV: Into<FqxValue> + AsRef<FqxValue>,
    RV: Into<FqxValue> + AsRef<FqxValue>,
{
    type Ret = Vec<bool>;

    fn equal(&self, rhs: &'a FqxRowAbstract<LI, LV>) -> Self::Ret {
        compare_row_abs!(self, rhs, ==)
    }

    fn not_equal(&self, rhs: &'a FqxRowAbstract<LI, LV>) -> Self::Ret {
        compare_row_abs!(self, rhs, !=)
    }

    fn gt(&self, rhs: &'a FqxRowAbstract<LI, LV>) -> Self::Ret {
        compare_row_abs!(self, rhs, >)
    }

    fn gt_eq(&self, rhs: &'a FqxRowAbstract<LI, LV>) -> Self::Ret {
        compare_row_abs!(self, rhs, >=)
    }

    fn lt(&self, rhs: &'a FqxRowAbstract<LI, LV>) -> Self::Ret {
        compare_row_abs!(self, rhs, <)
    }

    fn lt_eq(&self, rhs: &'a FqxRowAbstract<LI, LV>) -> Self::Ret {
        compare_row_abs!(self, rhs, <=)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! compare_val_fqxdata {
    ($lhs:expr, $rhs:expr, $op:ident) => {
        $lhs.iter()
            .map(|row| row.as_abstract_ref().$op($rhs))
            .collect()
    };
}

impl<'a> OpCompare<&'a FqxValue> for FqxData {
    type Ret = Vec<Vec<bool>>;

    fn equal(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_val_fqxdata!(self, rhs, equal)
    }

    fn not_equal(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_val_fqxdata!(self, rhs, not_equal)
    }

    fn gt(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_val_fqxdata!(self, rhs, gt)
    }

    fn gt_eq(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_val_fqxdata!(self, rhs, gt_eq)
    }

    fn lt(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_val_fqxdata!(self, rhs, lt)
    }

    fn lt_eq(&self, rhs: &'a FqxValue) -> Self::Ret {
        compare_val_fqxdata!(self, rhs, lt_eq)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! compare_row_fqxdata {
    ($lhs:expr, $rhs:expr, $op:ident) => {
        $lhs.iter()
            .map(|row| row.as_abstract_ref().$op($rhs))
            .collect()
    };
}

impl<'a, I, V> OpCompare<&'a FqxRowAbstract<I, V>> for FqxData
where
    I: IntoIterator<Item = V>,
    for<'b> &'b I: IntoIterator<Item = &'b V>,
    V: Into<FqxValue> + AsRef<FqxValue>,
{
    type Ret = Vec<Vec<bool>>;

    fn equal(&self, rhs: &'a FqxRowAbstract<I, V>) -> Self::Ret {
        compare_row_fqxdata!(self, rhs, equal)
    }

    fn not_equal(&self, rhs: &'a FqxRowAbstract<I, V>) -> Self::Ret {
        compare_row_fqxdata!(self, rhs, not_equal)
    }

    fn gt(&self, rhs: &'a FqxRowAbstract<I, V>) -> Self::Ret {
        compare_row_fqxdata!(self, rhs, gt)
    }

    fn gt_eq(&self, rhs: &'a FqxRowAbstract<I, V>) -> Self::Ret {
        compare_row_fqxdata!(self, rhs, gt_eq)
    }

    fn lt(&self, rhs: &'a FqxRowAbstract<I, V>) -> Self::Ret {
        compare_row_fqxdata!(self, rhs, lt)
    }

    fn lt_eq(&self, rhs: &'a FqxRowAbstract<I, V>) -> Self::Ret {
        compare_row_fqxdata!(self, rhs, lt_eq)
    }
}
