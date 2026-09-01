use ratatui::Frame;

use crate::{
    components::button::Button, focus::Focus, ui::shared::input_layout,
};

pub struct DownloadButton;

impl DownloadButton {
    pub fn render(frame: &mut Frame, focus: &Focus, disabled: bool) {
        let focused = matches!(focus, Focus::DownloadButton) && !disabled;
        let layout = input_layout(frame);

        Button::default()
            .set_border_title("Action".into())
            .set_text(" Download ".into())
            .render(frame, &layout[1], focused);
    }
}
