//! file: merge.rs
//! author: Jacob Xie
//! date: 2023/10/10 09:11:09 Tuesday
//! brief:

use crate::adt::FqxD;
use crate::ops::utils::{_join, _outer_join};

// ================================================================================================
// FqxJoinType
// ================================================================================================

pub enum FqxJoinType {
    Left,
    Right,
    Outer,
    Inner,
    // Cross,
}

impl Default for FqxJoinType {
    fn default() -> Self {
        Self::Left
    }
}

// ================================================================================================
// OpMerge
// ================================================================================================

pub trait OpMerge: Sized {
    type Ret;

    fn merge<O, N, S>(self, other: O, left_on: &N, right_on: &N, how: FqxJoinType) -> Self::Ret
    where
        Self: From<O>,
        for<'a> &'a N: IntoIterator<Item = &'a S>,
        S: AsRef<str>;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<U> OpMerge for U
where
    Self: Sized,
    U: FqxD,
{
    type Ret = U;

    fn merge<O, N, S>(self, other: O, left_on: &N, right_on: &N, how: FqxJoinType) -> Self::Ret
    where
        Self: From<O>,
        for<'a> &'a N: IntoIterator<Item = &'a S>,
        S: AsRef<str>,
    {
        let (l, r) = (self, other.into());
        match how {
            FqxJoinType::Left => _join(l, r, left_on, right_on, false),
            FqxJoinType::Right => _join(r, l, right_on, left_on, false),
            FqxJoinType::Inner => _join(l, r, left_on, right_on, true),
            FqxJoinType::Outer => _outer_join(l, r, left_on, right_on),
        }
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_merge {
    use super::*;
    use crate::ops::mock::data::{D6, D7};
    use crate::ops::OpSelect;

    #[test]
    fn merge_self_success() {
        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // merge left

        let d1 = D6.clone();
        let d2 = D7.clone();

        let res = d1.merge(d2.rf(), &["Fruit"], &["Name"], FqxJoinType::Left);
        println!("merge left:");
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }

        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // merge right

        let d1 = D6.clone();
        let d2 = D7.clone();

        let res = d1.merge(d2, &["Fruit"], &["Name"], FqxJoinType::Right);
        println!("merge right:");
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
    }

    #[test]
    fn merge_self_success2() {
        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // merge inner

        let d1 = D6.clone();
        let d2 = D7.clone();

        let res = d1.merge(d2, &["Fruit"], &["Name"], FqxJoinType::Inner);
        println!("merge inner:");
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // merge outer

        let d1 = D6.clone();
        let d2 = D7.clone();

        let res = d1.merge(d2, &["Fruit"], &["Name"], FqxJoinType::Outer);
        println!("merge outer:");
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
    }
}
