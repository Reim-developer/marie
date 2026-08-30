use reqwest::{Client, Response};
use std::sync::Arc;
use thiserror::Error;

pub struct HttpClient {
    inner: Arc<Client>,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Client::new()),
        }
    }

    /// # Errors
    /// Network failure.
    pub async fn fetch(&self, url: impl Into<String>) -> Result<Body, Error> {
        let response = self.inner.get(url.into()).send().await?;

        Ok(Body { inner: response })
    }

    /// # Errors
    /// Network failure.
    pub async fn fetch_text(
        &self,
        url: impl Into<String>,
    ) -> Result<String, Error> {
        self.fetch(url).await?.text().await
    }
}

pub struct Body {
    inner: Response,
}

impl Body {
    /// # Errors
    /// Read body failed.
    pub async fn text(self) -> Result<String, Error> {
        Ok(self.inner.text().await?)
    }

    /// # Errors
    /// Read body failed.
    pub async fn bytes(self) -> Result<Vec<u8>, Error> {
        Ok(self.inner.bytes().await?.to_vec())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("network request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
}
