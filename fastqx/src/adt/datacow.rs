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

    // fn dcst_(self) -> (Self::ColumnsT, Self::TypesT, Self::DataT) {
    //     (self.columns, self.types, self.data)
    // }

    // fn cst_(c: Self::ColumnsT, t: Self::TypesT, d: Self::DataT) -> Self {
    //     Self {
    //         columns: c,
    //         types: t,
    //         data: d,
    //     }
    // }

    // fn height_(&self) -> usize {
    //     self.data.len()
    // }

    // fn width_(&self) -> usize {
    //     self.columns.len()
    // }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_r {
    use std::ops::Range;

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

    fn slice_cow<'a>(cow: Cow<'a, [String]>, rng: Range<usize>) -> Cow<'a, [String]> {
        let start = rng.start;
        match cow {
            Cow::Borrowed(s) => {
                let end = rng.end.min(s.len());
                if let Some(bwd_slice) = s.get(start..end) {
                    Cow::Borrowed(bwd_slice)
                } else {
                    Cow::Borrowed(&[])
                }
            }
            Cow::Owned(mut v) => {
                let end = rng.end.min(v.len());
                v.drain(..start);
                v.truncate(end - start);
                Cow::Owned(v)
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
        ];

        let borrowed_cow = Cow::Borrowed(&owned_c[..]);

        println!(">>> {:?}", slice_cow(borrowed_cow, 2..6));

        let owned_cow = Cow::Owned(owned_c);
        println!(">>> {:?}", slice_cow(owned_cow, 1..3));
    }
}
