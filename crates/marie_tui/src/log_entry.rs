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

#[cfg(test)]
mod tests {
    use super::LogEntry;
    use ratatui::style::Color;

    #[test]
    fn info_is_gray() {
        assert_eq!(LogEntry::Info(String::new()).color(), Color::Gray);
    }

    #[test]
    fn success_is_green() {
        assert_eq!(LogEntry::Success(String::new()).color(), Color::Green);
    }

    #[test]
    fn error_is_red() {
        assert_eq!(LogEntry::Error(String::new()).color(), Color::Red);
    }

    #[test]
    fn text_extracts_inner() {
        assert_eq!(LogEntry::Info("hello".into()).text(), "hello");
    }
}
