//! file: tpool.rs
//! author: Jacob Xie
//! date: 2023/09/16 23:30:46 Saturday
//! brief:

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use bb8::{ManageConnection, Pool, PooledConnection};
use tiberius::error::Error;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub type PoolConnectionMsSql = PooledConnection<'static, MsSqlConnectionManager>;

// ================================================================================================
// MsSqlConnectionManager
// ================================================================================================

#[derive(Debug, Clone)]
pub struct MsSqlConnectionManager {
    config: Config,
}

const ERR_STR: &str = "connection string format: mssql://<user>:<pass>@<host>:<port>/<db>";

macro_rules! return_fmt_err {
    ($s:expr) => {
        if $s.len() != 2 {
            bail!(ERR_STR)
        }
    };
}

impl MsSqlConnectionManager {
    pub(crate) fn new(
        host: &str,
        port: Option<u16>,
        user: &str,
        pass: &str,
        database: &str,
    ) -> Result<Self> {
        let mut config = Config::new();

        config.host(host);
        port.map(|p| config.port(p));
        config.authentication(AuthMethod::sql_server(user, pass));
        config.database(database);
        config.trust_cert();

        Ok(Self { config })
    }

    pub(crate) fn new_from_str(url: &str) -> Result<Self> {
        let s0 = url.split("://").collect::<Vec<_>>();
        return_fmt_err!(s0);
        if s0[0] != "mssql" {
            bail!("driver must be `mssql`");
        }
        let (user, s1) = s0[1].split_once(':').ok_or(anyhow!(ERR_STR))?;
        let s2 = s1.split('@').collect::<Vec<_>>();
        return_fmt_err!(s2);
        let pass = s2[0];
        let s3 = s2[1].split(":").collect::<Vec<_>>();
        return_fmt_err!(s3);
        let host = s3[0];
        let s4 = s3[1].split('/').collect::<Vec<_>>();
        return_fmt_err!(s4);
        let port = s4[0].parse::<u16>()?;
        let db = s4[1];

        Self::new(host, Some(port), user, pass, db)
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
pub struct PoolMsSql(pub(crate) Pool<MsSqlConnectionManager>);

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_pool {

    use anyhow::anyhow;
    use futures::TryStreamExt;

    use super::*;
    use crate::sources::sql::{FromTiberiusRow, TryGetFromTiberiusRow};

    const HOST: &str = "localhost";
    const USER: &str = "dev";
    const PASS: &str = "StrongPassword123";
    const DB: &str = "devdb";

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_conn() -> Result<()> {
        let m = MsSqlConnectionManager::new(HOST, None, USER, PASS, DB)?;

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

    #[tokio::test]
    async fn test_conn_str() -> Result<()> {
        let s = "mssql://dev:StrongPassword123@localhost:1433/devdb";

        let c = MsSqlConnectionManager::new_from_str(s);
        assert!(c.is_ok());

        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Users {
        id: i64,
        name: String,
        description: Option<String>,
        score: f32,
    }

    impl FromTiberiusRow for Users {
        fn from_row(row: tiberius::Row) -> Result<Self> {
            let id: i64 = row.try_get("id")?.ok_or(anyhow!("id is None"))?;
            let name: &str = row.try_get("name")?.ok_or(anyhow!("name is None"))?;
            let description: Option<&str> = row.try_get("description")?;
            let score: f32 = row.try_get("score")?.ok_or(anyhow!("score is None"))?;

            let users = Users {
                id,
                name: name.to_string(),
                description: description.map(str::to_string),
                score,
            };

            Ok(users)
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Users2 {
        id: i64,
        name: String,
        description: Option<String>,
        score: f32,
    }

    impl FromTiberiusRow for Users2 {
        fn from_row(row: tiberius::Row) -> Result<Self> {
            let id: i64 = TryGetFromTiberiusRow::try_get(&row, "id")?;
            let name: String = TryGetFromTiberiusRow::try_get(&row, "name")?;
            let description: Option<String> = TryGetFromTiberiusRow::try_get(&row, "description")?;
            let score: f32 = TryGetFromTiberiusRow::try_get(&row, "score")?;

            Ok(Self {
                id,
                name,
                description,
                score,
            })
        }
    }
}
