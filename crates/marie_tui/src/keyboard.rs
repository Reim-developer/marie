#[cfg(debug_assertions)]
use crate::utils::{clean_debug_file, debug_to_file};

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
        debug_to_file(&format!("KeyCodePressed: {key:?}"), "debug.txt");
        match key {
            KeyCode::Left | KeyCode::Up | KeyCode::Down | KeyCode::Right => {
                app.focus.handle(key);
                return Self::None;
            }

            KeyCode::Esc => {
                if app.focus != Focus::UrlInput {
                    clean_debug_file("debug.txt");
                    return Self::Exit;
                }
            }
            _ => match app.focus {
                Focus::UrlInput => {
                    Self::textbox_input(key, &mut app.url_value);
                }

                Focus::DownloadButton => {
                    if key == KeyCode::Enter {
                        return Self::Download;
                    }
                }
                Focus::FeaturesTable => match key {
                    KeyCode::Char('1') => app.features_selected = Some(0),
                    KeyCode::Char('2') => app.features_selected = Some(1),
                    _ => {}
                },
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
