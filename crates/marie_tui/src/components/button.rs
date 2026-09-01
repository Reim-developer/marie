use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::utils::focused_style;

#[derive(Default)]
pub struct Button {
    border_title: String,
    text: String,
    disabled: bool,
}

impl Button {
    pub fn set_text(&mut self, title: String) -> &mut Self {
        self.text = title;

        self
    }

    pub fn set_border_title(&mut self, border_title: String) -> &mut Self {
        self.border_title = border_title;

        self
    }

    pub const fn set_disabled(&mut self, disabled: bool) -> &mut Self {
        self.disabled = disabled;

        self
    }

    fn render_button_style(&self, frame: &mut Frame, area: Rect, style: Style) {
        type P<'a> = Paragraph<'a>;
        type B<'a> = Block<'a>;

        let block = B::bordered().title(self.border_title.clone());
        let button = P::new(self.text.clone()).block(block).style(style);

        frame.render_widget(button, area);
    }

    /// # Errors
    /// Render `DownloadButton` component failed.
    pub fn render(&mut self, frame: &mut Frame, area: &Rect, focused: bool) {
        let style = if self.disabled {
            Style::default().fg(Color::DarkGray)
        } else {
            focused_style(focused)
        };

        self.render_button_style(frame, *area, style);
    }
}
