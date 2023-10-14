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

impl<'a> FqxDataRef<'a> {
    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.columns.len()
    }
}

// ================================================================================================
// FqxIdx
// ================================================================================================

pub trait FqxIdx<'a> {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a>;
}

type S = usize;
type F = RangeFull;
type R = Range<usize>;
type RF = RangeFrom<usize>;
type RI = RangeInclusive<usize>;
type RT = RangeTo<usize>;
type RTI = RangeToInclusive<usize>;

macro_rules! impl_fqx_idx {
    ($t:ident, $f:ident) => {
        impl<'a> FqxIdx<'a> for $t {
            fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
                $f(d, self)
            }
        }
    };
    ($r:ident, $c:ident, $fr:ident, $fc:ident) => {
        impl<'a> FqxIdx<'a> for ($r, $c) {
            fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
                $fc($fr(d, self.0), self.1)
            }
        }
    };
}

impl_fqx_idx!(S, row_wise_s);
impl_fqx_idx!(F, row_wise_f);
impl_fqx_idx!(R, row_wise_r);
impl_fqx_idx!(RF, row_wise_rf);
impl_fqx_idx!(RI, row_wise_ri);
impl_fqx_idx!(RT, row_wise_rt);
impl_fqx_idx!(RTI, row_wise_rti);

impl_fqx_idx!(S, S, row_wise_s, col_wise_s);
impl_fqx_idx!(S, F, row_wise_s, col_wise_f);
impl_fqx_idx!(S, R, row_wise_s, col_wise_r);
impl_fqx_idx!(S, RF, row_wise_s, col_wise_rf);
impl_fqx_idx!(S, RI, row_wise_s, col_wise_ri);
impl_fqx_idx!(S, RT, row_wise_s, col_wise_rt);
impl_fqx_idx!(S, RTI, row_wise_s, col_wise_rti);

impl_fqx_idx!(F, S, row_wise_f, col_wise_s);
impl_fqx_idx!(F, F, row_wise_f, col_wise_f);
impl_fqx_idx!(F, R, row_wise_f, col_wise_r);
impl_fqx_idx!(F, RF, row_wise_f, col_wise_rf);
impl_fqx_idx!(F, RI, row_wise_f, col_wise_ri);
impl_fqx_idx!(F, RT, row_wise_f, col_wise_rt);
impl_fqx_idx!(F, RTI, row_wise_f, col_wise_rti);

impl_fqx_idx!(R, S, row_wise_r, col_wise_s);
impl_fqx_idx!(R, F, row_wise_r, col_wise_f);
impl_fqx_idx!(R, R, row_wise_r, col_wise_r);
impl_fqx_idx!(R, RF, row_wise_r, col_wise_rf);
impl_fqx_idx!(R, RI, row_wise_r, col_wise_ri);
impl_fqx_idx!(R, RT, row_wise_r, col_wise_rt);
impl_fqx_idx!(R, RTI, row_wise_r, col_wise_rti);

impl_fqx_idx!(RF, S, row_wise_rf, col_wise_s);
impl_fqx_idx!(RF, F, row_wise_rf, col_wise_f);
impl_fqx_idx!(RF, R, row_wise_rf, col_wise_r);
impl_fqx_idx!(RF, RF, row_wise_rf, col_wise_rf);
impl_fqx_idx!(RF, RI, row_wise_rf, col_wise_ri);
impl_fqx_idx!(RF, RT, row_wise_rf, col_wise_rt);
impl_fqx_idx!(RF, RTI, row_wise_rf, col_wise_rti);

impl_fqx_idx!(RI, S, row_wise_ri, col_wise_s);
impl_fqx_idx!(RI, F, row_wise_ri, col_wise_f);
impl_fqx_idx!(RI, R, row_wise_ri, col_wise_r);
impl_fqx_idx!(RI, RF, row_wise_ri, col_wise_rf);
impl_fqx_idx!(RI, RI, row_wise_ri, col_wise_ri);
impl_fqx_idx!(RI, RT, row_wise_ri, col_wise_rt);
impl_fqx_idx!(RI, RTI, row_wise_ri, col_wise_rti);

impl_fqx_idx!(RT, S, row_wise_rt, col_wise_s);
impl_fqx_idx!(RT, F, row_wise_rt, col_wise_f);
impl_fqx_idx!(RT, R, row_wise_rt, col_wise_r);
impl_fqx_idx!(RT, RF, row_wise_rt, col_wise_rf);
impl_fqx_idx!(RT, RI, row_wise_rt, col_wise_ri);
impl_fqx_idx!(RT, RT, row_wise_rt, col_wise_rt);
impl_fqx_idx!(RT, RTI, row_wise_rt, col_wise_rti);

impl_fqx_idx!(RTI, S, row_wise_rti, col_wise_s);
impl_fqx_idx!(RTI, F, row_wise_rti, col_wise_f);
impl_fqx_idx!(RTI, R, row_wise_rti, col_wise_r);
impl_fqx_idx!(RTI, RF, row_wise_rti, col_wise_rf);
impl_fqx_idx!(RTI, RI, row_wise_rti, col_wise_ri);
impl_fqx_idx!(RTI, RT, row_wise_rti, col_wise_rt);
impl_fqx_idx!(RTI, RTI, row_wise_rti, col_wise_rti);

// ================================================================================================
// OpX
// ================================================================================================

pub trait OpX<'a> {
    fn x<I>(&'a self, idx: I) -> FqxDataRef<'a>
    where
        I: FqxIdx<'a>;
}

