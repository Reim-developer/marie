use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::utils::focused_style;

#[derive(Default)]
pub struct InputTextbox {
    title: &'static str,
}

type SpanV<'a> = [Span<'a>];
const MARKER: &str = ">  ";

impl InputTextbox {
    #[must_use]
    pub const fn set_title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;

        self
    }

    fn render_cursor(area: Rect, frame: &mut Frame, value: &str) {
        let chars_count = value.chars().count();
        let text_length = u16::try_from(chars_count + 4).unwrap_or(u16::MAX);
        let cursor_x = area.x + text_length;
        let cursor_y = area.y + 1;

        frame.set_cursor_position(Position::new(cursor_x, cursor_y));
    }

    fn render_input(
        &self,
        frame: &mut Frame,
        area: Rect,
        focused: bool,
        value: &str,
    ) {
        type P<'a> = Paragraph<'a>;
        type S<'a> = Span<'a>;
        type L<'a> = Line<'a>;
        type B<'a> = Block<'a>;

        let raw_span: &SpanV = &[S::raw(MARKER), S::raw(value)];
        let input_border = B::bordered().title(self.title);
        let input_line = L::from(raw_span);
        let input = P::new(input_line).block(
            input_border
                .border_style(focused_style(focused))
                .style(Style::default()),
        );

        frame.render_widget(input, area);
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: &Rect,
        focused: bool,
        value: &str,
    ) {
        self.render_input(frame, *area, focused, value);
        if focused {
            Self::render_cursor(*area, frame, value);
        }
    }
}
