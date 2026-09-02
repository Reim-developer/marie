use ratatui::Frame;

use crate::{
    components::input_textbox::InputTextbox, focus::Focus, ui::shared::UiLayout,
};

pub struct UrlInput;

impl UrlInput {
    pub fn render(
        frame: &mut Frame,
        focus: &Focus,
        value: &str,
        layout: &UiLayout,
    ) {
        let layout = layout.url_input;

        InputTextbox::default().set_title("URL").render(
            frame,
            &layout,
            matches!(focus, Focus::UrlInput),
            value,
        );
    }
}
