//! file: rowcow.rs
//! author: Jacob Xie
//! date: 2023/12/08 15:56:31 Friday
//! brief:

use std::borrow::Cow;
use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use ref_cast::RefCast;
use serde::{Deserialize, Serialize};

use crate::adt::FqxValue;

// TODO

#[derive(
    RefCast, Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord,
)]
#[repr(transparent)]
pub struct FqxRowCow<'a>(pub(crate) Cow<'a, [FqxValue]>);
