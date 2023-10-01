//! file: arith.rs
//! author: Jacob Xie
//! date: 2023/09/23 22:32:10 Saturday
//! brief:

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use itertools::{EitherOrBoth, Itertools};

use crate::adt::{FqxRow, FqxRowAbstract, FqxValue};

macro_rules! binary_fn {
    ($lhs:expr, $op:tt, $rhs:expr) => {
        match $lhs {
            FqxValue::U8(v) => u8::try_from($rhs)
                .map(|n| FqxValue::U8(n $op v))
                .unwrap_or_default(),
            FqxValue::U16(v) => u16::try_from($rhs)
                .map(|n| FqxValue::U16(n $op v))
                .unwrap_or_default(),
            FqxValue::U32(v) => u32::try_from($rhs)
                .map(|n| FqxValue::U32(n $op v))
                .unwrap_or_default(),
            FqxValue::U64(v) => u64::try_from($rhs)
                .map(|n| FqxValue::U64(n $op v))
                .unwrap_or_default(),
            FqxValue::I8(v) => i8::try_from($rhs)
                .map(|n| FqxValue::I8(n $op v))
                .unwrap_or_default(),
            FqxValue::I16(v) => i16::try_from($rhs)
                .map(|n| FqxValue::I16(n $op v))
                .unwrap_or_default(),
            FqxValue::I32(v) => i32::try_from($rhs)
                .map(|n| FqxValue::I32(n $op v))
                .unwrap_or_default(),
            FqxValue::I64(v) => i64::try_from($rhs)
                .map(|n| FqxValue::I64(n $op v))
                .unwrap_or_default(),
            FqxValue::F32(v) => f32::try_from($rhs)
                .map(|n| FqxValue::F32(n $op v))
                .unwrap_or_default(),
            FqxValue::F64(v) => f64::try_from($rhs)
                .map(|n| FqxValue::F64(n $op v))
                .unwrap_or_default(),
            _ => FqxValue::Null,
        }
    };
}

