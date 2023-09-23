//! file: arith.rs
//! author: Jacob Xie
//! date: 2023/09/23 22:32:10 Saturday
//! brief:

use std::ops::{Add, Div, Mul, Sub};

use crate::adt::FqxValue;

impl Add for FqxValue {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Sub for FqxValue {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for FqxValue {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div for FqxValue {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
