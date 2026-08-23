use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Default)]
pub struct UrlInput {
    pub url: String,
}

impl UrlInput {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.url.as_str())
            .block(Block::bordered().title("URL").borders(Borders::ALL));

        frame.render_widget(input, area);
    }
}
