use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::{
    components::text_panel::TextPanel,
    core::context::AppContext,
    focus::Focus,
    keyboard::KeyboardAction,
    ui::{component::Component, shared::UiLayout},
};

#[derive(Default)]
pub struct LogPanel;

impl Component for LogPanel {
    fn render(
        &mut self,
        frame: &mut Frame,
        ui_layout: &UiLayout,
        ctx: &AppContext,
    ) {
        let focused = matches!(ctx.focus(), Focus::LogPanel);
        let content_layout = ui_layout.content_right;
        let mut log_entries = ctx.log_entries();
        let logs_ref = log_entries.make_contiguous();
        let scroll = ctx.vscroll();
        let hscroll = ctx.hscroll();

        let (v, h) = TextPanel::new("Download Log", logs_ref, scroll, hscroll)
            .hint("k, j, h, l to navigate")
            .render(frame, content_layout, focused);

        ctx.set_vscroll(v);
        ctx.set_hscroll(h);
    }

    fn handle_key(
        &mut self,
        key: crossterm::event::KeyCode,
        ctx: &AppContext,
    ) -> Option<KeyboardAction> {
        if ctx.focus() != Focus::LogPanel {
            return None;
        }

        match key {
            KeyCode::Char('k') => {
                let vscroll = ctx.vscroll().saturating_sub(1);
                ctx.set_vscroll(vscroll);

                Some(KeyboardAction::None)
            }
            KeyCode::Char('j') => {
                let mut vscroll = ctx.vscroll();
                vscroll += 1;

                ctx.set_vscroll(vscroll);

                Some(KeyboardAction::None)
            }
            KeyCode::Char('h') => {
                let hscroll = ctx.hscroll().saturating_sub(1);

                ctx.set_hscroll(hscroll);

                Some(KeyboardAction::None)
            }
            KeyCode::Char('l') => {
                let mut hscroll = ctx.hscroll();
                hscroll += 1;

                ctx.set_hscroll(hscroll);

                Some(KeyboardAction::None)
            }

            _ => Some(KeyboardAction::None),
        }
    }
}
