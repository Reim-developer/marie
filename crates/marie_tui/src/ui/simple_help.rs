use ratatui::Frame;

use crate::{
    components::simple_help::render_simple_help,
    core::context::AppContext,
    keyboard::KeyboardAction,
    ui::{component::Component, shared::UiLayout},
};

pub struct SimpleHelp;

impl Component for SimpleHelp {
    fn render(
        &mut self,
        frame: &mut Frame,
        ui_layout: &UiLayout,
        _app_context: &AppContext,
    ) {
        render_simple_help(frame, &ui_layout.simple_help);
    }

    fn handle_key(
        &mut self,
        _key: crossterm::event::KeyCode,
        _ctx: &AppContext,
    ) -> Option<KeyboardAction> {
        None
    }
}
