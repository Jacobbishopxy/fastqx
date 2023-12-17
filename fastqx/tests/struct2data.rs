//! file: struct2data.rs
//! author: Jacob Xie
//! date: 2023/12/17 22:29:05 Sunday
//! brief: struct to FqxData

use fastqx::chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use fastqx::{base::*, fqx, fqxt};

// TODO

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
fn struct2fqx() {
    let columns = vec![
        "id",
        "name",
        "email",
        "registry_time",
        "expired_time",
        "birthday",
        "cron",
        "init_score",
        "up_time",
        "rand_int",
    ];

    let types = fqxt![
        "u64",
        "string",
        "string",
        "timestamp",
        "datetime",
        "date",
        "time",
        "f32",
        "u16",
        "i32",
    ];

    let mut mock = FqxData::new_empty(columns, types).unwrap();

    let row = fqx!(
        FqxValue::U64(1),
        "jacob",
        fqx!(),
        Local::now(),
        fqx!(),
        NaiveDate::from_ymd_opt(2040, 12, 31).unwrap(),
        fqx!(),
        FqxValue::F32(9.8),
        FqxValue::U16(1),
        FqxValue::I32(-2)
    );

    mock.push(row).unwrap();

    println!("{:?}", mock);
}
