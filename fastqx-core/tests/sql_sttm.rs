//! file: sql_sttm.rs
//! author: Jacob Xie
//! date: 2023/09/10 00:03:25 Sunday
//! brief: https://docs.rs/sea-query/latest/sea_query/index.html

use sea_query::{
    Alias, ColumnDef, ColumnType, MysqlQueryBuilder, PostgresQueryBuilder, Query,
    SqliteQueryBuilder, Table, Value,
};

#[test]
fn create_table() {
    let table = Table::create()
        .table(Alias::new("tmp_table"))
        .if_not_exists()
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Alias::new("font_size")).integer().not_null())
        .col(ColumnDef::new(Alias::new("charactor")).string().not_null())
        .col(ColumnDef::new_with_type(Alias::new("size_w"), ColumnType::Integer).not_null())
        .col(ColumnDef::new(Alias::new("size_h")).integer().not_null())
        .col(
            ColumnDef::new(Alias::new("font_id"))
                .integer()
                .default(Value::Int(None)),
        )
        .to_owned();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
    println!("{:?}", table.to_string(SqliteQueryBuilder));
}

#[test]
fn drop_table() {
    let table = Table::drop().table(Alias::new("tmp_table")).to_owned();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
    println!("{:?}", table.to_string(SqliteQueryBuilder));
}

#[test]
fn truncate_table() {
    let table = Table::truncate().table(Alias::new("tmp_table")).to_owned();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
}

struct TmpTable {
    aspect: f32,
    image: String,
}

#[test]
fn insert_data() -> anyhow::Result<()> {
    let mut query = Query::insert();
    query
        .into_table(Alias::new("tmp_table"))
        .columns([Alias::new("aspect"), Alias::new("image")]);

    // let query = query
    //     .values([5.15.into(), "12A".into()])?
    //     .values([4.21.into(), "123".into()])?
    //     .to_owned();

    // let data = vec![[5.15.into(), "12A".into()], [4.21.into(), "123".into()]];
    let data = vec![
        TmpTable {
            aspect: 5.15,
            image: String::from("12A"),
        },
        TmpTable {
            aspect: 4.21,
            image: String::from("123"),
        },
    ];

    // for d in data.into_iter() {
    //     query.values([ d.aspect.into(), d.image.into()])?;
    // }

    for TmpTable { aspect, image } in data.into_iter() {
        query.values([aspect.into(), image.into()])?;
    }

    let query = query.to_owned();

    println!("{:?}", query.to_string(MysqlQueryBuilder));
    println!("{:?}", query.to_string(PostgresQueryBuilder));
    println!("{:?}", query.to_string(SqliteQueryBuilder));

    Ok(())
}
