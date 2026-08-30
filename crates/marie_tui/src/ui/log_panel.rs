use ratatui::Frame;

use crate::{
    components::text_panel::TextPanel, focus::Focus, ui::shared::content_layout,
};

#[derive(Default)]
pub struct LogPanel;

impl LogPanel {
    pub fn render(frame: &mut Frame, focus: &Focus) {
        let focused = matches!(focus, Focus::LogPanel);
        let content_layout = content_layout(frame);

        let mut text_panel = TextPanel::default();
        text_panel
            .title_bottom("k, j to navigate".into())
            .title("Download Log".into())
            .render(frame, content_layout[1], focused);
    }
}
