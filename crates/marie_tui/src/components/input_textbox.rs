use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

#[derive(Default)]
pub struct InputTextbox {
    pub value: String,
}

type SpanV<'a> = [Span<'a>];
const MARKER: &str = ">  ";

impl InputTextbox {
    fn render_cursor(&self, area: Rect, frame: &mut Frame) {
        let chars_count = self.value.chars().count();
        let text_length = u16::try_from(chars_count + 4).unwrap_or(u16::MAX);
        let cursor_x = area.x + text_length;
        let cursor_y = area.y + 1;

        frame.set_cursor_position(Position::new(cursor_x, cursor_y));
    }

    fn style(focused: bool) -> Style {
        if focused {
            Style::default()
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }

    fn render_input(&self, frame: &mut Frame, area: Rect, focused: bool) {
        type P<'a> = Paragraph<'a>;
        type S<'a> = Span<'a>;
        type L<'a> = Line<'a>;
        type B<'a> = Block<'a>;

        let raw_span: &SpanV = &[S::raw(MARKER), S::raw(self.value.as_str())];
        let input_border = B::bordered().title("URL");
        let input_line = L::from(raw_span);
        let input = P::new(input_line).block(
            input_border
                .border_style(Self::style(focused))
                .style(Self::style(focused)),
        );

        frame.render_widget(input, area);
    }

    /// # Errors
    /// Handle event or rendering TUI component failed.
    pub fn render(
        &self,
        frame: &mut Frame,
        area: &Rect,
        focused: bool,
    ) -> Result<(), anyhow::Error> {
        self.render_input(frame, *area, focused);
        if focused {
            self.render_cursor(*area, frame);
        }

        Ok(())
    }
}
