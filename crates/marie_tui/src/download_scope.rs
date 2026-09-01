#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DownloadScope {
    #[default]
    PageImages,
    SiteImages,
}

#[cfg(test)]
mod tests {
    use super::DownloadScope;

    #[test]
    fn default_is_page_images() {
        assert!(matches!(
            DownloadScope::default(),
            DownloadScope::PageImages
        ));
    }
}
