use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crossterm::event::KeyCode;

use crate::{download_scope::DownloadScope, focus::Focus, log_entry::LogEntry};

type ArcMutex<T> = Arc<Mutex<T>>;

#[derive(Clone, Default)]
pub struct AppContext {
    focus: ArcMutex<Focus>,
    url_value: ArcMutex<String>,
    command_value: ArcMutex<String>,
    command_palette_visible: ArcMutex<bool>,
    log_entries: ArcMutex<VecDeque<LogEntry>>,
    log_scroll: ArcMutex<usize>,
    log_hscroll: ArcMutex<usize>,
    features_selected: ArcMutex<DownloadScope>,
    is_busy: ArcMutex<bool>,
}

impl AppContext {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn features_selected(&self) -> DownloadScope {
        *self
            .features_selected
            .lock()
            .expect("features_selected mutex poisoned")
    }

    pub fn set_features_selected(&self, d: DownloadScope) {
        *self
            .features_selected
            .lock()
            .expect("features_selected mutex poisoned") = d;
    }

    #[must_use]
    pub fn log_entries(&self) -> VecDeque<LogEntry> {
        self.log_entries
            .lock()
            .expect("log_entries mutex poisoned")
            .clone()
    }

    #[must_use]
    pub fn hscroll(&self) -> usize {
        *self.log_hscroll.lock().expect("log_hscroll mutex poisoned")
    }

    #[must_use]
    pub fn vscroll(&self) -> usize {
        *self.log_scroll.lock().expect("log_scroll mutex poisoned")
    }

    pub fn set_hscroll(&self, h: usize) {
        *self.log_hscroll.lock().expect("log_hscroll mutex poisoned") = h;
    }

    pub fn set_vscroll(&self, v: usize) {
        *self.log_scroll.lock().expect("log_scroll mutex poisoned") = v;
    }

    #[must_use]
    pub fn focus(&self) -> Focus {
        *self.focus.lock().expect("focus mutex poisoned")
    }

    pub fn set_focus(&self, f: Focus) {
        *self.focus.lock().expect("focus mutex poisoned") = f;
    }

    pub fn handle_focus(&self, key: KeyCode) {
        let mut focus = self.focus.lock().expect("focus mutex poisoned");
        let command_palette_visible = self.command_palette_visible();

        match key {
            KeyCode::Left => focus.left(),
            KeyCode::Right => focus.right(),
            KeyCode::Up => focus.up(),
            KeyCode::Down => focus.down(),
            _ => {}
        }

        if *focus == Focus::CommandPalette && !command_palette_visible {
            *focus = Focus::FeaturesTable;
        }
    }

    #[must_use]
    pub fn url_value(&self) -> String {
        self.url_value
            .lock()
            .expect("url_value mutex poisoned")
            .clone()
    }

    pub fn set_url_value(&self, v: &str) {
        *self.url_value.lock().expect("url_value mutex poisoned") =
            v.to_string();
    }

    #[must_use]
    pub fn command_value(&self) -> String {
        self.command_value
            .lock()
            .expect("command_value mutex poisoned")
            .clone()
    }

    pub fn add_command_value(&self, c: char) {
        self.command_value
            .lock()
            .expect("command_value mutex poisoned")
            .push(c);
    }

    pub fn remove_command_value(&self) {
        self.command_value
            .lock()
            .expect("command_value mutex poisoned")
            .pop();
    }

    pub fn set_command_palette_visible(&self, v: bool) {
        *self
            .command_palette_visible
            .lock()
            .expect("command_palette_visible mutex poisoned") = v;
    }

    #[must_use]
    pub fn command_palette_visible(&self) -> bool {
        *self
            .command_palette_visible
            .lock()
            .expect("command_palette_visible mutex poisoned")
    }

    pub fn push_log(&self, entry: impl Into<LogEntry>) {
        const MAX_LOG_LINES: usize = 1_000;

        let mut entries =
            self.log_entries.lock().expect("log_entries mutex poisoned");

        if entries.len() >= MAX_LOG_LINES {
            entries.pop_front();

            let mut scroll =
                self.log_scroll.lock().expect("log_scroll mutex poisoned");
            *scroll = scroll.saturating_sub(1);
        }

        entries.push_back(entry.into());
    }

    #[must_use]
    pub fn is_busy(&self) -> bool {
        *self.is_busy.lock().expect("is_busy mutex poisoned")
    }

    pub fn set_busy(&self, busy: bool) {
        *self.is_busy.lock().expect("is_busy mutex poisoned") = busy;
    }
}
