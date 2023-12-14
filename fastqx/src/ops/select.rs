//! file: select.rs
//! author: Jacob Xie
//! date: 2023/09/25 15:16:03 Monday
//! brief:

use crate::adt::{FqxData, FqxDataCow};
use crate::ops::FqxIdx;

// ================================================================================================
// OpSelect
// ================================================================================================

pub trait OpSelect<'a> {
    fn select<I>(&'a self, idx: I) -> FqxDataCow<'a>
    where
        I: FqxIdx<'a>;

    fn take<I>(self, idx: I) -> Self
    where
        I: FqxIdx<'a>;

    fn rf(&'a self) -> FqxDataCow<'a> {
        self.select(..)
    }
}

// ================================================================================================
// Impl
// ================================================================================================

impl<'a> OpSelect<'a> for FqxData {
    fn select<I>(&'a self, idx: I) -> FqxDataCow<'a>
    where
        I: FqxIdx<'a>,
    {
        let cow = FqxDataCow::from(self);
        idx.cvt_cow(cow)
    }

    fn take<I>(self, idx: I) -> Self
    where
        I: FqxIdx<'a>,
    {
        idx.cvt_own(self)
    }
}

impl<'a> OpSelect<'a> for FqxDataCow<'a> {
    fn select<I>(&'a self, idx: I) -> FqxDataCow<'a>
    where
        I: FqxIdx<'a>,
    {
        idx.cvt_cow(self.clone())
    }

    fn take<I>(self, idx: I) -> Self
    where
        I: FqxIdx<'a>,
    {
        idx.cvt_cow(self)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_select {
    use super::*;
    use crate::ops::mock::data::D1;

    #[test]
    fn select_success() {
        let data = D1.clone();

        let refd = data.select(1);
        println!("{:?}", refd);
        let refd = data.select(..);
        println!("{:?}", refd);
        let refd = data.select(1..2);
        println!("{:?}", refd);
        let refd = data.select(1..);
        println!("{:?}", refd);
        let refd = data.select(1..=2);
        println!("{:?}", refd);
        let refd = data.select(..2);
        println!("{:?}", refd);
        let refd = data.select(..=2);
        println!("{:?}", refd);

        println!();

        let refd = data.select((0, 1));
        println!("{:?}", refd);
        let refd = data.select((0, ..));
        println!("{:?}", refd);
        let refd = data.select((0, 1..2));
        println!("{:?}", refd);
        let refd = data.select((0, 1..));
        println!("{:?}", refd);
        let refd = data.select((0, 1..=2));
        println!("{:?}", refd);
        let refd = data.select((0, ..2));
        println!("{:?}", refd);
        let refd = data.select((0, ..=2));
        println!("{:?}", refd);
    }

    #[test]
    fn select_success2() {
        let data = D1.clone();

        let refd = data.select("c2");
        println!("{:?}", refd);
        let refd = data.select(String::from("c2"));
        println!("{:?}", refd);

        let refd = data.select([2, 0].as_slice());
        println!("{:?}", refd);
        let refd = data.select(vec![2, 0]);
        println!("{:?}", refd);

        let refd = data.select(["c3", "c1"].as_slice());
        println!("{:?}", refd);
        let refd = data.select(vec![String::from("c3"), String::from("c1")]);
        println!("{:?}", refd);
    }

    #[test]
    fn select_select_success() {
        let data = D1.clone();

        let refd1 = data.select([2, 0].as_slice());
        println!("{:?}", refd1);
        let refd2 = refd1.select(1..);
        println!("{:?}", refd2);
    }
}
