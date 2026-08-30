#[derive(Clone)]
pub enum AppEvent {
    Log(String),
    Error(String),
}
