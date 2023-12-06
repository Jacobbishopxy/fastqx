//! file: position.rs
//! author: Jacob Xie
//! date: 2023/10/09 21:59:34 Monday
//! brief:

use itertools::Itertools;

use crate::adt::FqxD;

// ================================================================================================
// OpPosition
// ================================================================================================

pub trait OpPosition<const OWNED: bool> {
    type Item;
    type Ret<A>;

    fn find_or_first<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool;

    fn find_or_last<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool;

    fn find_position<F>(self, pred: F) -> Self::Ret<(usize, Self::Item)>
    where
        F: FnMut(&Self::Item) -> bool;

    fn find_positions<F>(self, pred: F) -> Vec<usize>
    where
        F: FnMut(Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<U> OpPosition<true> for U
where
    U: FqxD,
{
    type Item = U::RowT;

    type Ret<A> = Option<A>;

    fn find_or_first<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_or_first(self.data_take().into_iter(), pred)
    }

    fn find_or_last<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_or_last(self.data_take().into_iter(), pred)
    }

    fn find_position<F>(self, pred: F) -> Self::Ret<(usize, Self::Item)>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_position(&mut self.data_take().into_iter(), pred)
    }

    fn find_positions<F>(self, pred: F) -> Vec<usize>
    where
        F: FnMut(Self::Item) -> bool,
    {
        Itertools::positions(self.data_take().into_iter(), pred).collect()
    }
}

impl<'a, U> OpPosition<false> for &'a U
where
    U: FqxD,
{
    type Item = &'a U::RowT;

    type Ret<A> = Option<A>;

    fn find_or_first<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_or_first(self.data().into_iter(), pred)
    }

    fn find_or_last<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_or_last(self.data().into_iter(), pred)
    }

    fn find_position<F>(self, pred: F) -> Self::Ret<(usize, Self::Item)>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_position(&mut self.data().into_iter(), pred)
    }

    fn find_positions<F>(self, pred: F) -> Vec<usize>
    where
        F: FnMut(Self::Item) -> bool,
    {
        Itertools::positions(self.data().into_iter(), pred).collect()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_position {
    use super::*;
    use crate::fqx;
    use crate::mock::data::D2;
    use crate::ops::OpSelect;

    #[test]
    fn position_self_success() {
        let data = D2.clone();

        let foo = (&data).find_positions(|r| r[0] == fqx!(1));
        println!("{:?}", foo);

        let foo = data.find_positions(|r| r[0] == fqx!(1));
        println!("{:?}", foo);
    }

    // #[test]
    // fn position_slice_success() {
    //     let data = D2.clone();

    //     let slice = &data[..];

    //     let foo = slice.find_positions(|r| r[0] == fqx!(1));

    //     println!("{:?}", foo);
    // }

    #[test]
    fn position_selected_success() {
        let data = D2.clone();

        let selected = (&data).select([0, 2].as_slice());
        let foo = selected.find_positions(|r| r[0] == fqx!(1));
        println!("{:?}", foo);

        let selected = data.select([0, 2].as_slice());
        let foo = selected.find_positions(|r| r[0] == fqx!(1));
        println!("{:?}", foo);
    }
}
