//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/09/28 18:02:11 Thursday
//! brief:

use std::cmp::Ordering;

use crate::adt::{FqxRowAbstract, FqxValue, FqxValueType};

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn merge_bool_to_ordering(b: bool) -> Ordering {
    if b {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

pub(crate) fn sort_bool_to_ordering(b: bool) -> Ordering {
    if b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn get_min(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Less) = a.partial_cmp(&b) {
        a
    } else {
        b
    }
}

pub(crate) fn get_max(a: FqxValue, b: FqxValue) -> FqxValue {
    if let Some(Ordering::Greater) = a.partial_cmp(&b) {
        a
    } else {
        b
    }
}

pub(crate) fn calc_mean<I, V, E>(row_of_sum: E, count: usize) -> E
where
    E: Into<FqxRowAbstract<I, V>>,
    E: From<Vec<FqxValue>>,
    I: IntoIterator<Item = V>,
    V: Into<FqxValue>,
{
    let inner = row_of_sum
        .into()
        .0
        .into_iter()
        .map(|e| {
            let numer = e
                .into()
                .try_cast(&FqxValueType::F64)
                .unwrap_or(FqxValue::Null);
            let denom = FqxValue::F64(count as f64);

            numer / denom
        })
        .collect::<Vec<_>>();

    E::from(inner)
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) mod refd_helpers {
    use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

    use crate::ops::{FqxDataRef, FqxRowSelect};

    pub(crate) type S = usize;
    pub(crate) type F = RangeFull;
    pub(crate) type R = Range<usize>;
    pub(crate) type RF = RangeFrom<usize>;
    pub(crate) type RI = RangeInclusive<usize>;
    pub(crate) type RT = RangeTo<usize>;
    pub(crate) type RTI = RangeToInclusive<usize>;

    // ================================================================================================
    // row-wise
    // ================================================================================================

    pub(crate) fn row_wise_empty(d: FqxDataRef) -> FqxDataRef {
        FqxDataRef { data: vec![], ..d }
    }

    pub(crate) fn row_wise_s(d: FqxDataRef, idx: S) -> FqxDataRef {
        FqxDataRef {
            columns: d.columns,
            types: d.types,
            data: d.data.into_iter().nth(idx).map_or(vec![], |r| vec![r]),
        }
    }

    pub(crate) fn row_wise_f(d: FqxDataRef, _idx: F) -> FqxDataRef {
        d
    }

    pub(crate) fn row_wise_r(d: FqxDataRef, idx: R) -> FqxDataRef {
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

    pub(crate) fn row_wise_rf(d: FqxDataRef, idx: RF) -> FqxDataRef {
        FqxDataRef {
            columns: d.columns,
            types: d.types,
            data: d.data.into_iter().skip(idx.start).collect(),
        }
    }

    pub(crate) fn row_wise_ri(d: FqxDataRef, idx: RI) -> FqxDataRef {
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

    pub(crate) fn row_wise_rt(d: FqxDataRef, idx: RT) -> FqxDataRef {
        FqxDataRef {
            columns: d.columns,
            types: d.types,
            data: d.data.into_iter().take(idx.end).collect(),
        }
    }

    pub(crate) fn row_wise_rti(d: FqxDataRef, idx: RTI) -> FqxDataRef {
        FqxDataRef {
            columns: d.columns,
            types: d.types,
            data: d.data.into_iter().take(idx.end + 1).collect(),
        }
    }

    // ================================================================================================
    // col-wise
    // ================================================================================================

    pub(crate) fn col_wise_empty(d: FqxDataRef) -> FqxDataRef {
        FqxDataRef {
            columns: vec![],
            types: vec![],
            data: vec![FqxRowSelect(vec![]); d.height()],
        }
    }

    pub(crate) fn col_wise_s(d: FqxDataRef, idx: S) -> FqxDataRef {
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

    pub(crate) fn col_wise_f(d: FqxDataRef, _idx: F) -> FqxDataRef {
        d
    }

    pub(crate) fn col_wise_r(d: FqxDataRef, idx: R) -> FqxDataRef {
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
                        .skip(idx.start)
                        .take(idx.end - idx.start)
                        .collect()
                })
                .collect(),
        }
    }

    pub(crate) fn col_wise_rf(d: FqxDataRef, idx: RF) -> FqxDataRef {
        FqxDataRef {
            columns: d.columns.into_iter().skip(idx.start).collect(),
            types: d.types.into_iter().skip(idx.start).collect(),
            data: d
                .data
                .into_iter()
                .map(|r| r.into_iter().skip(idx.start).collect())
                .collect(),
        }
    }

    pub(crate) fn col_wise_ri(d: FqxDataRef, idx: RI) -> FqxDataRef {
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
                        .skip(*idx.start())
                        .take(*idx.end() - *idx.start() + 1)
                        .collect()
                })
                .collect(),
        }
    }

    pub(crate) fn col_wise_rt(d: FqxDataRef, idx: RT) -> FqxDataRef {
        FqxDataRef {
            columns: d.columns.into_iter().take(idx.end).collect(),
            types: d.types.into_iter().take(idx.end).collect(),
            data: d
                .data
                .into_iter()
                .map(|r| r.into_iter().take(idx.end).collect())
                .collect(),
        }
    }

    pub(crate) fn col_wise_rti(d: FqxDataRef, idx: RTI) -> FqxDataRef {
        FqxDataRef {
            columns: d.columns.into_iter().take(idx.end + 1).collect(),
            types: d.types.into_iter().take(idx.end + 1).collect(),
            data: d
                .data
                .into_iter()
                .map(|r| r.into_iter().take(idx.end + 1).collect())
                .collect(),
        }
    }
}
