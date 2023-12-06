//! file: r.rs
//! author: Jacob Xie
//! date: 2023/12/06 16:37:18 Wednesday
//! brief:

pub trait RowProps {
    type Item;

    fn get_nth(&self) -> Option<Self::Item>;
}
