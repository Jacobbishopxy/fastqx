//! file: data.rs
//! author: Jacob Xie
//! date: 2023/10/20 18:09:31 Friday
//! brief:

use once_cell::sync::Lazy;

use crate::adt::{FqxData, FqxRow};
use crate::{fqx, fqx_row, fqx_val};

pub static D1: Lazy<FqxData> = Lazy::new(|| {
    fqx!(
        (1, "A", 1.1),
        (2, "B", 2.2),
        (fqx_val!(), "C", 3.3),
        (4, fqx_val!(), 4.4),
        (5, "E", fqx_val!()),
        (6, "F", 6.6),
        (7, "G", 7.7),
        (8, "H", 8.8),
        (9, "I", 9.9),
    )
    .unwrap()
});

pub static D2: Lazy<FqxData> = Lazy::new(|| {
    fqx!(
        (1, "A", 1.1),
        (2, "B", 2.2),
        (3, "C", 3.3),
        (4, "D", 4.4),
        (5, "E", 5.5),
        (6, "F", 6.6),
        (7, "G", 7.7),
        (8, "H", 8.8),
        (9, "I", 9.9),
    )
    .unwrap()
});

pub static D3: Lazy<FqxData> = Lazy::new(|| {
    fqx!(
        (1, "A", 1.1),
        (2, "B", 2.2),
        (3, "C", 3.3),
        (4, "D", 4.4),
        (5, "E", 5.5),
    )
    .unwrap()
});

pub static D4: Lazy<FqxData> = Lazy::new(|| {
    fqx!(
        (1, "A", 1.1),
        (2, "B", 2.2),
        (3, "C", 3.3),
        (2, "D", 4.4),
        (2, "E", 5.5),
    )
    .unwrap()
});

pub static D5: Lazy<FqxData> = Lazy::new(|| {
    fqx!(
        (1, "A", 1.1),
        (2, "B", 2.2),
        (3, "C", 3.3),
        (2, "D", 4.4),
        (1, "E", 5.5),
        (2, "F", 6.6),
        (3, "G", 7.7),
        (3, "H", 8.8),
        (1, "I", 9.9),
    )
    .unwrap()
});

pub static D6: Lazy<FqxData> = Lazy::new(|| {
    fqx!(
        (1, "lA", 1.1),
        (2, "lB", 2.2),
        (3, "lC", 3.3),
        (2, "lD", 4.4),
        (1, "lE", 5.5),
        (2, "lF", 6.6),
        (3, "lG", 7.7),
        (3, "lH", 8.8),
        (1, "lI", 9.9),
    )
    .unwrap()
});

pub static D7: Lazy<FqxData> = Lazy::new(|| {
    fqx!(
        (1, "rA", 1.1),
        (4, "rB", 2.2),
        (1, "rC", 3.3),
        (3, "rD", 4.4),
        (1, "rE", 5.5),
    )
    .unwrap()
});

///////////////////////////////////////////////////////////////////////////////////////////////////

pub static R1: Lazy<FqxRow> = Lazy::new(|| fqx_row!(2, "A", 3.3));