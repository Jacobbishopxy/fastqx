//! file: arith.rs
//! author: Jacob Xie
//! date: 2023/09/23 22:32:10 Saturday
//! brief:

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::adt::FqxValue;

macro_rules! binary_fn {
    ($lhs:expr, $op:tt, $rhs:expr) => {
        match $lhs {
            FqxValue::Bool(_) => FqxValue::Null,
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
            FqxValue::String(_) => FqxValue::Null,
            FqxValue::Blob(_) => FqxValue::Null,
            FqxValue::Null => FqxValue::Null,
        }
    };
}

// ================================================================================================
// Arithmetic
// ================================================================================================

impl Add for FqxValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        binary_fn!(self, +, rhs)
    }
}

impl AddAssign for FqxValue {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub for FqxValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        binary_fn!(self, -, rhs)
    }
}

impl SubAssign for FqxValue {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl Mul for FqxValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        binary_fn!(self, *, rhs)
    }
}

impl MulAssign for FqxValue {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl Div for FqxValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        binary_fn!(self, /, rhs)
    }
}

impl DivAssign for FqxValue {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}
