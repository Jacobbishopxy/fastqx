//! file: datacow.rs
//! author: Jacob Xie
//! date: 2023/11/25 09:30:06 Saturday
//! brief:

use std::borrow::Cow;

use anyhow::{bail, Result};

use crate::adt::util::{slice_cow, takes_cow};
use crate::adt::{FqxD, FqxData, FqxRow, FqxRowCow, FqxValueType, FromTo, RowProps, SeqSlice};

// ================================================================================================
// FqxDataR
// ================================================================================================

#[derive(Debug, Clone)]
pub struct FqxDataCow<'a> {
    pub(crate) columns: Cow<'a, [String]>,
    pub(crate) types: Cow<'a, [FqxValueType]>,
    pub(crate) data: Vec<FqxRowCow<'a>>,
}

impl<'a> From<FqxData> for FqxDataCow<'a> {
    fn from(d: FqxData) -> Self {
        FqxDataCow {
            columns: Cow::from(d.columns),
            types: Cow::from(d.types),
            data: d.data.into_iter().map(FqxRowCow::from).collect(),
        }
    }
}

impl<'a> From<&'a FqxData> for FqxDataCow<'a> {
    fn from(d: &'a FqxData) -> Self {
        FqxDataCow {
            columns: Cow::from(&d.columns),
            types: Cow::from(&d.types),
            data: d.data.iter().map(FqxRowCow::from).collect(),
        }
    }
}

impl<'a> From<FqxDataCow<'a>> for FqxData {
    fn from(d: FqxDataCow<'a>) -> Self {
        FqxData {
            columns: d.columns.to_vec(),
            types: d.types.to_vec(),
            data: d.data.into_iter().map(FqxRow::from).collect(),
        }
    }
}

// ================================================================================================
// impl SeqSlice
// ================================================================================================

impl<'a> SeqSlice for Cow<'a, [String]> {
    fn empty() -> Self {
        Cow::Borrowed(&[])
    }

    fn sliced<I>(self, range: I) -> Self
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

impl<'a> SeqSlice for Cow<'a, [FqxValueType]> {
    fn empty() -> Self {
        Cow::Borrowed(&[])
    }

    fn sliced<I>(self, range: I) -> Self
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

// ================================================================================================
// impl FqxR
// ================================================================================================

impl<'a> FqxD for FqxDataCow<'a> {
    type ColumnsT = Cow<'a, [String]>;

    type TypesT = Cow<'a, [FqxValueType]>;

    type RowT = FqxRowCow<'a>;

    fn cst(c: Self::ColumnsT, t: Self::TypesT, d: Vec<Self::RowT>) -> Self {
        FqxDataCow {
            columns: c,
            types: t,
            data: d,
        }
    }

    fn dcst(self) -> (Self::ColumnsT, Self::TypesT, Vec<Self::RowT>) {
        (self.columns, self.types, self.data)
    }

    fn columns(&self) -> &[String] {
        &self.columns
    }

    fn columns_mut(&mut self) -> &mut [String] {
        self.columns.to_mut()
    }

    fn set_columns(&mut self, cols: Self::ColumnsT) -> Result<()> {
        if self.width() != cols.len() {
            bail!("length mismatch")
        }

        self.columns = cols;

        Ok(())
    }

    fn columns_take(self) -> Self::ColumnsT {
        self.columns
    }

    fn types(&self) -> &[FqxValueType] {
        &self.types
    }

    fn types_mut(&mut self) -> &mut [FqxValueType] {
        self.types.to_mut()
    }

    fn set_types(&mut self, types: Self::TypesT) -> Result<()> {
        if self.width() != types.len() {
            bail!("length mismatch")
        }

        self.types = types;

        Ok(())
    }

    fn types_take(self) -> Self::TypesT {
        self.types
    }

    fn data(&self) -> &[Self::RowT] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Vec<Self::RowT> {
        &mut self.data
    }

    fn set_data(&mut self, data: Vec<Self::RowT>) -> Result<()> {
        let width = self.width();

        let mut _data = vec![];
        for row in data.into_iter() {
            let mut count = 0;

            for (d, t) in row.0.iter().zip(self.types().iter()) {
                if !d.eq(t) {
                    bail!("type mismatch")
                }
                count += 1;
            }

            if width != count {
                bail!("length mismatch")
            }

            _data.push(row);
        }

        *self.data_mut() = _data;

        Ok(())
    }

    fn data_take(self) -> Vec<Self::RowT> {
        self.data
    }

    fn check_row_validation(&self, row: &Self::RowT) -> bool {
        if self.width() != row.len() {
            return false;
        }

        for (v, t) in row.iter().zip(self.types().into_iter()) {
            if !v.is_type(t) {
                return false;
            }
        }

        true
    }

    fn iter_owned(self) -> std::vec::IntoIter<Self::RowT> {
        self.data.into_iter()
    }

    fn iter(&self) -> std::slice::Iter<'_, Self::RowT> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Self::RowT> {
        self.data.iter_mut()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_r {

    use super::*;

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
