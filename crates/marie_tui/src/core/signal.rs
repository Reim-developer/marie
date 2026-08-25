#[derive(Debug)]
pub enum AppSignal {
    Download { url: String },
    Exit,
}
