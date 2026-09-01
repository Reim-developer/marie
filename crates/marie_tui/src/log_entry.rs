use ratatui::style::Color;

pub enum LogEntry {
    Info(String),
    Success(String),
    Error(String),
}

impl LogEntry {
    #[must_use]
    pub const fn color(&self) -> Color {
        match self {
            Self::Info(_) => Color::Gray,
            Self::Success(_) => Color::Green,
            Self::Error(_) => Color::Red,
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Self::Info(s) | Self::Success(s) | Self::Error(s) => s,
        }
    }
}

impl From<String> for LogEntry {
    fn from(value: String) -> Self {
        Self::Info(value)
    }
}
