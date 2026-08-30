use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph},
};

pub struct TextPanel<'a> {
    pub title: &'a str,
    pub logs: &'a [String],
    pub scroll: usize,
    pub hint: Option<&'a str>,
}

impl<'a> TextPanel<'a> {
    #[must_use]
    pub const fn new(
        title: &'a str,
        logs: &'a [String],
        scroll: usize,
    ) -> Self {
        Self {
            title,
            logs,
            scroll,
            hint: None,
        }
    }

    #[must_use]
    pub const fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);

        self
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        focused: bool,
    ) -> usize {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if focused {
                Color::Cyan
            } else {
                Color::White
            }))
            .title(self.title)
            .padding(Padding::left(3))
            .style(Style::default().fg(Color::Gray));

        if focused && let Some(hint) = self.hint {
            block = block.title_bottom(hint);
        }

        let content = if self.logs.is_empty() {
            "No thing to show.".to_string()
        } else {
            self.logs.join("\n")
        };

        let max_scroll = self.logs.len().saturating_sub(1);
        let scroll = self.scroll.min(max_scroll);

        let paragraph = Paragraph::new(content)
            .block(block)
            .style(Style::default().fg(Color::Gray))
            .scroll((u16::try_from(scroll).unwrap_or(0), 0));

        frame.render_widget(paragraph, area);

        scroll
    }
}
