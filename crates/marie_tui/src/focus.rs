use crossterm::event::KeyCode;

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
            _ => {}
        }
    }

    pub const fn left(&mut self) {
        *self = Self::UrlInput;
    }

    pub const fn right(&mut self) {
        *self = Self::DownloadButton;
    }
}
