use crate::{app::App, focus::Focus};
use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy)]
pub enum KeyboardAction {
    None,
    Exit,
    Download,
}

impl KeyboardAction {
    #[must_use]
    pub const fn from_key(key: KeyCode) -> Self {
        match key {
            KeyCode::Esc => Self::Exit,
            _ => Self::None,
        }
    }

    pub fn keyboard(key: KeyCode, app: &mut App) -> Self {
        match key {
            KeyCode::Left | KeyCode::Right => {
                app.focus.handle(key);
                return Self::None;
            }

            KeyCode::Esc => {
                if app.focus != Focus::UrlInput {
                    return Self::Exit;
                }
            }
            _ => match app.focus {
                Focus::UrlInput => {
                    Self::textbox_input(key, &mut app.url_input.value);
                }

                Focus::DownloadButton => {
                    if key == KeyCode::Enter {
                        return Self::Download;
                    }
                }
            },
        }

        Self::None
    }

    pub fn textbox_input(key: KeyCode, text: &mut String) {
        type K = KeyCode;

        match key {
            K::Char(c) => text.push(c),
            K::Backspace => {
                text.pop();
            }
            _ => {}
        }
    }
}
