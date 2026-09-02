use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::{
    core::context::AppContext, keyboard::KeyboardAction, ui::shared::UiLayout,
};

pub trait Component: Send {
    fn render(
        &mut self,
        frame: &mut Frame,
        layout: &UiLayout,
        ctx: &AppContext,
    );

    fn handle_key(
        &mut self,
        key: KeyCode,
        ctx: &AppContext,
    ) -> Option<KeyboardAction>;
}
