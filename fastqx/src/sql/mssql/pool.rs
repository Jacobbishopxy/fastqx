//! file: pool.rs
//! author: Jacob Xie
//! date: 2023/09/16 23:30:46 Saturday
//! brief:

use anyhow::Result;
use async_trait::async_trait;
use bb8::{ManageConnection, Pool};
use futures::TryStreamExt;
use tiberius::error::Error;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use super::FromTiberiusRow;

// ================================================================================================
// MsSqlConnectionManager
// ================================================================================================

#[derive(Debug, Clone)]
pub(crate) struct MsSqlConnectionManager {
    config: Config,
}

impl MsSqlConnectionManager {
    fn new(host: &str, port: Option<u16>, user: &str, pass: &str) -> Result<Self> {
        let mut config = Config::new();

        config.host(host);
        port.map(|p| config.port(p));
        config.authentication(AuthMethod::sql_server(user, pass));
        config.trust_cert();

        Ok(Self { config })
    }
}

#[async_trait]
impl ManageConnection for MsSqlConnectionManager {
    type Connection = Client<Compat<TcpStream>>;

    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let tcp = TcpStream::connect(self.config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        Client::connect(self.config.clone(), tcp.compat_write()).await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        //debug!("Checking {:?}", conn);
        conn.simple_query("").await?.into_row().await?;
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

// ================================================================================================
// PoolMsSql
// ================================================================================================

#[derive(Debug, Clone)]
pub struct PoolMsSql(Pool<MsSqlConnectionManager>);

impl PoolMsSql {
    pub async fn new(host: &str, port: Option<u16>, user: &str, pass: &str) -> Result<Self> {
        let m = MsSqlConnectionManager::new(host, port, user, pass)?;

        let pool = Pool::builder().build(m).await?;

        Ok(Self(pool))
    }

    pub async fn close(self) -> Result<()> {
        // TODO: `close` method takes ownership
        // let conn = self.0.get_owned().await?;
        // conn.close().await?;
        Ok(())
    }

    pub async fn execute(&self, sql: &str) -> Result<()> {
        let mut conn = self.0.get_owned().await?;

        conn.execute(sql, &[]).await?;

        Ok(())
    }

    pub async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: for<'r> FromTiberiusRow<'r>,
    {
        let mut conn = self.0.get_owned().await?;

        let query = conn.simple_query(sql).await?;

        let mut stream = query.into_row_stream();
        let mut res = vec![];

        while let Ok(Some(row)) = stream.try_next().await {
            let r = R::from_row(&row)?;
            res.push(r);
        }

        Ok(res)
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_pool {
    use std::borrow::Cow;

    use futures::TryStreamExt;

    use super::*;

    const HOST: &str = "localhost";
    const USER: &str = "dev";
    const PASS: &str = "StrongPassword123";

    #[tokio::test]
    async fn test_conn() -> Result<()> {
        let m = MsSqlConnectionManager::new(HOST, None, USER, PASS)?;

        let c = m.connect().await;
        assert!(c.is_ok());

        let pool = Pool::builder().build(m).await?;

        let mut pc = pool.get().await?;
        let query = pc.simple_query("select 1 as col").await?;

        let stream = query.into_row_stream();

        let res = stream.try_collect::<Vec<_>>().await?;

        println!("{:?}", res);

        Ok(())
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Users {
        id: i64,
        name: String,
        description: Option<String>,
        score: f32,
    }

    macro_rules! tiberius_err {
        ($s: expr) => {
            tiberius::error::Error::Encoding(Cow::Borrowed($s))
        };
    }

    impl<'r> FromTiberiusRow<'r> for Users {
        fn from_row(row: &'r tiberius::Row) -> std::result::Result<Self, Error> {
            let id: i64 = row.try_get("id")?.ok_or(tiberius_err!("id is None"))?;
            let name: &str = row.try_get("name")?.ok_or(tiberius_err!("name is None"))?;
            let description: Option<&str> = row.try_get("description")?;
            let score: f32 = row
                .try_get("score")?
                .ok_or(tiberius_err!("score is None"))?;

            let users = Users {
                id,
                name: name.to_string(),
                description: description.map(str::to_string),
                score,
            };

            Ok(users)
        }
    }

    #[tokio::test]
    async fn test_fetch() -> anyhow::Result<()> {
        let pool = PoolMsSql::new(HOST, None, USER, PASS).await?;

        let res = pool.fetch::<Users>("select * from users").await?;

        println!("{:?}", res);

        Ok(())
    }
}
