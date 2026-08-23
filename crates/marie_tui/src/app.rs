use ratatui::layout::{Constraint, Direction};
use ratatui::{Frame, layout::Layout};

use crate::components::url_input::UrlInput;

#[derive(Default)]
pub struct App {
    pub url_input: UrlInput,
}

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(frame.area());

        self.url_input.render(frame, layout[1]);
    }
}
