//! file: refd.rs
//! author: Jacob Xie
//! date: 2023/10/12 22:50:44 Thursday
//! brief:

// use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::adt::{FqxData, FqxValue, FqxValueType};
use crate::ops::FqxRowSelect;

// ================================================================================================
// FqxDataRef
// ================================================================================================

#[derive(Debug)]
pub struct FqxDataRef<'a> {
    pub columns: Vec<&'a String>,
    pub types: Vec<&'a FqxValueType>,
    pub data: Vec<FqxRowSelect<&'a FqxValue>>,
}

// TODO: Slice/Select FqxData -> FqxDataRef
// .x[Range<usize>, Range<usize>]

// ================================================================================================
// OpX
// ================================================================================================

pub trait OpX<'a> {
    fn x(&'a self) -> FqxDataRef<'a>;
}

// ================================================================================================
// FqxData
// ================================================================================================

impl<'a> OpX<'a> for FqxData {
    fn x(&'a self) -> FqxDataRef<'a> {
        FqxDataRef {
            columns: self.columns.iter().collect(),
            types: self.types.iter().collect(),
            data: self
                .data
                .iter()
                .map(|r| FqxRowSelect(r.into_iter().collect()))
                .collect(),
        }
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod tests {

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
    fn test_refd() {
        let data = DATA.clone();

        let refd = data.x();

        println!("{:?}", refd);
    }
}
