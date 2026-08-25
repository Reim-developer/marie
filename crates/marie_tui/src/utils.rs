use std::any::Any;

use ratatui::style::{Color, Style};

pub const fn not_used(var: &dyn Any) {
    _ = var;
}

#[must_use]
pub fn focused_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::LightCyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}
