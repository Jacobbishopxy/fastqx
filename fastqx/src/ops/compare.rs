//! file: compare.rs
//! author: Jacob Xie
//! date: 2023/09/30 23:59:57 Saturday
//! brief:

// ================================================================================================
use crate::adt::{FqxData, FqxDataCow, FqxValue};
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

impl<'a> OpCompare<FqxValue> for FqxData {
    type Ret = Vec<Vec<bool>>;

    fn equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn not_equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn gt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn gt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn lt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn lt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> OpCompare<FqxValue> for FqxDataCow<'a> {
    type Ret = Vec<Vec<bool>>;

    fn equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn not_equal<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn gt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn gt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn lt<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }

    fn lt_eq<R>(&self, rhs: R) -> Self::Ret
    where
        R: AsRef<FqxValue>,
    {
        todo!()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_compare {
    use super::*;

    use crate::fqx;
    use crate::mock::data::D3;

    #[test]
    fn value_data_cmp_success() {
        let data = D3.clone();

        let res = data.gt(fqx!(0));
        println!("{:?}", res);
    }
}
