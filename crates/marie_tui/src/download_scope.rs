#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DownloadScope {
    #[default]
    PageImages,
    SiteImages,
}
