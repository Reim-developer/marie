use crate::core::context::AppContext;
use crate::focus::Focus;
use crate::keyboard::KeyboardAction;
use crate::log_entry::LogEntry;
use crate::ui::registry::UiRegistry;
use crate::ui::shared::UiLayout;
use crossterm::event::KeyCode;
use ratatui::Frame;

#[derive(Default, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Idle,
    Busy,
}

#[derive(Default)]
pub struct App {
    pub ctx: AppContext,
    pub ui: UiRegistry,
}

impl App {
    #[must_use]
    pub const fn ctx(&self) -> &AppContext {
        &self.ctx
    }

    #[must_use]
    pub const fn ui(&self) -> &UiRegistry {
        &self.ui
    }

    #[must_use]
    pub const fn ui_mut(&mut self) -> &mut UiRegistry {
        &mut self.ui
    }

    #[must_use]
    pub const fn split_mut(&mut self) -> (&mut AppContext, &mut UiRegistry) {
        (&mut self.ctx, &mut self.ui)
    }

    pub fn push_log(&self, entry: impl Into<LogEntry>) {
        self.ctx.push_log(entry);
    }

    #[must_use]
    pub fn is_busy(&self) -> bool {
        self.ctx.is_busy()
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let visible = self.ctx.command_palette_visible();
        let layout = UiLayout::new(frame, visible);

        self.ui.render(frame, &layout, &self.ctx);
    }

    pub fn handle_key(&mut self, key: KeyCode) -> KeyboardAction {
        if let KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down =
            key
        {
            self.ctx.focus().handle(key);
            return KeyboardAction::None;
        }

        if let Some(action) = self.ui.handle_key(key, &self.ctx) {
            return action;
        }

        if key == KeyCode::Esc && self.ctx.focus() != Focus::UrlInput {
            return KeyboardAction::Exit;
        }

        KeyboardAction::None
    }
}
