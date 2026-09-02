use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::{
    components::input_textbox::InputTextbox,
    core::context::AppContext,
    focus::Focus,
    keyboard::KeyboardAction,
    ui::{component::Component, shared::UiLayout},
};

pub struct CommandPalette;

impl Component for CommandPalette {
    fn render(
        &mut self,
        frame: &mut Frame,
        ui_layout: &UiLayout,
        ctx: &AppContext,
    ) {
        if let Some(area) = ui_layout.command_palette {
            let focused = ctx.focus() == Focus::CommandPalette;

            InputTextbox::default()
                .set_title("Command Palette: ESC To Exit")
                .render(frame, &area, focused, &ctx.command_value());
        }
    }

    fn handle_key(
        &mut self,
        key: crossterm::event::KeyCode,
        ctx: &AppContext,
    ) -> Option<KeyboardAction> {
        if key == KeyCode::Char(':') && ctx.focus() != Focus::UrlInput {
            ctx.set_command_palette_visible(true);
            ctx.set_focus(Focus::CommandPalette);

            return Some(KeyboardAction::None);
        }

        if ctx.focus() != Focus::CommandPalette {
            return None;
        }

        match key {
            KeyCode::Esc => {
                ctx.set_command_palette_visible(false);
                ctx.set_focus(Focus::UrlInput);

                Some(KeyboardAction::None)
            }
            KeyCode::Char(c) => {
                ctx.add_command_value(c);

                Some(KeyboardAction::None)
            }
            KeyCode::Backspace => {
                ctx.remove_command_value();

                Some(KeyboardAction::None)
            }
            _ => Some(KeyboardAction::None),
        }
    }
}
