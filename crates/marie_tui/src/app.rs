use std::rc::Rc;

use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::{Frame, layout::Layout};

use crate::components::downloade_button::DownloadButton;
use crate::components::simple_help::render_simple_help;
use crate::components::url_input::UrlInput;

#[derive(Default)]
pub struct App {
    pub url_input: UrlInput,
    pub download_button: DownloadButton,
}

type C = Constraint;
impl App {
    const CONSTRAINTS: [C; 3] = [C::Min(1), C::Length(3), C::Length(1)];
    const INPUT_CONSTRAINTS: [C; 2] = [C::Min(1), C::Length(12)];

    fn url_input_layout(layout: &Rc<[Rect]>) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Self::INPUT_CONSTRAINTS)
            .split(layout[1])
    }

    /// # Errors
    /// Render TUI application failed.
    pub fn render(&self, frame: &mut Frame) -> Result<(), anyhow::Error> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(Self::CONSTRAINTS)
            .split(frame.area());

        let input_layout = Self::url_input_layout(&layout);
        self.url_input.render(frame, &input_layout[0])?;
        self.download_button.render(frame, &input_layout[1])?;

        render_simple_help(frame, &layout[2]);
        Ok(())
    }
}
