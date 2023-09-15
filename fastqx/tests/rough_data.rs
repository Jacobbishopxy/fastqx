//! file: rough_data.rs
//! author: Jacob Xie
//! date: 2023/09/13 22:01:42 Wednesday
//! brief:

use fastqx::prelude::*;

static CONN_STR: &str = "postgres://dev:devpass@localhost:5437/dev";

#[tokio::test]
async fn dyn_save_success() {
    let data = FastqxData::new(
        vec![String::from("c1"), String::from("c2"), String::from("c3")],
        vec![
            FastqxValueType::I32,
            FastqxValueType::String,
            FastqxValueType::F32,
        ],
        vec![
            vec![
                FastqxValue::I32(1),
                FastqxValue::String(String::from("A")),
                FastqxValue::F32(2.1),
            ],
            vec![
                FastqxValue::I32(2),
                FastqxValue::String(String::from("B")),
                FastqxValue::F32(1.3),
            ],
            vec![
                FastqxValue::I32(3),
                FastqxValue::String(String::from("C")),
                FastqxValue::F32(3.2),
            ],
        ],
    )
    .unwrap();

    let conn = Connector::new(CONN_STR).unwrap();

    let res = conn
        .dyn_save(data, "tmp_table", SaveMode::Override, false)
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn dyn_fetch_success() {
    let conn = Connector::new(CONN_STR).unwrap();

    let data = conn.dyn_fetch("select * from tmp_table").await;
    println!("{:?}", data);
    assert!(data.is_ok());
}
