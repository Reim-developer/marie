use std::rc::Rc;

use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::{Frame, layout::Layout};

use crate::components::download_button::DownloadButton;
use crate::components::simple_help::render_simple_help;
use crate::components::url_input::UrlInput;
use crate::focus::Focus;

#[derive(Default)]
pub struct App {
    pub url_input: UrlInput,
    pub download_button: DownloadButton,
    pub focus: Focus,
}

type C = Constraint;
impl App {
    const CONSTRAINTS: [C; 3] = [C::Min(1), C::Length(3), C::Length(1)];
    const INPUT_CONSTRAINTS: [C; 2] = [C::Min(1), C::Length(12)];

    pub fn keyboard_handle(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Left | KeyCode::Right => {
                self.focus.handle(key);
            }

            KeyCode::Esc => return true,
            _ => match self.focus {
                Focus::UrlInput => {
                    if self.url_input.input_handle(key) {
                        return true;
                    }
                }

                Focus::DownloadButton => {
                    if key == KeyCode::Enter { /* Implement Soon. */ }
                }
            },
        }

        false
    }

    fn input_layout(layout: &Rc<[Rect]>) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Self::INPUT_CONSTRAINTS)
            .split(layout[1])
    }

    fn render_input_url(
        &self,
        layout: &Rc<[Rect]>,
        frame: &mut Frame,
    ) -> Result<(), anyhow::Error> {
        self.url_input.render(
            frame,
            &layout[0],
            matches!(self.focus, Focus::UrlInput),
        )?;

        Ok(())
    }

    fn render_download_button(
        &self,
        frame: &mut Frame,
        layout: &Rc<[Rect]>,
    ) -> Result<(), anyhow::Error> {
        let focused = matches!(self.focus, Focus::DownloadButton);
        self.download_button.render(frame, &layout[1], focused)?;

        Ok(())
    }

    /// # Errors
    /// Render TUI application failed.
    pub fn render(&self, frame: &mut Frame) -> Result<(), anyhow::Error> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(Self::CONSTRAINTS)
            .split(frame.area());

        let input_layout = Self::input_layout(&layout);
        self.render_input_url(&input_layout, frame)?;
        self.render_download_button(frame, &input_layout)?;

        render_simple_help(frame, &layout[2]);
        Ok(())
    }
}
