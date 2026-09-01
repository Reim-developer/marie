use crate::download_scope::DownloadScope;

#[derive(Debug)]
pub enum AppSignal {
    Download { url: String, scope: DownloadScope },
    Exit,
}
