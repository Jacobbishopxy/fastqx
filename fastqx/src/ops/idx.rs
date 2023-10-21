//! file: idx.rs
//! author: Jacob Xie
//! date: 2023/10/16 14:59:56 Monday
//! brief:

use itertools::Itertools;

use crate::adt::ab::d::*;
use crate::adt::{FqxData, FqxValue, FqxValueType};
use crate::ops::FqxDataRef;

// ================================================================================================
// FqxIdx
// ================================================================================================

pub trait FqxIdx<'a> {
    fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a>;
    fn cvt_own(self, d: FqxData) -> FqxData;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_fqx_idx {
    ($t:ident, $f:ident) => {
        impl<'a> FqxIdx<'a> for $t {
            fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
                d.$f(self)
            }

            fn cvt_own(self, d: FqxData) -> FqxData {
                d.$f(self)
            }
        }
    };
    ($r:ident, $c:ident, $fr:ident, $fc:ident) => {
        impl<'a> FqxIdx<'a> for ($r, $c) {
            fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
                d.$fr(self.0).$fc(self.1)
            }

            fn cvt_own(self, d: FqxData) -> FqxData {
                d.$fr(self.0).$fc(self.1)
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

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> FqxIdx<'a> for () {
    fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        d.row_wise_empty()
    }

    fn cvt_own(self, d: FqxData) -> FqxData {
        d.row_wise_empty()
    }
}

impl<'a> FqxIdx<'a> for &str {
    fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        let p = d.columns.iter().position(|s| self.eq(s.as_str()));

        match p {
            Some(i) => d.col_wise_s(i),
            None => d.col_wise_empty(),
        }
    }

    fn cvt_own(self, d: FqxData) -> FqxData {
        let p = d.columns.iter().position(|s| self.eq(s.as_str()));

        match p {
            Some(i) => d.col_wise_s(i),
            None => d.col_wise_empty(),
        }
    }
}

impl<'a> FqxIdx<'a> for String {
    fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        FqxIdx::cvt_ref(self.as_str(), d)
    }

    fn cvt_own(self, d: FqxData) -> FqxData {
        FqxIdx::cvt_own(self.as_str(), d)
    }
}

impl<'a> FqxIdx<'a> for &[usize] {
    fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        let len = d.width();
        let mut columns = vec![];
        let mut types = vec![];

        for &p in self.iter() {
            if p < len {
                columns.push(d.columns[p]);
                types.push(d.types[p]);
            }
        }

        let data = d
            .data
            .into_iter()
            .map(|r| {
                self.iter()
                    .filter_map(|&p| if p < len { Some(r[p]) } else { None })
                    .collect()
            })
            .collect();

        FqxDataRef {
            columns,
            types,
            data,
        }
    }

    fn cvt_own(self, mut d: FqxData) -> FqxData {
        let len = d.width();
        let mut columns = vec![];
        let mut types = vec![];

        for &p in self.iter() {
            if p < len {
                let c = std::mem::replace(&mut d.columns[p], String::new());
                columns.push(c);
                let t = std::mem::replace(&mut d.types[p], FqxValueType::Null);
                types.push(t);
            }
        }

        let data = d
            .data
            .into_iter()
            .map(|mut r| {
                self.iter()
                    .filter_map(|&p| {
                        if p < len {
                            let e = std::mem::replace(&mut r[p], FqxValue::Null);
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        FqxData {
            columns,
            types,
            data,
        }
    }
}

impl<'a> FqxIdx<'a> for Vec<usize> {
    fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        FqxIdx::cvt_ref(self.as_slice(), d)
    }

    fn cvt_own(self, d: FqxData) -> FqxData {
        FqxIdx::cvt_own(self.as_slice(), d)
    }
}

impl<'a> FqxIdx<'a> for &[&str] {
    fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        let ps = self
            .iter()
            .filter_map(|c| d.columns.iter().position(|&dc| dc == c))
            .collect_vec();

        FqxIdx::cvt_ref(ps, d)
    }

    fn cvt_own(self, d: FqxData) -> FqxData {
        let ps = self
            .iter()
            .filter_map(|c| d.columns.iter().position(|dc| dc == c))
            .collect_vec();

        FqxIdx::cvt_own(ps, d)
    }
}

impl<'a> FqxIdx<'a> for &[String] {
    fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        let ps = self
            .iter()
            .filter_map(|c| d.columns.iter().position(|&dc| dc == c))
            .collect_vec();

        FqxIdx::cvt_ref(ps, d)
    }

    fn cvt_own(self, d: FqxData) -> FqxData {
        let ps = self
            .iter()
            .filter_map(|c| d.columns.iter().position(|dc| dc == c))
            .collect_vec();

        FqxIdx::cvt_own(ps, d)
    }
}

impl<'a> FqxIdx<'a> for Vec<String> {
    fn cvt_ref(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        FqxIdx::cvt_ref(self.as_slice(), d)
    }

    fn cvt_own(self, d: FqxData) -> FqxData {
        FqxIdx::cvt_own(self.as_slice(), d)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_idx {
    use crate::mock::data::D1;
    use crate::ops::OpSelect;

    #[test]
    fn iter_success() {
        let data = D1.clone();

        let refd = data.select(..);
        for r in refd.into_iter() {
            println!("{:?}", r);
        }

        let take = data.take(1..);
        for r in take.into_iter() {
            println!("{:?}", r);
        }
    }

    #[test]
    fn iter_success2() {
        let data = D1.clone();

        // Out of range, on purpose
        let refd = data.select([0, 4].as_slice());
        for r in refd.into_iter() {
            println!("{:?}", r);
        }

        let take = data.take([0, 4].as_slice());
        for r in take.into_iter() {
            println!("{:?}", r);
        }
    }
}
