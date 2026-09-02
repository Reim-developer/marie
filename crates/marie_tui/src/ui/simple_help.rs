use ratatui::Frame;

use crate::{
    components::simple_help::render_simple_help, ui::shared::UiLayout,
};

pub struct SimpleHelp;

impl SimpleHelp {
    pub fn render(frame: &mut Frame, ui_layout: &UiLayout) {
        render_simple_help(frame, &ui_layout.simple_help);
    }
}
