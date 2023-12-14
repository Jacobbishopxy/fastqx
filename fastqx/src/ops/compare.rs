//! file: compare.rs
//! author: Jacob Xie
//! date: 2023/09/30 23:59:57 Saturday
//! brief:

use crate::adt::{FqxD, FqxValue, RowProps};

// ================================================================================================
// OpCompare
// ================================================================================================

pub trait OpCompare<I, const V: usize> {
    type Ret;

    fn eq(&self, rhs: &I) -> Self::Ret;

    fn neq(&self, rhs: &I) -> Self::Ret;

    fn gt(&self, rhs: &I) -> Self::Ret;

    fn gte(&self, rhs: &I) -> Self::Ret;

    fn lt(&self, rhs: &I) -> Self::Ret;

    fn lte(&self, rhs: &I) -> Self::Ret;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<S> OpCompare<FqxValue, 0> for S
where
    S: RowProps,
{
    type Ret = Vec<bool>;

    fn eq(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|e| e == rhs).collect()
    }

    fn neq(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|e| e != rhs).collect()
    }

    fn gt(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|e| e > rhs).collect()
    }

    fn gte(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|e| e >= rhs).collect()
    }

    fn lt(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|e| e < rhs).collect()
    }

    fn lte(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|e| e <= rhs).collect()
    }
}

impl<S, V> OpCompare<V, 0> for S
where
    S: RowProps,
    V: RowProps,
{
    type Ret = Vec<bool>;

    fn eq(&self, rhs: &V) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l == r).collect()
    }

    fn neq(&self, rhs: &V) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l != r).collect()
    }

    fn gt(&self, rhs: &V) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l > r).collect()
    }

    fn gte(&self, rhs: &V) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l >= r).collect()
    }

    fn lt(&self, rhs: &V) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l < r).collect()
    }

    fn lte(&self, rhs: &V) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l <= r).collect()
    }
}

impl<U> OpCompare<FqxValue, 1> for U
where
    U: FqxD,
{
    type Ret = Vec<Vec<bool>>;

    fn eq(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|row| row.eq(rhs)).collect()
    }

    fn neq(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|row| row.neq(rhs)).collect()
    }

    fn gt(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|row| row.gt(rhs)).collect()
    }

    fn gte(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|row| row.gte(rhs)).collect()
    }

    fn lt(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|row| row.lt(rhs)).collect()
    }

    fn lte(&self, rhs: &FqxValue) -> Self::Ret {
        self.iter().map(|row| row.lte(rhs)).collect()
    }
}

impl<U, V> OpCompare<V, 1> for U
where
    U: FqxD,
    V: RowProps,
{
    type Ret = Vec<Vec<bool>>;

    fn eq(&self, rhs: &V) -> Self::Ret {
        self.iter().map(|r| r.eq(rhs)).collect()
    }

    fn neq(&self, rhs: &V) -> Self::Ret {
        self.iter().map(|r| r.neq(rhs)).collect()
    }

    fn gt(&self, rhs: &V) -> Self::Ret {
        self.iter().map(|r| r.gt(rhs)).collect()
    }

    fn gte(&self, rhs: &V) -> Self::Ret {
        self.iter().map(|r| r.gte(rhs)).collect()
    }

    fn lt(&self, rhs: &V) -> Self::Ret {
        self.iter().map(|r| r.lt(rhs)).collect()
    }

    fn lte(&self, rhs: &V) -> Self::Ret {
        self.iter().map(|r| r.lte(rhs)).collect()
    }
}

impl<U> OpCompare<U, 2> for U
where
    U: FqxD,
{
    type Ret = Vec<Vec<bool>>;

    fn eq(&self, rhs: &U) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l.eq(r)).collect()
    }

    fn neq(&self, rhs: &U) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l.neq(r)).collect()
    }

    fn gt(&self, rhs: &U) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l.gt(r)).collect()
    }

    fn gte(&self, rhs: &U) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l.gte(r)).collect()
    }

    fn lt(&self, rhs: &U) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l.lt(r)).collect()
    }

    fn lte(&self, rhs: &U) -> Self::Ret {
        self.iter().zip(rhs.iter()).map(|(l, r)| l.lte(r)).collect()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_compare {
    use super::*;

    use crate::fqx;
    use crate::ops::mock::data::D3;

    #[test]
    fn value_data_cmp_success() {
        let data = D3.clone();

        let res = data.eq(&fqx!(2));
        println!("{:?}", res);
    }
}
