use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::{
    components::input_textbox::InputTextbox,
    core::context::AppContext,
    focus::Focus,
    keyboard::KeyboardAction,
    ui::{component::Component, shared::UiLayout},
};

pub struct UrlInput;

impl Component for UrlInput {
    fn render(
        &mut self,
        frame: &mut Frame,
        layout: &UiLayout,
        ctx: &AppContext,
    ) {
        let layout = layout.url_input;

        InputTextbox::default().set_title("URL").render(
            frame,
            &layout,
            matches!(ctx.focus(), Focus::UrlInput),
            &ctx.url_value(),
        );
    }

    fn handle_key(
        &mut self,
        key: crossterm::event::KeyCode,
        ctx: &AppContext,
    ) -> Option<KeyboardAction> {
        if ctx.focus() != Focus::UrlInput {
            return None;
        }

        match key {
            KeyCode::Esc => {
                ctx.set_focus(Focus::UrlInput);
                Some(KeyboardAction::None)
            }
            KeyCode::Char(c) => {
                let mut val = ctx.url_value();
                val.push(c);

                ctx.set_url_value(&val);
                Some(KeyboardAction::None)
            }
            KeyCode::Backspace => {
                let mut val = ctx.url_value();
                val.pop();

                ctx.set_url_value(&val);
                Some(KeyboardAction::None)
            }
            _ => Some(KeyboardAction::None),
        }
    }
}
