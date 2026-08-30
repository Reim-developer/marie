use ratatui::Frame;

use crate::{
    components::simple_help::render_simple_help, ui::shared::app_layout,
};

pub struct SimpleHelp;

impl SimpleHelp {
    pub fn render(frame: &mut Frame) {
        render_simple_help(frame, &app_layout(frame)[2]);
    }
}
