//! file: position.rs
//! author: Jacob Xie
//! date: 2023/10/09 21:59:34 Monday
//! brief:

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

    fn find_positions<F>(self, pred: F) -> Self::Ret<Vec<usize>>
    where
        F: FnMut(&Self::Item) -> bool;
}

// ================================================================================================
// Impl
// ================================================================================================

// TODO
