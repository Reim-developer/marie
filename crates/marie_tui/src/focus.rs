use crossterm::event::KeyCode;

use crate::focus::Focus::FeaturesTable;

#[derive(Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    UrlInput,
    DownloadButton,
    FeaturesTable,
    LogPanel,
}

impl Focus {
    pub const fn handle(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => self.left(),
            KeyCode::Right => self.right(),
            KeyCode::Up => self.up(),
            KeyCode::Down => self.down(),
            _ => {}
        }
    }

    pub const fn left(&mut self) {
        *self = match self {
            Self::DownloadButton => Self::UrlInput,
            Self::FeaturesTable => Self::LogPanel,
            Self::LogPanel => Self::FeaturesTable,
            Self::UrlInput => Self::DownloadButton,
        }
    }

    pub const fn right(&mut self) {
        *self = match self {
            Self::DownloadButton => Self::UrlInput,
            Self::UrlInput => Self::DownloadButton,
            Self::LogPanel => Self::FeaturesTable,
            Self::FeaturesTable => Self::LogPanel,
        }
    }

    pub const fn up(&mut self) {
        *self = match self {
            Self::FeaturesTable => Self::UrlInput,
            Self::UrlInput => FeaturesTable,
            Self::LogPanel => Self::DownloadButton,
            Self::DownloadButton => Self::LogPanel,
        }
    }

    pub const fn down(&mut self) {
        *self = match self {
            Self::FeaturesTable => Self::UrlInput,
            Self::UrlInput => Self::FeaturesTable,
            Self::LogPanel => Self::DownloadButton,
            Self::DownloadButton => Self::LogPanel,
        }
    }
}
