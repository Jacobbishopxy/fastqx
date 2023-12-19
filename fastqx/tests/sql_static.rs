//! file: sql_static.rs
//! author: Jacob Xie
//! date: 2023/09/10 09:34:02 Sunday
//! brief:

use fastqx::chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use fastqx::prelude::*;
use fastqx::utils::*;
use once_cell::sync::Lazy;

#[derive(Clone, FqxSql, Debug)]
struct Users {
    #[fastqx(primary_key, auto_increment)]
    id: i64,
    #[fastqx(unique_key)]
    name: String,
    email: Option<String>,
    registry_time: DateTime<Local>,
    expired_time: Option<NaiveDateTime>,
    birthday: NaiveDate,
    cron: Option<NaiveTime>,
    init_score: f32,
    up_time: u16,
    rand_int: i32,
    small_ui: u8,
}

static DATA: Lazy<Vec<Users>> = Lazy::new(|| {
    vec![
        Users {
            id: 1,
            name: String::from("Jacob"),
            email: None,
            registry_time: gen_local_datetime_unchecked(1992, 1, 1, 6, 0, 0),
            expired_time: None,
            birthday: gen_naive_date_unchecked(1990, 1, 1),
            cron: Some(gen_naive_time_unchecked(8, 0, 0)),
            init_score: 9.8,
            up_time: 1,
            rand_int: 7,
            small_ui: 7,
        },
        Users {
            id: 2,
            name: String::from("Mia"),
            email: Some(String::from("Mia@fastqx.com")),
            registry_time: gen_local_datetime_unchecked(1998, 2, 1, 11, 0, 0),
            expired_time: Some(gen_naive_datetime_unchecked(2050, 1, 1, 0, 0, 0)),
            birthday: gen_naive_date_unchecked(1995, 2, 1),
            cron: None,
            init_score: 8.8,
            up_time: 2,
            rand_int: -2,
            small_ui: 9,
        },
        Users {
            id: 3,
            name: String::from("White"),
            email: Some(String::from("J.W@fastqx.com")),
            registry_time: gen_local_datetime_unchecked(1990, 6, 1, 17, 0, 0),
            expired_time: Some(gen_naive_datetime_unchecked(2040, 1, 1, 0, 0, 0)),
            birthday: gen_naive_date_unchecked(1987, 6, 1),
            cron: Some(gen_naive_time_unchecked(21, 30, 0)),
            init_score: 8.6,
            up_time: 3,
            rand_int: 6,
            small_ui: 3,
        },
    ]
});

#[test]
fn derive_success() {
    let driver = Driver::POSTGRES;

    let create_table = Users::create_table(&driver);
    println!("{:?}", create_table);

    let drop_table = Users::drop_table(&driver);
    println!("{:?}", drop_table);

    let insert = Users::insert(&driver, DATA.clone());
    println!("{:?}", insert);
}

#[tokio::test]
async fn to_postgres_success() {
    let conn_str = "postgres://dev:devpass@localhost:5437/dev";

    let conn = SqlConnector::new(conn_str).await.unwrap();

    // 1. create table

    // let create_table = Users::create_table().to_string(PostgresQueryBuilder);
    // conn.execute(&create_table).await.unwrap();

    // 2. insert data

    conn.save(DATA.clone(), SaveMode::Override).await.unwrap();

    // 3. query data

    let res = conn.fetch::<Users>("select * from users").await.unwrap();

    println!("{:?}", res);
}

#[tokio::test]
async fn to_mssql_success() {
    let conn_str = "mssql://dev:StrongPassword123@localhost:1433/devdb";

    let conn = SqlConnector::new(conn_str).await.unwrap();

    conn.save(DATA.clone(), SaveMode::Override).await.unwrap();

    let res = conn.fetch::<Users>("select * from users").await.unwrap();

    println!("{:?}", res);
}
