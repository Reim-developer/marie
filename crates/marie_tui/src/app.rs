use std::collections::VecDeque;

use crate::focus::Focus;
use crate::ui::download_button::DownloadButton;
use crate::ui::features_table::FeaturesTable;
use crate::ui::log_panel::LogPanel;
use crate::ui::simple_help::SimpleHelp;
use crate::ui::url_input::UrlInput;
use ratatui::Frame;

const MAX_LOG_LINES: usize = 1_000;

#[derive(Default)]
pub struct App {
    pub url_value: String,
    pub log_msg: VecDeque<String>,
    pub log_scroll: usize,
    pub focus: Focus,
    pub features_selected: Option<usize>,
}

impl App {
    pub fn push_log(&mut self, msg: impl Into<String>) {
        let msg = msg.into();

        for line in msg.lines() {
            if self.log_msg.len() >= MAX_LOG_LINES {
                self.log_msg.pop_front();
                self.log_scroll = self.log_scroll.saturating_sub(1);
            }

            self.log_msg.push_back(line.to_string());
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        UrlInput::render(frame, &self.focus, &self.url_value);
        DownloadButton::render(frame, &self.focus);
        FeaturesTable::render(frame, &self.focus, self.features_selected);
        LogPanel::render(
            frame,
            &self.focus,
            &self.log_msg,
            &mut self.log_scroll,
        );
        SimpleHelp::render(frame);
    }
}
