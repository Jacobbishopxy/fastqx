//! file: data.rs
//! author: Jacob Xie
//! date: 2023/10/20 18:09:31 Friday
//! brief:

use once_cell::sync::Lazy;

use crate::adt::{FqxD, FqxData, FqxRow};
use crate::fqx;

pub static D1: Lazy<FqxData> = Lazy::new(|| {
    fqx!(
        (1, "A", 1.1),
        (2, "B", 2.2),
        (fqx!(), "C", 3.3),
        (4, fqx!(), 4.4),
        (5, "E", fqx!()),
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
    let mut d = fqx!(("Pear", 11), ("Apple", 22), ("Pear", 12), ("Banana", 15),).unwrap();
    d.set_columns(vec![
        "Fruit".to_string(),
        "Phosphorus (mg/100g)".to_string(),
    ])
    .unwrap();
    d
});

pub static D7: Lazy<FqxData> = Lazy::new(|| {
    let mut d = fqx!(
        ("Apple", 107),
        ("Pear", 358),
        ("Pear", 115),
        ("Pineapple", 200),
        ("Pear", 116),
    )
    .unwrap();
    d.set_columns(vec!["Name".to_string(), "Potassium (mg/100g)".to_string()])
        .unwrap();
    d
});

pub static D8: Lazy<FqxData> = Lazy::new(|| {
    let mut d = fqx!(("Pear", 11), ("Apple", 22), ("Pear", 12), ("Banana", 15),).unwrap();
    d.set_columns(vec!["Name".to_string(), "Phosphorus (mg/100g)".to_string()])
        .unwrap();
    d
});

///////////////////////////////////////////////////////////////////////////////////////////////////

pub static R1: Lazy<FqxRow> = Lazy::new(|| fqx!(2, "A", 3.3));
