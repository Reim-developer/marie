use ratatui::Frame;

use crate::{
    components::input_textbox::InputTextbox, focus::Focus,
    ui::shared::input_layout,
};

pub struct UrlInput;

impl UrlInput {
    pub fn render(frame: &mut Frame, focus: &Focus, value: &str) {
        let layout = input_layout(frame);

        InputTextbox.render(
            frame,
            &layout[0],
            matches!(focus, Focus::UrlInput),
            value,
        );
    }
}
