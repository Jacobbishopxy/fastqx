//! file: sql_dynamic.rs
//! author: Jacob Xie
//! date: 2023/09/13 22:01:42 Wednesday
//! brief:

use fastqx::prelude::*;
use once_cell::sync::Lazy;

static CONN_PG: &str = "postgresql://dev:devpass@localhost:5437/dev";
static CONN_MS: &str = "mssql://dev:StrongPassword123@localhost:1433/devdb";

static DATA: Lazy<FqxData> = Lazy::new(|| {
    FqxData::new(
        vec![
            String::from("c1"),
            String::from("c2"),
            String::from("c3"),
            String::from("c4"),
            String::from("c5"),
            String::from("c6"),
            String::from("c7"),
        ],
        vec![
            FqxValueType::I16,
            FqxValueType::I32,
            FqxValueType::I64,
            FqxValueType::Bool,
            FqxValueType::String,
            FqxValueType::F32,
            FqxValueType::F64,
        ],
        vec![
            vec![
                FqxValue::I16(1),
                FqxValue::I32(10),
                FqxValue::I64(100),
                FqxValue::Bool(true),
                FqxValue::String(String::from("A")),
                FqxValue::F32(2.1),
                FqxValue::F64(20.1),
            ],
            vec![
                FqxValue::I16(2),
                FqxValue::I32(20),
                FqxValue::I64(200),
                FqxValue::Bool(false),
                FqxValue::String(String::from("B")),
                FqxValue::F32(1.3),
                FqxValue::F64(10.3),
            ],
            vec![
                FqxValue::Null,
                FqxValue::Null,
                FqxValue::Null,
                FqxValue::Null,
                FqxValue::Null,
                FqxValue::Null,
                FqxValue::Null,
            ],
        ],
    )
    .unwrap()
});

#[tokio::test]
async fn dyn_save_pg_success() {
    let conn = SqlConnector::new(CONN_PG).await.unwrap();

    let res = conn
        .dyn_save(DATA.clone(), "tmp_table", SaveMode::Override, false)
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn dyn_fetch_pg_success() {
    let conn = SqlConnector::new(CONN_PG).await.unwrap();

    let data = conn.dyn_fetch("select * from tmp_table").await;
    println!("{:?}", data);
    assert!(data.is_ok());
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn dyn_save_ms_success() {
    let conn = SqlConnector::new(CONN_MS).await.unwrap();

    let res = conn
        .dyn_save(DATA.clone(), "tmp_table", SaveMode::Override, false)
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn dyn_fetch_ms_success() {
    let conn = SqlConnector::new(CONN_MS).await.unwrap();

    let data = conn.dyn_fetch("select * from tmp_table").await;
    println!("{:?}", data);
    assert!(data.is_ok());
}
