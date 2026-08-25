use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::{Frame, layout::Layout};
use std::rc::Rc;

use crate::components::button::{Button, ButtonProperties};
use crate::components::input_textbox::InputTextbox;
use crate::components::simple_help::render_simple_help;
use crate::components::table_list::TableList;
use crate::focus::Focus;

#[derive(Default)]
pub struct App {
    pub url_input: InputTextbox,
    pub download_button: Button,
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
        type BP = ButtonProperties;
        type S = String;
        let focused = matches!(self.focus, Focus::DownloadButton);

        let properties = BP::new(S::from("Action"), S::from(" Download "));
        self.download_button
            .render(frame, &layout[1], focused, properties)?;

        Ok(())
    }

    /// # Errors
    /// Render TUI application failed.
    pub fn render(&mut self, frame: &mut Frame) -> Result<(), anyhow::Error> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(Self::CONSTRAINTS)
            .split(frame.area());

        self.features_order
            .title("Download Options".into())
            .rows(vec![
                vec!["ALL Image".into()],
                vec!["ALL Image in URL(s)".into()],
            ])
            .render(frame, layout[0]);

        let input_layout = Self::input_layout(&layout);
        self.render_input_url(&input_layout, frame)?;
        self.render_download_button(frame, &input_layout)?;
        render_simple_help(frame, &layout[2]);
        Ok(())
    }
}
