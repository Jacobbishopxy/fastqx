//! file: join.rs
//! author: Jacob Xie
//! date: 2023/10/31 14:36:38 Tuesday
//! brief:

use crate::adt::FqxD;
use crate::ops::utils::{_join, _outer_join};
use crate::ops::FqxJoinType;

// ================================================================================================
// OpJoin
// ================================================================================================

pub trait OpJoin: Sized {
    type Ret;

    fn join<O, N, S>(self, other: O, on: &N, how: FqxJoinType) -> Self::Ret
    where
        Self: From<O>,
        for<'a> &'a N: IntoIterator<Item = &'a S>,
        S: AsRef<str>;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<U> OpJoin for U
where
    Self: Sized,
    U: FqxD,
{
    type Ret = U;

    fn join<O, N, S>(self, other: O, on: &N, how: FqxJoinType) -> Self::Ret
    where
        Self: From<O>,
        for<'a> &'a N: IntoIterator<Item = &'a S>,
        S: AsRef<str>,
    {
        let (l, r) = (self, other.into());
        match how {
            FqxJoinType::Left => _join(l, r, on, on, false),
            FqxJoinType::Right => _join(r, l, on, on, false),
            FqxJoinType::Inner => _join(l, r, on, on, true),
            FqxJoinType::Outer => _outer_join(l, r, on, on),
        }
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_join {
    use super::*;
    use crate::ops::mock::data::{D7, D8};
    use crate::ops::OpSelect;

    #[test]
    fn join_self_success() {
        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // join left

        let d1 = D7.clone();
        let d2 = D8.clone();

        let res = d1.join(d2.rf(), &["Name"], FqxJoinType::Left);
        println!("join left:");
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }

        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // join right

        let d1 = D7.clone();
        let d2 = D8.clone();

        let res = d1.join(d2, &["Name"], FqxJoinType::Right);
        println!("join right:");
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
    }

    #[test]
    fn join_self_success2() {
        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // join inner

        let d1 = D7.clone();
        let d2 = D8.clone();

        let res = d1.join(d2, &["Name"], FqxJoinType::Inner);
        println!("join inner:");
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
        ///////////////////////////////////////////////////////////////////////////////////////////////////
        // join outer

        let d1 = D7.clone();
        let d2 = D8.clone();

        let res = d1.join(d2, &["Name"], FqxJoinType::Outer);
        println!("join outer:");
        println!("{:?}", res.columns());
        println!("{:?}", res.types());
        for r in res.data().iter() {
            println!("{:?}", r);
        }
    }
}
