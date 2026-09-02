use ratatui::Frame;

use crate::{components::button::Button, focus::Focus, ui::shared::UiLayout};

pub struct DownloadButton;

impl DownloadButton {
    pub fn render(
        frame: &mut Frame,
        focus: &Focus,
        disabled: bool,
        ui_layout: &UiLayout,
    ) {
        let focused = matches!(focus, Focus::DownloadButton) && !disabled;
        let layout = ui_layout.download_button;

        Button::default()
            .set_border_title("Action".into())
            .set_text(" Download ".into())
            .render(frame, &layout, focused);
    }
}
