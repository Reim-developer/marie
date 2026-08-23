use anyhow::anyhow;
use scraper::{ElementRef, Html, Selector, element_ref::Text};

pub struct HtmlSelector<'a> {
    pub selector: Selector,
    pub document: &'a Html,
}

type HrefsResult<'a> = Result<Vec<&'a str>, anyhow::Error>;
impl<'a> HtmlSelector<'a> {
    #[must_use]
    pub const fn new(selector: Selector, document: &'a Html) -> Self {
        Self { selector, document }
    }

    #[must_use]
    pub fn has(&self) -> bool {
        self.document.select(&self.selector).next().is_some()
    }

    #[must_use]
    pub fn find(&self) -> Option<ElementRef<'a>> {
        self.document.select(&self.selector).next()
    }

    /// # Errors
    /// Parse failed.
    pub fn hrefs(&self) -> HrefsResult<'a> {
        type S = Selector;

        let selector = S::parse("[href]").map_err(|e| anyhow!("{e}"))?;
        let hrefs = self
            .document
            .select(&selector)
            .filter_map(|element| element.value().attr("href"))
            .collect();

        Ok(hrefs)
    }

    #[must_use]
    pub fn text_element(text: Text) -> String {
        text.collect::<String>()
    }
}

#[test]
fn test_has_element() {
    use crate::scraper::Scraper;

    const HTML: &str = "
        <div> <p> Hello Earth ! <p> <div>
    ";

    let html = Scraper::new(HTML.to_string()).parse_html();
    let selector = Scraper::selector("html", &html).unwrap();
    let selector_2 = Scraper::selector("div.z", &html).unwrap();

    assert!(selector.has());
    assert!(!selector_2.has());
}

#[test]
fn test_element() {
    use crate::scraper::Scraper;

    const HTML: &str = "
        <div> <p> Hello Earth ! <p> <div>
    ";

    let html = Scraper::new(HTML.to_string()).parse_html();
    let selector = Scraper::selector("div", &html).unwrap();

    assert!(selector.find().is_some());
    assert_eq!(selector.find().unwrap().value().name(), "div");
}

#[test]
fn test_text_element() {
    use crate::scraper::Scraper;

    const HTML: &str = "
        <div> <p>Hello Earth !<p> <div>
    ";

    let html = Scraper::new(HTML.to_string()).parse_html();
    let selector = Scraper::selector("div p", &html).unwrap();
    let element = selector.find().unwrap();

    assert_eq!(HtmlSelector::text_element(element.text()), "Hello Earth !");
}

#[test]
fn test_href_element() {
    use crate::scraper::Scraper;

    const HTML: &str = "
        <div> 
            <a href='https://google.com'> </a>
            <a href='https://youtube.com'> </a>
        <div>
    ";

    let html = Scraper::new(HTML.to_string()).parse_html();
    let selector = Scraper::selector("[href]", &html).unwrap();
    let hrefs = selector.hrefs().unwrap();

    for href in hrefs {
        assert!(!href.is_empty());
    }
}
