//! file: struct2data.rs
//! author: Jacob Xie
//! date: 2023/12/17 22:29:05 Sunday
//! brief: struct to FqxData

use fastqx::base::*;
use fastqx::chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use fastqx::fqx;
use fastqx::utils::*;

#[allow(dead_code)]
#[derive(FqxCst)]
struct User {
    id: u64,
    name: String,
    email: Option<String>,
    registry_time: DateTime<Local>,
    expired_time: Option<NaiveDateTime>,
    birthday: NaiveDate,
    cron: Option<NaiveTime>,
    init_score: f32,
    up_time: u16,
    rand_int: i32,
}

#[test]
fn columns_and_types() {
    let cols = User::columns();
    let types = User::types();

    println!("{:?}", cols);
    println!("{:?}", types);
}

#[test]
fn struct2fqx() {
    let mut mock = User::new_empty();

    let row = fqx!(
        fqx!(1u64),
        "jacob",
        fqx!(),
        gen_local_datetime_unchecked(1992, 1, 1, 6, 0, 0),
        fqx!(),
        gen_naive_date_unchecked(1990, 1, 1),
        Some(gen_naive_time_unchecked(8, 0, 0)),
        fqx!(9.8f32),
        fqx!(1u16),
        fqx!(-2i32)
    );

    mock.push(row).unwrap();

    let rows = vec![
        fqx!(
            fqx!(2u64),
            "mia",
            fqx!(),
            gen_local_datetime_unchecked(1998, 2, 1, 11, 0, 0),
            Some(gen_naive_datetime_unchecked(2050, 1, 1, 0, 0, 0)),
            gen_naive_date_unchecked(1995, 2, 1),
            fqx!(),
            fqx!(8.1f32),
            fqx!(2u16),
            fqx!(-5i32)
        ),
        fqx!(
            fqx!(3u64),
            "jo",
            fqx!(),
            gen_local_datetime_unchecked(1990, 6, 1, 17, 0, 0),
            Some(gen_naive_datetime_unchecked(2040, 1, 1, 0, 0, 0)),
            gen_naive_date_unchecked(1987, 6, 1),
            Some(gen_naive_time_unchecked(21, 30, 0)),
            fqx!(5.1f32),
            fqx!(4u16),
            fqx!(2i32)
        ),
    ];

    mock.extend(rows).unwrap();

    println!("{:?}", mock);
}
