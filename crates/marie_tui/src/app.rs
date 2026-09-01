use std::collections::VecDeque;

use crate::download_scope::DownloadScope;
use crate::focus::Focus;
use crate::log_entry::LogEntry;
use crate::ui::download_button::DownloadButton;
use crate::ui::features_table::FeaturesTable;
use crate::ui::log_panel::LogPanel;
use crate::ui::simple_help::SimpleHelp;
use crate::ui::url_input::UrlInput;
use ratatui::Frame;

const MAX_LOG_LINES: usize = 1_000;

#[derive(Default, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Idle,
    Busy,
}

#[derive(Default)]
pub struct App {
    pub url_value: String,
    pub log_entries: VecDeque<LogEntry>,
    pub log_scroll: usize,
    pub log_hscroll: usize,
    pub focus: Focus,
    pub features_selected: DownloadScope,
    pub state: AppState,
}

impl App {
    pub fn push_log(&mut self, entry: impl Into<LogEntry>) {
        if self.log_entries.len() >= MAX_LOG_LINES {
            self.log_entries.pop_front();
            self.log_scroll = self.log_scroll.saturating_sub(1);
        }

        self.log_entries.push_back(entry.into());
    }

    #[must_use]
    pub fn is_busy(&self) -> bool {
        self.state == AppState::Busy
    }

    pub fn render(&mut self, frame: &mut Frame) {
        UrlInput::render(frame, &self.focus, &self.url_value);
        DownloadButton::render(frame, &self.focus, self.is_busy());
        FeaturesTable::render(frame, &self.focus, self.features_selected);
        LogPanel::render(
            frame,
            &self.focus,
            &mut self.log_entries,
            &mut self.log_scroll,
            &mut self.log_hscroll,
        );
        SimpleHelp::render(frame);
    }
}
