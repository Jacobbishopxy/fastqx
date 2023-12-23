//! file: mock_fqxcst.rs
//! author: Jacob Xie
//! date: 2023/12/23 13:34:14 Saturday
//! brief:

use fastqx::base::*;
use fastqx::chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use fastqx::utils::*;
use mockall::predicate::*;
use mockall::*;
use once_cell::sync::Lazy;

///////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
#[derive(FqxCst, Debug, Clone, PartialEq)]
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

///////////////////////////////////////////////////////////////////////////////////////////////////

#[automock]
trait TestFqxCst<T: 'static> {
    fn cst(&self) -> FqxData;

    fn with_one_row(&self, t: T) -> FqxData;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn test_fqx_push(my_struct: &dyn TestFqxCst<User>, row: User) -> FqxData {
    my_struct.with_one_row(row)
}

///////////////////////////////////////////////////////////////////////////////////////////////////

const COLUMNS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
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
    ]
});

const TYPES: Lazy<Vec<FqxValueType>> = Lazy::new(|| {
    vec![
        fqxt!("u64"),
        fqxt!("string"),
        fqxt!("string"),
        fqxt!("timestamp"),
        fqxt!("datetime"),
        fqxt!("date"),
        fqxt!("time"),
        fqxt!("f32"),
        fqxt!("u16"),
        fqxt!("i32"),
    ]
});

#[test]
fn mock_user_cst_success() {
    let mut mock = MockTestFqxCst::<User>::new();

    mock.expect_cst()
        .return_const(FqxData::new_empty(&*COLUMNS, &*TYPES).unwrap());

    let d = User::new_empty();

    assert_eq!(d.columns(), &*COLUMNS);
    assert_eq!(d.types(), &*TYPES);
}

#[test]
fn mock_user_success() {
    let mut mock = MockTestFqxCst::<User>::new();

    let d = User {
        id: 1,
        name: "jacob".to_string(),
        email: None,
        registry_time: gen_local_datetime_unchecked(1992, 1, 1, 6, 0, 0),
        expired_time: None,
        birthday: gen_naive_date_unchecked(1990, 1, 1),
        cron: Some(gen_naive_time_unchecked(8, 0, 0)),
        init_score: 9.8f32,
        up_time: 1u16,
        rand_int: -2i32,
    };

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

    mock.expect_with_one_row()
        .return_const(FqxData::new(&*COLUMNS, &*TYPES, vec![row.clone()]).unwrap());

    assert_eq!(&row, &test_fqx_push(&mock, d).data()[0]);
}
