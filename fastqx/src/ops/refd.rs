//! file: refd.rs
//! author: Jacob Xie
//! date: 2023/10/12 22:50:44 Thursday
//! brief:

use itertools::Itertools;

use crate::adt::{FqxData, FqxRow, FqxValue, FqxValueType};
use crate::ops::utils::refd_helpers::*;
use crate::ops::{FqxRowSelect, OpCloned};

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

///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> OpCloned for FqxDataRef<'a> {
    type Ret = FqxData;

    fn cloned(self) -> Self::Ret {
        FqxData {
            columns: self.columns.into_iter().map(Clone::clone).collect_vec(),
            types: self.types.into_iter().map(Clone::clone).collect_vec(),
            data: self.data.into_iter().map(FqxRow::from).collect_vec(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

// owned
pub struct FqxRII<'a> {
    inner: std::vec::IntoIter<FqxRowSelect<&'a FqxValue>>,
}

impl<'a> Iterator for FqxRII<'a> {
    type Item = FqxRowSelect<&'a FqxValue>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for FqxDataRef<'a> {
    type Item = FqxRowSelect<&'a FqxValue>;

    type IntoIter = FqxRII<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRII {
            inner: self.data.into_iter(),
        }
    }
}

// ref
pub struct FqxRRefII<'a, 'b>
where
    'a: 'b,
{
    inner: std::slice::Iter<'b, FqxRowSelect<&'a FqxValue>>,
}

impl<'a, 'b> Iterator for FqxRRefII<'a, 'b>
where
    'a: 'b,
{
    type Item = &'b FqxRowSelect<&'a FqxValue>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, 'b> IntoIterator for &'b FqxDataRef<'a> {
    type Item = &'b FqxRowSelect<&'a FqxValue>;

    type IntoIter = FqxRRefII<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRRefII {
            inner: self.data.iter(),
        }
    }
}

// ref mut
pub struct FqxRMutRefII<'a, 'b>
where
    'a: 'b,
{
    inner: std::slice::IterMut<'b, FqxRowSelect<&'a FqxValue>>,
}

impl<'a, 'b> Iterator for FqxRMutRefII<'a, 'b>
where
    'a: 'b,
{
    type Item = &'b mut FqxRowSelect<&'a FqxValue>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, 'b> IntoIterator for &'b mut FqxDataRef<'a>
where
    'a: 'b,
{
    type Item = &'b mut FqxRowSelect<&'a FqxValue>;

    type IntoIter = FqxRMutRefII<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        FqxRMutRefII {
            inner: self.data.iter_mut(),
        }
    }
}

// ================================================================================================
// FqxIdx
// ================================================================================================

pub trait FqxIdx<'a> {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a>;
}

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

impl<'a> FqxIdx<'a> for () {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        row_wise_empty(d)
    }
}

impl<'a> FqxIdx<'a> for &str {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        let p = d.columns.iter().position(|s| self.eq(s.as_str()));

        match p {
            Some(i) => col_wise_s(d, i),
            None => col_wise_empty(d),
        }
    }
}

impl<'a> FqxIdx<'a> for String {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        FqxIdx::cvt(self.as_str(), d)
    }
}

impl<'a> FqxIdx<'a> for &[usize] {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        let mut columns = vec![];
        let mut types = vec![];

        for &p in self.iter() {
            columns.push(d.columns[p]);
            types.push(d.types[p]);
        }

        let data = d
            .data
            .into_iter()
            .map(|r| self.iter().map(|&p| r[p]).collect())
            .collect();

        FqxDataRef {
            columns,
            types,
            data,
        }
    }
}

impl<'a> FqxIdx<'a> for Vec<usize> {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        FqxIdx::cvt(self.as_slice(), d)
    }
}

impl<'a> FqxIdx<'a> for &[&str] {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        let ps = self
            .iter()
            .filter_map(|c| d.columns.iter().position(|&dc| dc == c))
            .collect_vec();

        FqxIdx::cvt(ps, d)
    }
}

impl<'a> FqxIdx<'a> for &[String] {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        let ps = self
            .iter()
            .filter_map(|c| d.columns.iter().position(|&dc| dc == c))
            .collect_vec();

        FqxIdx::cvt(ps, d)
    }
}

impl<'a> FqxIdx<'a> for Vec<String> {
    fn cvt(self, d: FqxDataRef<'a>) -> FqxDataRef<'a> {
        FqxIdx::cvt(self.as_slice(), d)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_refd {
    use once_cell::sync::Lazy;

    use crate::adt::*;
    use crate::ops::OpSelect;

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
    fn iter_success() {
        let data = DATA.clone();

        let refd = data.select(..);

        for r in refd.into_iter() {
            println!("{:?}", r);
        }
    }
}
