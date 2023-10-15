//! file: position.rs
//! author: Jacob Xie
//! date: 2023/10/09 21:59:34 Monday
//! brief:

use itertools::Itertools;

use crate::adt::{FqxRowAbstract, FqxValue};

// ================================================================================================
// OpPosition
// ================================================================================================

pub trait OpPosition<T> {
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

impl<I, V, T, E> OpPosition<FqxRowAbstract<I, V>> for T
where
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
    T: IntoIterator<Item = E>,
    E: Into<FqxRowAbstract<I, V>>,
{
    type Item = E;

    type Ret<A> = Option<A>;

    fn find_or_first<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_or_first(self.into_iter(), pred)
    }

    fn find_or_last<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_or_last(self.into_iter(), pred)
    }

    fn find_position<F>(self, pred: F) -> Self::Ret<(usize, Self::Item)>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_position(&mut self.into_iter(), pred)
    }

    fn find_positions<F>(self, pred: F) -> Vec<usize>
    where
        F: FnMut(Self::Item) -> bool,
    {
        Itertools::positions(self.into_iter(), pred).collect()
    }
}

impl<'a, I, V, T, E> OpPosition<&'a FqxRowAbstract<I, V>> for &'a T
where
    I: IntoIterator<Item = V> + 'a,
    V: Into<FqxValue> + 'a,
    T: ?Sized,
    for<'b> &'b T: IntoIterator<Item = &'b E>,
    E: AsRef<FqxRowAbstract<I, V>> + 'a,
{
    type Item = &'a E;

    type Ret<A> = Option<A>;

    fn find_or_first<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_or_first(self.into_iter(), pred)
    }

    fn find_or_last<F>(self, pred: F) -> Self::Ret<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_or_last(self.into_iter(), pred)
    }

    fn find_position<F>(self, pred: F) -> Self::Ret<(usize, Self::Item)>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        Itertools::find_position(&mut self.into_iter(), pred)
    }

    fn find_positions<F>(self, pred: F) -> Vec<usize>
    where
        F: FnMut(Self::Item) -> bool,
    {
        Itertools::positions(self.into_iter(), pred).collect()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_position {
    use once_cell::sync::Lazy;

    use super::*;
    use crate::adt::*;
    use crate::ops::OpSelect;

    static DATA: Lazy<FqxData> = Lazy::new(|| {
        FqxData::new(
            vec![String::from("c1"), String::from("c2"), String::from("c3")],
            vec![FqxValueType::I32, FqxValueType::String, FqxValueType::F32],
            vec![
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("A")),
                    FqxValue::F32(2.1),
                ],
                vec![
                    FqxValue::I32(2),
                    FqxValue::String(String::from("B")),
                    FqxValue::F32(1.3),
                ],
                vec![
                    FqxValue::I32(1),
                    FqxValue::String(String::from("C")),
                    FqxValue::F32(3.2),
                ],
            ],
        )
        .unwrap()
    });

    #[test]
    fn position_self_success() {
        let data = DATA.clone();

        let foo = (&data).find_positions(|r| r[0] == 1.into());
        println!("{:?}", foo);

        let foo = data.find_positions(|r| r[0] == 1.into());
        println!("{:?}", foo);
    }

    #[test]
    fn position_slice_success() {
        let data = DATA.clone();

        let slice = &data[..];

        let foo = slice.find_positions(|r| r[0] == 1.into());

        println!("{:?}", foo);
    }

    #[test]
    fn position_selected_success() {
        let data = DATA.clone();

        let selected = (&data).select([0, 2].as_slice());
        let foo = selected.find_positions(|r| r[0] == &1.into());
        println!("{:?}", foo);

        let selected = data.select([0, 2].as_slice());
        let foo = selected.find_positions(|r| r[0] == &1.into());
        println!("{:?}", foo);
    }
}
