//! file: refd.rs
//! author: Jacob Xie
//! date: 2023/10/12 22:50:44 Thursday
//! brief:

use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

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
// .x[(Range<usize>, Range<usize>)]

// ================================================================================================
// FqxIdx
// ================================================================================================

pub trait FqxIdx {
    fn ty(&self) -> &'static str;
}

type S = usize;
type F = RangeFull;
type R = Range<usize>;
type RF = RangeFrom<usize>;
type RI = RangeInclusive<usize>;
type RT = RangeTo<usize>;
type RTI = RangeToInclusive<usize>;

macro_rules! impl_fqx_idx {
    ($t:ident, $n:expr) => {
        impl FqxIdx for $t {
            fn ty(&self) -> &'static str {
                $n
            }
        }
    };
    ($r:ident, $c:ident, $n:expr) => {
        impl FqxIdx for ($r, $c) {
            fn ty(&self) -> &'static str {
                $n
            }
        }
    };
}

impl_fqx_idx!(S, "S");
impl_fqx_idx!(F, "F");
impl_fqx_idx!(R, "R");
impl_fqx_idx!(RF, "RF");
impl_fqx_idx!(RI, "RI");
impl_fqx_idx!(RT, "RT");
impl_fqx_idx!(RTI, "RTI");

impl_fqx_idx!(S, S, "S,S");
impl_fqx_idx!(S, F, "S,F");
impl_fqx_idx!(S, R, "S,R");
impl_fqx_idx!(S, RF, "S,RF");
impl_fqx_idx!(S, RI, "S,RI");
impl_fqx_idx!(S, RT, "S,RT");
impl_fqx_idx!(S, RTI, "S,RTI");

impl_fqx_idx!(F, S, "F,S");
impl_fqx_idx!(F, F, "F,F");
impl_fqx_idx!(F, R, "F,R");
impl_fqx_idx!(F, RF, "F,RF");
impl_fqx_idx!(F, RI, "F,RI");
impl_fqx_idx!(F, RT, "F,RT");
impl_fqx_idx!(F, RTI, "F,RTI");

impl_fqx_idx!(R, S, "R,S");
impl_fqx_idx!(R, F, "R,F");
impl_fqx_idx!(R, R, "R,R");
impl_fqx_idx!(R, RF, "R,RF");
impl_fqx_idx!(R, RI, "R,RI");
impl_fqx_idx!(R, RT, "R,RT");
impl_fqx_idx!(R, RTI, "R,RTI");

impl_fqx_idx!(RF, S, "RF,S");
impl_fqx_idx!(RF, F, "RF,F");
impl_fqx_idx!(RF, R, "RF,R");
impl_fqx_idx!(RF, RF, "RF,RF");
impl_fqx_idx!(RF, RI, "RF,RI");
impl_fqx_idx!(RF, RT, "RF,RT");
impl_fqx_idx!(RF, RTI, "RF,RTI");

impl_fqx_idx!(RI, S, "RI,S");
impl_fqx_idx!(RI, F, "RI,F");
impl_fqx_idx!(RI, R, "RI,R");
impl_fqx_idx!(RI, RF, "RI,RF");
impl_fqx_idx!(RI, RI, "RI,RI");
impl_fqx_idx!(RI, RT, "RI,RT");
impl_fqx_idx!(RI, RTI, "RI,RTI");

impl_fqx_idx!(RT, S, "RT,S");
impl_fqx_idx!(RT, F, "RT,F");
impl_fqx_idx!(RT, R, "RT,R");
impl_fqx_idx!(RT, RF, "RT,RF");
impl_fqx_idx!(RT, RI, "RT,RI");
impl_fqx_idx!(RT, RT, "RT,RT");
impl_fqx_idx!(RT, RTI, "RT,RTI");

impl_fqx_idx!(RTI, S, "RTI,S");
impl_fqx_idx!(RTI, F, "RTI,F");
impl_fqx_idx!(RTI, R, "RTI,R");
impl_fqx_idx!(RTI, RF, "RTI,RF");
impl_fqx_idx!(RTI, RI, "RTI,RI");
impl_fqx_idx!(RTI, RT, "RTI,RT");
impl_fqx_idx!(RTI, RTI, "RTI,RTI");

// ================================================================================================
// OpX
// ================================================================================================

pub trait OpX<'a> {
    fn x<I>(&'a self, idx: I) -> FqxDataRef<'a>
    where
        I: FqxIdx;
}

// ================================================================================================
// FqxData
// ================================================================================================

impl<'a> OpX<'a> for FqxData {
    fn x<I>(&'a self, _idx: I) -> FqxDataRef<'a>
    where
        I: FqxIdx,
    {
        // TODO: idx

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

        let refd = data.x(1);
        println!("{:?}", refd);
        let refd = data.x(1..);
        println!("{:?}", refd);
        let refd = data.x(..);
        println!("{:?}", refd);
        let refd = data.x((0, 1));
        println!("{:?}", refd);
        let refd = data.x((1, ..2));
        println!("{:?}", refd);
        let refd = data.x((1.., ..=2));
        println!("{:?}", refd);
    }
}
