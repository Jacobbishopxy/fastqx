//! file: slice.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:31:56 Sunday
//! brief:

use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use anyhow::{anyhow, Result};
use ref_cast::RefCast;

use crate::adt::{FqxData, FqxRow, FqxValueType};

// ================================================================================================
// FqxSlice
// ================================================================================================

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct FqxSlice(pub(crate) [FqxRow]);

impl FqxSlice {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn uncheck_cast(&mut self, idx: usize, typ: &FqxValueType) -> Result<()> {
        for r in self.0.iter_mut() {
            r[idx].try_cast_mut(typ)?;
        }

        Ok(())
    }

    pub fn cast(&mut self, idx: usize, typ: &FqxValueType) -> Result<()> {
        match self.0.first() {
            Some(r) => {
                if idx >= r.len() {
                    return Err(anyhow!(format!("idx: {} out of boundary {}", idx, r.len())));
                } else {
                    self.uncheck_cast(idx, typ)?;
                    Ok(())
                }
            }
            None => return Ok(()),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> IntoIterator for &'a mut FqxSlice {
    type Item = &'a mut FqxRow;

    type IntoIter = std::slice::IterMut<'a, FqxRow>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a> IntoIterator for &'a FqxSlice {
    type Item = &'a FqxRow;

    type IntoIter = std::slice::Iter<'a, FqxRow>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// ================================================================================================
// Index<Range<usize>>
// No boundary check!
// ================================================================================================

impl Index<usize> for FqxData {
    type Output = FqxRow;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for FqxData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_index_range {
    () => {
        impl Index<RangeFull> for FqxData {
            type Output = FqxSlice;

            fn index(&self, index: RangeFull) -> &Self::Output {
                FqxSlice::ref_cast(&self.data[index])
            }
        }

        impl IndexMut<RangeFull> for FqxData {
            fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
                FqxSlice::ref_cast_mut(&mut self.data[index])
            }
        }
    };
    ($t:ident) => {
        impl Index<$t<usize>> for FqxData {
            type Output = FqxSlice;

            fn index(&self, index: $t<usize>) -> &Self::Output {
                FqxSlice::ref_cast(&self.data[index])
            }
        }

        impl IndexMut<$t<usize>> for FqxData {
            fn index_mut(&mut self, index: $t<usize>) -> &mut Self::Output {
                FqxSlice::ref_cast_mut(&mut self.data[index])
            }
        }
    };
}

impl_index_range!();
impl_index_range!(Range);
impl_index_range!(RangeFrom);
impl_index_range!(RangeTo);
impl_index_range!(RangeToInclusive);
impl_index_range!(RangeInclusive);

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_slice {

    use once_cell::sync::Lazy;

    use super::*;
    use crate::adt::FqxValue;

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
    fn range_1_success() {
        let data = DATA.clone();

        let foo = &data[0];

        println!("{:?}", foo);
    }

    #[test]
    fn range_2_success() {
        let data = DATA.clone();

        let foo = &data[0..2];

        println!("{:?}", foo);
    }

    #[test]
    fn range_3_success() {
        let data = DATA.clone();

        let foo = &data[1..];

        println!("{:?}", foo);
    }
}
