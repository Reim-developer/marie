use std::path::PathBuf;

use crate::{net::HttpClient, scraper::Scraper};

pub struct ImageDownloader {
    save_path: PathBuf,
    website_url: String,
    client: HttpClient,
}

impl ImageDownloader {
    #[must_use]
    pub const fn new(
        save_path: PathBuf,
        website_url: String,
        client: HttpClient,
    ) -> Self {
        Self {
            save_path,
            website_url,
            client,
        }
    }

    #[must_use]
    pub fn save_path(&self) -> PathBuf {
        self.save_path.clone()
    }

    /// # Errors
    /// Download image(s) failed.
    ///
    /// This will returns `Vec<String>`, as is path to the image it just
    /// downloaded.
    pub async fn download(&self) -> Result<Vec<String>, anyhow::Error> {
        let response_body = self.client.fetch(self.website_url.clone()).await?;
        let response_text = response_body.text().await?;
        let scraper = Scraper::new(response_text);
        let response_html = scraper.parse_html();
        let selector = Scraper::selector("img", &response_html)?;

        let src = selector.srcs();

        Ok(src)
    }
}