macro_rules! assign_fn {
    ($lhs:expr, $op:tt, $rhs:expr) => {
        match $lhs {
            FqxValue::U8(v) => {
                if let Err(_) = u8::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            FqxValue::U16(v) => {
                if let Err(_) = u16::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            FqxValue::U32(v) => {
                if let Err(_) = u32::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            FqxValue::U64(v) => {
                if let Err(_) = u64::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            FqxValue::I8(v) => {
                if let Err(_) = i8::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            FqxValue::I16(v) => {
                if let Err(_) = i16::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            FqxValue::I32(v) => {
                if let Err(_) = i32::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            FqxValue::I64(v) => {
                if let Err(_) = i64::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            FqxValue::F32(v) => {
                if let Err(_) = f32::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            FqxValue::F64(v) => {
                if let Err(_) = f64::try_from($rhs).map(|n| *v $op n) {
                    *$lhs = FqxValue::Null;
                }
            }
            _ => {
                *$lhs = FqxValue::Null;
            }
        }
    };
}

// ================================================================================================
// Arithmetic: FqxValue
// ================================================================================================

macro_rules! impl_arith_for_value {
    ($t:ident, $tf:tt, $ta:ident, $taf:tt, $op:tt, $opa:tt) => {
        impl $t for FqxValue {
            type Output = Self;

            fn $tf(self, rhs: Self) -> Self::Output {
                binary_fn!(self, $op, rhs)
            }
        }

        impl $ta for FqxValue {
            fn $taf(&mut self, rhs: Self) {
                assign_fn!(self, $opa, rhs)
            }
        }
    };
}

impl_arith_for_value!(Add, add, AddAssign, add_assign, +, +=);
impl_arith_for_value!(Sub, sub, SubAssign, sub_assign, -, -=);
impl_arith_for_value!(Mul, mul, MulAssign, mul_assign, *, *=);
impl_arith_for_value!(Div, div, DivAssign, div_assign, /, /=);
impl_arith_for_value!(Rem, rem, RemAssign, rem_assign, %, %=);

// ================================================================================================
// Arithmetic: FqxRow
// ================================================================================================

macro_rules! impl_arith_for_row {
    ($t:ident, $tf:tt, $ta:ident, $taf:tt, $op:tt, $opa:tt) => {
        impl $t for FqxRow {
            type Output = FqxRow;

            fn $tf(self, rhs: Self) -> Self::Output {
                let inner = self
                    .0
                    .into_iter()
                    .zip_longest(rhs.0.into_iter())
                    .map(|pair| match pair {
                        EitherOrBoth::Both(l, r) => l $op r,
                        EitherOrBoth::Left(_) => FqxValue::Null,
                        EitherOrBoth::Right(_) => FqxValue::Null,
                    })
                    .collect();

                FqxRow(inner)
            }
        }

        impl $ta for FqxRow {
            fn $taf(&mut self, rhs: Self) {
                self.0
                    .iter_mut()
                    .zip_longest(rhs.0.into_iter())
                    .for_each(|pair| match pair {
                        EitherOrBoth::Both(l, r) => *l $opa r,
                        _ => {}
                    })
            }
        }
    };
}

impl_arith_for_row!(Add, add, AddAssign, add_assign, +, +=);
impl_arith_for_row!(Sub, sub, SubAssign, sub_assign, -, -=);
impl_arith_for_row!(Mul, mul, MulAssign, mul_assign, *, *=);
impl_arith_for_row!(Div, div, DivAssign, div_assign, /, /=);
impl_arith_for_row!(Rem, rem, RemAssign, rem_assign, %, %=);

// ================================================================================================
// Arithmetic: FqxRowAbstract
// ================================================================================================

macro_rules! impl_arith_for_abs_row {
    ($t:ident, $tf:tt, $ta:ident, $taf:tt, $op:tt, $opa:tt) => {
        impl<I, V> $t<FqxRowAbstract<I, V>> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V> + FromIterator<FqxValue>,
            V: Into<FqxValue> + From<FqxValue>,
        {
            type Output = FqxRowAbstract<I, V>;

            fn $tf(self, rhs: FqxRowAbstract<I, V>) -> Self::Output {
                let inner = self
                    .0
                    .into_iter()
                    .zip_longest(rhs.0.into_iter())
                    .map(|pair| match pair {
                        EitherOrBoth::Both(l, r) => l.into() $op r.into(),
                        EitherOrBoth::Left(_) => FqxValue::Null,
                        EitherOrBoth::Right(_) => FqxValue::Null,
                    })
                    .collect();

                FqxRowAbstract(inner)
            }
        }

        impl<I, V> $ta<FqxRowAbstract<I, V>> for FqxRowAbstract<I, V>
        where
            I: IntoIterator<Item = V>,
            for<'a> &'a mut I: IntoIterator<Item = &'a mut V>,
            V: Into<FqxValue> + AsMut<FqxValue>,
        {
            fn $taf(&mut self, rhs: FqxRowAbstract<I, V>) {
                (&mut self.0)
                    .into_iter()
                    .zip_longest(rhs.0.into_iter())
                    .for_each(|pair| match pair {
                        EitherOrBoth::Both(l, r) => *l.as_mut() $opa r.into(),
                        _ => {}
                    })
            }
        }
    };
}

impl_arith_for_abs_row!(Add, add, AddAssign, add_assign, +, +=);
impl_arith_for_abs_row!(Sub, sub, SubAssign, sub_assign, -, -=);
impl_arith_for_abs_row!(Mul, mul, MulAssign, mul_assign, *, *=);
impl_arith_for_abs_row!(Div, div, DivAssign, div_assign, /, /=);
impl_arith_for_abs_row!(Rem, rem, RemAssign, rem_assign, %, %=);

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_arith {
    // use super::*;
}
