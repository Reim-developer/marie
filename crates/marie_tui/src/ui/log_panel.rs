use std::collections::VecDeque;

use ratatui::Frame;

use crate::{
    components::text_panel::TextPanel, focus::Focus, log_entry::LogEntry,
    ui::shared::content_layout,
};

#[derive(Default)]
pub struct LogPanel;

impl LogPanel {
    pub fn render(
        frame: &mut Frame,
        focus: &Focus,
        log_entries: &mut VecDeque<LogEntry>,
        scroll: &mut usize,
        hscroll: &mut usize,
    ) {
        let focused = matches!(focus, Focus::LogPanel);
        let content_layout = content_layout(frame);
        let logs_ref = log_entries.make_contiguous();

        let (v, h) =
            TextPanel::new("Download Log", logs_ref, *scroll, *hscroll)
                .hint("k, j, h, l to navigate")
                .render(frame, content_layout[1], focused);

        *scroll = v;
        *hscroll = h;
    }
}
