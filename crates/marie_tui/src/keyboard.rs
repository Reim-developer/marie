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
        debug_to_file("KeyCode Pressed:", "keyboard.tmp");

        let (ctx_mut, ui_mut) = app.split_mut();
        let ctx = &*ctx_mut;

        match key {
            KeyCode::Left | KeyCode::Right | KeyCode::Down | KeyCode::Up => {
                if ctx.focus() != Focus::CommandPalette {
                    ctx.handle_focus(key);
                }

                return Self::None;
            }
            KeyCode::Esc => {
                if ctx.focus() == Focus::CommandPalette {
                    ctx.set_command_palette_visible(false);
                    ctx.set_focus(Focus::UrlInput);

                    return Self::None;
                }

                if ctx.focus() != Focus::UrlInput {
                    clean_debug_file("keyboard.tmp");
                    return Self::Exit;
                }

                return Self::None;
            }

            KeyCode::Char(':') if ctx.focus() != Focus::UrlInput => {
                ctx.set_command_palette_visible(true);
                ctx.set_focus(Focus::CommandPalette);

                return Self::None;
            }

            _ => {}
        }

        if let Some(action) = ui_mut.handle_key(key, ctx) {
            return action;
        }

        Self::None
    }
}
