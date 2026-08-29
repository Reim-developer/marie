use std::rc::Rc;

use crate::components::input_textbox::InputTextbox;
use crate::components::simple_help::render_simple_help;
use crate::components::table_list::TableList;
use crate::focus::Focus;
use crate::ui::download_button::DownloadButton;
use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::{Frame, layout::Layout};

#[derive(Default)]
pub struct App {
    pub url_input: InputTextbox,
    pub features_order: TableList,
    pub focus: Focus,
}

type C = Constraint;
impl App {
    const CONSTRAINTS: [C; 3] = [C::Min(1), C::Length(3), C::Length(1)];
    const INPUT_CONSTRAINTS: [C; 2] = [C::Min(1), C::Length(12)];

    fn input_layout(layout: &Rc<[Rect]>) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Self::INPUT_CONSTRAINTS)
            .split(layout[1])
    }

    fn render_input_url(&self, layout: &Rc<[Rect]>, frame: &mut Frame) {
        self.url_input.render(
            frame,
            &layout[0],
            matches!(self.focus, Focus::UrlInput),
        );
    }

    /// # Errors
    /// Render TUI application failed.
    pub fn render(&mut self, frame: &mut Frame) -> Result<(), anyhow::Error> {
        let app_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(Self::CONSTRAINTS)
            .split(frame.area());

        self.features_order
            .title("Download Options".into())
            .rows(vec![
                vec![" 1. ALL Image".into()],
                vec![" 2. ALL Image in URL(s)".into()],
            ])
            .render(frame, app_layout[0]);

        let input_layout = Self::input_layout(&app_layout);
        self.render_input_url(&input_layout, frame);

        DownloadButton::render(frame, &self.focus);

        render_simple_help(frame, &app_layout[2]);
        Ok(())
    }
}
