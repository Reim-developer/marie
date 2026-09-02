use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::{
    components::button::Button,
    core::context::AppContext,
    focus::Focus,
    keyboard::KeyboardAction,
    log_entry::LogEntry,
    ui::{component::Component, shared::UiLayout},
};

pub struct DownloadButton;

impl Component for DownloadButton {
    fn render(
        &mut self,
        frame: &mut Frame,
        layout: &UiLayout,
        ctx: &AppContext,
    ) {
        let is_busy = ctx.is_busy();
        let focus = ctx.focus();
        let focused = matches!(focus, Focus::DownloadButton) && !is_busy;

        Button::default()
            .set_border_title("Action".into())
            .set_text(" Download ".into())
            .render(frame, &layout.download_button, focused);
    }

    fn handle_key(
        &mut self,
        key: crossterm::event::KeyCode,
        ctx: &AppContext,
    ) -> Option<crate::keyboard::KeyboardAction> {
        if ctx.focus() != Focus::DownloadButton {
            return None;
        }

        match key {
            KeyCode::Enter => {
                if ctx.is_busy() {
                    ctx.push_log(LogEntry::Error(
                        "Download in progress".into(),
                    ));

                    return Some(KeyboardAction::None);
                }

                Some(KeyboardAction::Download)
            }
            _ => Some(KeyboardAction::None),
        }
    }
}
