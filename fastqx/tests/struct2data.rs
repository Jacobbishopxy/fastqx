//! file: struct2data.rs
//! author: Jacob Xie
//! date: 2023/12/17 22:29:05 Sunday
//! brief: struct to FqxData

use fastqx::chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use fastqx::{base::*, fqx};

#[derive(FqxSchema)]
struct User {
    // id: u64,
    name: String,
    email: Option<String>,
    registry_time: DateTime<Local>,
    expired_time: Option<NaiveDateTime>,
    birthday: NaiveDate,
    cron: Option<NaiveTime>,
    init_score: f32,
    // up_time: u16,
    rand_int: i32,
}

#[test]
fn struct2fqx() {
    let mut mock = User::new_empty();

    let row = fqx!(
        // fqx!(1u64),
        "jacob",
        fqx!(),
        Local::now(),
        fqx!(),
        NaiveDate::from_ymd_opt(2040, 12, 31).unwrap(),
        fqx!(),
        fqx!(9.8f32),
        // fqx!(1u16),
        fqx!(-2i32)
    );

    mock.push(row).unwrap();

    println!("{:?}", mock);
}
