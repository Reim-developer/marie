use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

#[derive(Default)]
pub struct TextPanel {
    pub title: String,
    pub title_bottom: String,
    pub logs: Vec<String>,
    pub scroll: usize,
}

impl TextPanel {
    pub fn add(&mut self, msg: impl Into<String>) {
        self.logs.push(msg.into());
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;

        self
    }

    pub fn title_bottom(&mut self, title: String) -> &mut Self {
        self.title_bottom = title;

        self
    }

    pub const fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub const fn scroll_down(&mut self) {
        self.scroll += 1;
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if focused {
                Color::Cyan
            } else {
                Color::White
            }))
            .title(self.title.clone())
            .title_bottom(self.title_bottom.clone())
            .style(Style::default().fg(Color::Gray));

        let inner = block.inner(area);
        let visible = inner.height as usize;
        let max_scroll = self.logs.len().saturating_sub(visible);
        self.scroll = self.scroll.min(max_scroll);

        let content = if self.logs.is_empty() {
            "   Nothing to show.".to_string()
        } else {
            self.logs
                .iter()
                .skip(self.scroll)
                .take(visible.max(1))
                .map(|s| format!("   {s}"))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let paragraph = Paragraph::new(content)
            .block(block)
            .style(Style::default().fg(Color::Gray));

        frame.render_widget(paragraph, area);
    }
}
