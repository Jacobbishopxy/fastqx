//! file: slice.rs
//! author: Jacob Xie
//! date: 2023/09/24 01:31:56 Sunday
//! brief:

use std::ops::{Index, IndexMut, Range};

use anyhow::{anyhow, Result};
use ref_cast::RefCast;

use crate::adt::{FqxData, FqxValue, FqxValueType};

// ================================================================================================
// FqxSlice
// ================================================================================================

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct FqxSlice(pub(crate) [Vec<FqxValue>]);

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

// ================================================================================================
// Index<Range<usize>>
// No boundary check!
// ================================================================================================

impl Index<Range<usize>> for FqxData {
    type Output = FqxSlice;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        FqxSlice::ref_cast(&self.data[index])
    }
}

impl IndexMut<Range<usize>> for FqxData {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        FqxSlice::ref_cast_mut(&mut self.data[index])
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_slice {

    use once_cell::sync::Lazy;

    use super::*;

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
    fn range_success() {
        let data = DATA.clone();

        let foo = &data[0..2];

        println!("{:?}", foo);
    }
}
