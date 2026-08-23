use reqwest::{Client, Response};

pub struct Http {
    pub client: Client,
    pub url: String,
}

type ResponseObjectResult = Result<ResponseObject, anyhow::Error>;
impl Http {
    #[must_use]
    pub const fn new(url: String, client: Client) -> Self {
        Self { client, url }
    }

    /// # Errors
    /// Send request failed.
    pub async fn send(self) -> ResponseObjectResult {
        let response = self.client.get(self.url).send().await?;

        Ok(ResponseObject { response })
    }
}

pub struct ResponseObject {
    response: Response,
}

type ResponseTextResult = Result<String, anyhow::Error>;
impl ResponseObject {
    #[must_use]
    pub const fn new(response: Response) -> Self {
        Self { response }
    }

    /// # Errors
    /// Get the text from website failed.
    pub async fn text(self) -> ResponseTextResult {
        Ok(self.response.text().await?)
    }
}
