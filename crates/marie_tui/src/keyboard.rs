#[cfg(debug_assertions)]
use crate::utils::{clean_debug_file, debug_to_file};

use crate::{
    app::App, download_scope::DownloadScope, focus::Focus, log_entry::LogEntry,
};
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
                        if app.is_busy() {
                            app.push_log(LogEntry::Error(
                                "Download in progress.".into(),
                            ));
                            return Self::None;
                        }

                        return Self::Download;
                    }
                }
                Focus::FeaturesTable => match key {
                    KeyCode::Char('1') => {
                        app.features_selected = DownloadScope::PageImages;
                    }
                    KeyCode::Char('2') => {
                        app.features_selected = DownloadScope::SiteImages;
                    }
                    _ => {}
                },

                Focus::LogPanel => match key {
                    KeyCode::Char('k') => {
                        app.log_scroll = app.log_scroll.saturating_sub(1);
                    }
                    KeyCode::Char('j') => {
                        app.log_scroll += 1;
                    }
                    KeyCode::Char('h') => {
                        app.log_hscroll = app.log_hscroll.saturating_sub(1);
                    }
                    KeyCode::Char('l') => {
                        app.log_hscroll += 1;
                    }
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
