//! file: datacow.rs
//! author: Jacob Xie
//! date: 2023/11/25 09:30:06 Saturday
//! brief:

use std::borrow::Cow;
use std::collections::HashSet;

use crate::adt::{FqxData, FqxR, FqxValue, FqxValueType, FromTo, SliceRow};

// ================================================================================================
// FqxDataR
// ================================================================================================

#[derive(Debug)]
pub struct FqxDataCow<'a> {
    pub(crate) columns: Cow<'a, [String]>,
    pub(crate) types: Cow<'a, [FqxValueType]>,
    pub(crate) data: Vec<Cow<'a, [FqxValue]>>,
}

impl<'a> From<FqxData> for FqxDataCow<'a> {
    fn from(d: FqxData) -> Self {
        FqxDataCow {
            columns: Cow::from(d.columns),
            types: Cow::from(d.types),
            data: d.data.into_iter().map(|r| Cow::Owned(r.0)).collect(),
        }
    }
}

impl<'a> From<&'a FqxData> for FqxDataCow<'a> {
    fn from(d: &'a FqxData) -> Self {
        FqxDataCow {
            columns: Cow::from(&d.columns),
            types: Cow::from(&d.types),
            data: d.data.iter().map(|r| Cow::Borrowed(&r.0[..])).collect(),
        }
    }
}

// ================================================================================================
// impl SliceRow
// ================================================================================================

impl<'a> SliceRow for Cow<'a, [FqxValue]> {
    fn slice<I>(self, range: I) -> Self
    where
        I: FromTo,
    {
        slice_cow(self, range)
    }

    fn takes<I>(self, indices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        takes_cow(self, indices)
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
        Cow::Borrowed(slice) => slice
            .get(start..=end)
            .map(Cow::Borrowed)
            .unwrap_or(Cow::Borrowed(&[])),
        Cow::Owned(mut vec) => {
            vec.drain(..start);
            vec.truncate(end - start + 1);
            Cow::Owned(vec)
        }
    }
}

fn takes_cow<'a, E>(cow: Cow<'a, [E]>, indices: impl IntoIterator<Item = usize>) -> Cow<'a, [E]>
where
    [E]: ToOwned<Owned = Vec<E>>,
    E: Clone,
{
    let indices = indices.into_iter().collect::<HashSet<_>>();

    match cow {
        Cow::Borrowed(slice) => {
            let v =
                slice
                    .iter()
                    .enumerate()
                    .filter_map(|(i, e)| {
                        if indices.contains(&i) {
                            Some(e.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

            Cow::Owned(v)
        }
        Cow::Owned(vec) => {
            let v = vec
                .into_iter()
                .enumerate()
                .filter_map(|(i, e)| if indices.contains(&i) { Some(e) } else { None })
                .collect::<Vec<_>>();

            Cow::Owned(v)
        }
    }
}

// ================================================================================================
// impl FqxR
// ================================================================================================

impl<'a> FqxR for FqxDataCow<'a> {
    type RowT = Cow<'a, [FqxValue]>;

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

    #[test]
    fn take_cow_success() {
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

        println!(">>> {:?}", takes_cow(borrowed_cow, vec![1, 3, 5]));

        let owned_cow = Cow::Owned(owned_c);
        println!(">>> {:?}", takes_cow(owned_cow, vec![2, 4, 6]));
    }
}
