use crate::selector::HtmlSelector;
use scraper::{Html, Selector};

pub struct Scraper {
    pub html: String,
}

impl Scraper {
    #[must_use]
    pub const fn new(html: String) -> Self {
        Self { html }
    }

    #[must_use]
    pub fn parse_html(&self) -> Html {
        Html::parse_document(&self.html)
    }

    /// # Errors
    /// Selector parse failed.
    pub fn selector<'a>(
        query: &str,
        html: &'a Html,
    ) -> Result<HtmlSelector<'a>, anyhow::Error> {
        type S = Selector;
        use anyhow::anyhow;

        let selector = S::parse(query).map_err(|e| anyhow!("{e}"))?;

        Ok(HtmlSelector::new(selector, html))
    }
}
