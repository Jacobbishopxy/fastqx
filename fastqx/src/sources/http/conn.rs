//! file: conn.rs
//! author: Jacob Xie
//! date: 2023/10/02 12:37:03 Monday
//! brief:

use anyhow::Result;
use pyo3::pyclass;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{Client, ClientBuilder, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::adt::FqxData;

// ================================================================================================
// HttpConnector
// ================================================================================================

#[pyclass]
#[derive(Debug, Clone)]
pub struct HttpConnector {
    url: String,
    client: Client,
}

///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! send {
    ($s:expr, $sp:expr, $mtd:ident) => {{
        let pth = format!("{}/{}", $s.url, $sp.as_ref());
        let encoded = Url::parse(&pth)?;
        let res = $s.client.$mtd(encoded).send().await?.json::<R>().await?;

        Ok(res)
    }};
    ($s:expr, $sp:expr, $mtd:ident, $b:expr) => {{
        let pth = format!("{}/{}", $s.url, $sp.as_ref());
        let encoded = Url::parse(&pth)?;
        let res = $s
            .client
            .$mtd(encoded)
            .json($b)
            .send()
            .await?
            .json::<R>()
            .await?;

        Ok(res)
    }};
}

impl HttpConnector {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn new<S: ToString>(url: S, auth: Option<&str>) -> Result<Self> {
        let mut builder = ClientBuilder::new();

        if let Some(a) = auth {
            let mut hd = HeaderMap::new();
            hd.append(AUTHORIZATION, a.parse()?);

            builder = builder.default_headers(hd);
        }

        Ok(Self {
            url: url.to_string(),
            client: builder.build()?,
        })
    }

    pub async fn raw_get<P, R>(&self, subpath: P) -> Result<R>
    where
        P: AsRef<str>,
        R: DeserializeOwned,
    {
        send!(self, subpath, get)
    }

    pub async fn raw_post<P, T, R>(&self, subpath: P, req: &T) -> Result<R>
    where
        P: AsRef<str>,
        T: Serialize,
        R: DeserializeOwned,
    {
        send!(self, subpath, post, req)
    }

    pub async fn raw_put<P, T, R>(&self, subpath: P, req: &T) -> Result<R>
    where
        P: AsRef<str>,
        T: Serialize,
        R: DeserializeOwned,
    {
        send!(self, subpath, put, req)
    }

    pub async fn raw_delete<P, R>(&self, subpath: P) -> Result<R>
    where
        P: AsRef<str>,
        R: DeserializeOwned,
    {
        send!(self, subpath, delete)
    }

    pub async fn raw_patch<P, T, R>(&self, subpath: P, req: &T) -> Result<R>
    where
        P: AsRef<str>,
        T: Serialize,
        R: DeserializeOwned,
    {
        send!(self, subpath, patch, req)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    pub async fn get<P: AsRef<str>>(&self, subpath: P) -> Result<FqxData> {
        self.raw_get::<_, FqxData>(subpath).await
    }

    pub async fn post<P: AsRef<str>, T: Serialize>(&self, subpath: P, req: &T) -> Result<FqxData> {
        self.raw_post::<_, _, FqxData>(subpath, req).await
    }

    pub async fn put<P: AsRef<str>, T: Serialize>(&self, subpath: P, req: &T) -> Result<FqxData> {
        self.raw_put::<_, _, FqxData>(subpath, req).await
    }

    pub async fn patch<P: AsRef<str>, T: Serialize>(&self, subpath: P, req: &T) -> Result<FqxData> {
        self.raw_patch::<_, _, FqxData>(subpath, req).await
    }
}
