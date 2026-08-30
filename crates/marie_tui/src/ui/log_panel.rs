use std::collections::VecDeque;

use ratatui::Frame;

use crate::{
    components::text_panel::TextPanel, focus::Focus, ui::shared::content_layout,
};

#[derive(Default)]
pub struct LogPanel;

impl LogPanel {
    pub fn render(
        frame: &mut Frame,
        focus: &Focus,
        log_msg: &VecDeque<String>,
        scroll: &mut usize,
    ) {
        let focused = matches!(focus, Focus::LogPanel);
        let content_layout = content_layout(frame);

        let (front, back) = log_msg.as_slices();
        let logs_ref: Vec<String> =
            front.iter().chain(back.iter()).cloned().collect();

        *scroll = TextPanel::new("Download Log", &logs_ref, *scroll)
            .hint("k, j to navigate")
            .render(frame, content_layout[1], focused);
    }
}