// ================================================================================================
// FqxData
// ================================================================================================

impl<'a> OpX<'a> for FqxData {
    fn x<I>(&'a self, idx: I) -> FqxDataRef<'a>
    where
        I: FqxIdx<'a>,
    {
        let d = FqxDataRef {
            columns: self.columns.iter().collect(),
            types: self.types.iter().collect(),
            data: self
                .data
                .iter()
                .map(|r| FqxRowSelect(r.into_iter().collect()))
                .collect(),
        };

        idx.cvt(d)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// row-wise

fn row_wise_s(d: FqxDataRef, idx: S) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns,
        types: d.types,
        data: d.data.into_iter().nth(idx).map_or(vec![], |r| vec![r]),
    }
}

fn row_wise_f(d: FqxDataRef, _idx: F) -> FqxDataRef {
    d
}

fn row_wise_r(d: FqxDataRef, idx: R) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns,
        types: d.types,
        data: d
            .data
            .into_iter()
            .skip(idx.start)
            .take(idx.end - idx.start)
            .collect(),
    }
}

fn row_wise_rf(d: FqxDataRef, idx: RF) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns,
        types: d.types,
        data: d.data.into_iter().skip(idx.start).collect(),
    }
}

fn row_wise_ri(d: FqxDataRef, idx: RI) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns,
        types: d.types,
        data: d
            .data
            .into_iter()
            .skip(*idx.start())
            .take(*idx.end() - *idx.start() + 1)
            .collect(),
    }
}

fn row_wise_rt(d: FqxDataRef, idx: RT) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns,
        types: d.types,
        data: d.data.into_iter().take(idx.end).collect(),
    }
}

fn row_wise_rti(d: FqxDataRef, idx: RTI) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns,
        types: d.types,
        data: d.data.into_iter().take(idx.end + 1).collect(),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// col-wise

fn col_wise_s(d: FqxDataRef, idx: S) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns.into_iter().nth(idx).map_or(vec![], |c| vec![c]),
        types: d.types.into_iter().nth(idx).map_or(vec![], |t| vec![t]),
        data: d
            .data
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .nth(idx)
                    .map_or(FqxRowSelect(vec![]), |r| FqxRowSelect(vec![r]))
            })
            .collect(),
    }
}

fn col_wise_f(d: FqxDataRef, _idx: F) -> FqxDataRef {
    d
}

fn col_wise_r(d: FqxDataRef, idx: R) -> FqxDataRef {
    FqxDataRef {
        columns: d
            .columns
            .into_iter()
            .skip(idx.start)
            .take(idx.end - idx.start)
            .collect(),
        types: d
            .types
            .into_iter()
            .skip(idx.start)
            .take(idx.end - idx.start)
            .collect(),
        data: d
            .data
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .into_iter()
                    .skip(idx.start)
                    .take(idx.end - idx.start)
                    .collect()
            })
            .collect(),
    }
}

fn col_wise_rf(d: FqxDataRef, idx: RF) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns.into_iter().skip(idx.start).collect(),
        types: d.types.into_iter().skip(idx.start).collect(),
        data: d
            .data
            .into_iter()
            .map(|r| r.into_iter().into_iter().skip(idx.start).collect())
            .collect(),
    }
}

fn col_wise_ri(d: FqxDataRef, idx: RI) -> FqxDataRef {
    FqxDataRef {
        columns: d
            .columns
            .into_iter()
            .skip(*idx.start())
            .take(*idx.end() - *idx.start() + 1)
            .collect(),
        types: d
            .types
            .into_iter()
            .skip(*idx.start())
            .take(*idx.end() - *idx.start() + 1)
            .collect(),
        data: d
            .data
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .into_iter()
                    .skip(*idx.start())
                    .take(*idx.end() - *idx.start() + 1)
                    .collect()
            })
            .collect(),
    }
}

fn col_wise_rt(d: FqxDataRef, idx: RT) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns.into_iter().take(idx.end).collect(),
        types: d.types.into_iter().take(idx.end).collect(),
        data: d
            .data
            .into_iter()
            .map(|r| r.into_iter().into_iter().take(idx.end).collect())
            .collect(),
    }
}

fn col_wise_rti(d: FqxDataRef, idx: RTI) -> FqxDataRef {
    FqxDataRef {
        columns: d.columns.into_iter().take(idx.end + 1).collect(),
        types: d.types.into_iter().take(idx.end + 1).collect(),
        data: d
            .data
            .into_iter()
            .map(|r| r.into_iter().into_iter().take(idx.end + 1).collect())
            .collect(),
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
        let refd = data.x(..);
        println!("{:?}", refd);
        let refd = data.x(1..2);
        println!("{:?}", refd);
        let refd = data.x(1..);
        println!("{:?}", refd);
        let refd = data.x(1..=2);
        println!("{:?}", refd);
        let refd = data.x(..2);
        println!("{:?}", refd);
        let refd = data.x(..=2);
        println!("{:?}", refd);

        println!("");

        let refd = data.x((0, 1));
        println!("{:?}", refd);
        let refd = data.x((0, ..));
        println!("{:?}", refd);
        let refd = data.x((0, 1..2));
        println!("{:?}", refd);
        let refd = data.x((0, 1..));
        println!("{:?}", refd);
        let refd = data.x((0, 1..=2));
        println!("{:?}", refd);
        let refd = data.x((0, ..2));
        println!("{:?}", refd);
        let refd = data.x((0, ..=2));
        println!("{:?}", refd);
    }
}
