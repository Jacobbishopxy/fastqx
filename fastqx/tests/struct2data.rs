//! file: struct2data.rs
//! author: Jacob Xie
//! date: 2023/12/17 22:29:05 Sunday
//! brief: struct to FqxData

use fastqx::base::*;
use fastqx::chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use fastqx::fqx;
use fastqx::utils::ChronoHelper;

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
        ChronoHelper.gen_local_datetime_unchecked("19920101.060000"),
        fqx!(),
        ChronoHelper.gen_naive_date_unchecked("19900101"),
        Some(ChronoHelper.gen_naive_time_unchecked("080000")),
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
            ChronoHelper.gen_local_datetime_unchecked("19980201.080000"),
            Some(ChronoHelper.gen_naive_datetime_unchecked("20500101.000000")),
            ChronoHelper.gen_naive_date_unchecked("19950201"),
            fqx!(),
            fqx!(8.1f32),
            fqx!(2u16),
            fqx!(-5i32)
        ),
        fqx!(
            fqx!(3u64),
            "jo",
            fqx!(),
            ChronoHelper.gen_local_datetime_unchecked("19900601.170000"),
            Some(ChronoHelper.gen_naive_datetime_unchecked("20400101.000000")),
            ChronoHelper.gen_naive_date_unchecked("19870601"),
            Some(ChronoHelper.gen_naive_time_unchecked("2103000")),
            fqx!(5.1f32),
            fqx!(4u16),
            fqx!(2i32)
        ),
    ];

    mock.extend(rows).unwrap();

    println!("{:?}", mock);
}
