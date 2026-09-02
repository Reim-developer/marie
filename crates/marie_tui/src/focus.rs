use crossterm::event::KeyCode;

use crate::focus::Focus::FeaturesTable;

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Focus {
    #[default]
    UrlInput,
    DownloadButton,
    FeaturesTable,
    LogPanel,
    CommandPalette,
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
            Self::CommandPalette => Self::CommandPalette,
        }
    }

    pub const fn right(&mut self) {
        *self = match self {
            Self::DownloadButton => Self::UrlInput,
            Self::UrlInput => Self::DownloadButton,
            Self::LogPanel => Self::FeaturesTable,
            Self::FeaturesTable => Self::LogPanel,
            Self::CommandPalette => Self::CommandPalette,
        }
    }

    pub const fn up(&mut self) {
        *self = match self {
            Self::FeaturesTable | Self::CommandPalette => Self::UrlInput,
            Self::UrlInput => FeaturesTable,
            Self::LogPanel => Self::DownloadButton,
            Self::DownloadButton => Self::LogPanel,
        }
    }

    pub const fn down(&mut self) {
        *self = match self {
            Self::FeaturesTable => Self::UrlInput,
            Self::UrlInput => Self::CommandPalette,
            Self::LogPanel => Self::DownloadButton,
            Self::DownloadButton => Self::LogPanel,
            Self::CommandPalette => Self::FeaturesTable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Focus;
    use crossterm::event::KeyCode;

    #[test]
    fn focus_cycle_right() {
        let mut f = Focus::UrlInput;

        f.handle(KeyCode::Right);
        assert!(matches!(f, Focus::DownloadButton));
        f.handle(KeyCode::Right);
        assert!(matches!(f, Focus::UrlInput));
    }

    #[test]
    fn focus_cycle_left() {
        let mut f = Focus::UrlInput;

        f.handle(KeyCode::Left);
        assert!(matches!(f, Focus::DownloadButton));
        f.handle(KeyCode::Left);
        assert!(matches!(f, Focus::UrlInput));
    }

    #[test]
    fn focus_cycle_up() {
        let mut f = Focus::UrlInput;

        f.handle(KeyCode::Up);
        assert!(matches!(f, Focus::FeaturesTable));

        f.handle(KeyCode::Up);
        assert!(matches!(f, Focus::UrlInput));
    }

    #[test]
    fn focus_cycle_down() {
        let mut f = Focus::UrlInput;

        f.handle(KeyCode::Down);
        assert!(matches!(f, Focus::CommandPalette));

        f.handle(KeyCode::Down);
        assert!(matches!(f, Focus::FeaturesTable));
    }

    #[test]
    fn focus_keyboard() {
        let mut f = Focus::UrlInput;

        let up = KeyCode::Up;
        let down = KeyCode::Down;
        let right = KeyCode::Right;
        let left = KeyCode::Left;

        f.handle(up);

        assert!(matches!(f, Focus::FeaturesTable));

        f.handle(left);
        assert!(matches!(f, Focus::LogPanel));

        f.handle(up);
        assert!(matches!(f, Focus::DownloadButton));

        f.handle(left);
        assert!(matches!(f, Focus::UrlInput));

        f.handle(down);
        assert!(matches!(f, Focus::CommandPalette));

        f.handle(left);
        f.handle(up);

        assert!(matches!(f, Focus::UrlInput));

        f.handle(right);
        assert!(matches!(f, Focus::DownloadButton));
    }
}
