//! file: mssql.rs
//! author: Jacob Xie
//! date: 2023/09/16 19:17:17 Saturday
//! brief:

use anyhow::Result;
use futures::TryStreamExt;
use once_cell::sync::Lazy;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("dev", "StrongPassword123"));
    config.trust_cert();

    config
});

#[tokio::test]
async fn connection_success() -> Result<()> {
    let config = CONFIG.clone();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let stream = client.simple_query("select 1 as col").await?;
    let row = stream.into_row().await?;

    let res = row.unwrap().get::<i32, _>("col");
    println!("res: {:?}", res);

    Ok(())
}

#[tokio::test]
async fn get_row_success() -> Result<()> {
    let config = CONFIG.clone();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    // let query = tiberius::Query::new("select 1 as col");

    let query = client.simple_query("select 1 as col").await?;
    let mut stream = query.into_row_stream();

    while let Ok(Some(r)) = stream.try_next().await {
        println!("{:?}", r.columns());

        let ty = r.columns().get(0).unwrap().column_type();
        println!("{:?}", ty);

        let val: Option<i32> = r.try_get(0)?;

        println!("{:?}", val);
    }

    Ok(())
}
