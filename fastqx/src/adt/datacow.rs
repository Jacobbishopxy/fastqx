//! file: datacow.rs
//! author: Jacob Xie
//! date: 2023/11/25 09:30:06 Saturday
//! brief:

use std::borrow::Cow;

use crate::adt::{FqxData, FqxR, FqxRow, FqxValueType};

// ================================================================================================
// FqxDataR
// ================================================================================================

#[derive(Debug)]
pub struct FqxDataCow<'a> {
    pub(crate) columns: Cow<'a, [String]>,
    pub(crate) types: Cow<'a, [FqxValueType]>,
    pub(crate) data: Vec<Cow<'a, FqxRow>>,
}

impl<'a> From<FqxData> for FqxDataCow<'a> {
    fn from(d: FqxData) -> Self {
        FqxDataCow {
            columns: Cow::from(d.columns),
            types: Cow::from(d.types),
            data: d.data.into_iter().map(Cow::Owned).collect(),
        }
    }
}

impl<'a> From<&'a FqxData> for FqxDataCow<'a> {
    fn from(d: &'a FqxData) -> Self {
        FqxDataCow {
            columns: Cow::from(&d.columns),
            types: Cow::from(&d.types),
            data: d.data.iter().map(Cow::Borrowed).collect(),
        }
    }
}

// ================================================================================================
// impl FqxR
// ================================================================================================

impl<'a> FqxR for FqxDataCow<'a> {
    type RowT = Cow<'a, FqxRow>;

    fn columns_(&self) -> &[String] {
        &self.columns
    }

    fn columns_mut_(&mut self) -> &mut [String] {
        self.columns.to_mut()
    }

    fn types_(&self) -> &[FqxValueType] {
        &self.types
    }

    fn types_mut_(&mut self) -> &mut [FqxValueType] {
        self.types.to_mut()
    }

    fn data_(&self) -> &[Self::RowT] {
        &self.data
    }

    fn data_mut_(&mut self) -> &mut [Self::RowT] {
        &mut self.data
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_r {
    use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

    use super::*;
    use crate::mock::data::D1;

    #[test]
    fn get_set_r_success() {
        let d1 = D1.clone();

        let mut r1 = FqxDataCow::from(d1);

        let c = r1.columns_();

        let ans = c.iter().nth(2);

        println!("{:?}", ans);

        r1.columns_mut_().get_mut(2).map(|v| *v = "f".to_string());

        println!("{:?}", r1.columns_());
    }

    trait FromTo {
        fn from_to(&self, max_len: usize) -> (usize, usize);
    }

    impl FromTo for RangeFull {
        fn from_to(&self, max_len: usize) -> (usize, usize) {
            (0, max_len)
        }
    }
    impl FromTo for Range<usize> {
        fn from_to(&self, max_len: usize) -> (usize, usize) {
            (self.start, (self.end - 1).min(max_len))
        }
    }
    impl FromTo for RangeFrom<usize> {
        fn from_to(&self, max_len: usize) -> (usize, usize) {
            (self.start, max_len)
        }
    }
    impl FromTo for RangeInclusive<usize> {
        fn from_to(&self, max_len: usize) -> (usize, usize) {
            (*self.start(), *self.end().min(&max_len))
        }
    }
    impl FromTo for RangeTo<usize> {
        fn from_to(&self, max_len: usize) -> (usize, usize) {
            (0, (self.end - 1).min(max_len))
        }
    }
    impl FromTo for RangeToInclusive<usize> {
        fn from_to(&self, max_len: usize) -> (usize, usize) {
            (0, self.end.min(max_len))
        }
    }

    fn slice_cow<'a, E>(cow: Cow<'a, [E]>, range: impl FromTo) -> Cow<'a, [E]>
    where
        [E]: ToOwned<Owned = Vec<E>>,
    {
        let (start, end) = range.from_to(cow.len());
        if start > end {
            return Cow::Borrowed(&[]);
        }

        match cow {
            Cow::Borrowed(slice) => {
                if let Some(borrowed_slice) = slice.get(start..=end) {
                    Cow::Borrowed(borrowed_slice)
                } else {
                    Cow::Borrowed(&[])
                }
            }
            Cow::Owned(mut vec) => {
                vec.drain(..start);
                vec.truncate(end - start + 1);
                Cow::Owned(vec)
            }
        }
    }

    #[test]
    fn slice_cow_success() {
        let owned_c = vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
        ];

        let borrowed_cow = Cow::Borrowed(&owned_c[..]);

        println!(">>> {:?}", slice_cow(borrowed_cow, 2..6));

        let owned_cow = Cow::Owned(owned_c);
        println!(">>> {:?}", slice_cow(owned_cow, 1..3));
    }

    #[test]
    fn slice_cow_success2() {
        let owned_c = vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
        ];

        let borrowed_cow = Cow::Borrowed(&owned_c[..]);

        println!(">>> {:?}", slice_cow(borrowed_cow, ..=5));

        let owned_cow = Cow::Owned(owned_c);
        println!(">>> {:?}", slice_cow(owned_cow, ..=3));
    }
}
