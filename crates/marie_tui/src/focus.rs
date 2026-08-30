use crossterm::event::KeyCode;

use crate::focus::Focus::FeaturesTable;

#[derive(Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    UrlInput,
    DownloadButton,
    FeaturesTable,
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
        *self = Self::UrlInput;
    }

    pub const fn right(&mut self) {
        *self = Self::DownloadButton;
    }

    pub const fn up(&mut self) {
        *self = match self {
            Self::FeaturesTable => Self::UrlInput,
            Self::UrlInput | Self::DownloadButton => FeaturesTable,
        }
    }

    pub const fn down(&mut self) {
        *self = match self {
            Self::FeaturesTable => Self::UrlInput,
            Self::UrlInput | Self::DownloadButton => Self::FeaturesTable,
        }
    }
}
