use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

#[derive(Default)]
pub struct DownloadButton;

impl DownloadButton {
    fn style(focused: bool) -> Style {
        if focused {
            Style::default()
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }

    fn render_download_button_style(
        frame: &mut Frame,
        area: Rect,
        style: Style,
    ) {
        type P<'a> = Paragraph<'a>;
        type B<'a> = Block<'a>;

        let block = B::bordered().title("Action");
        let button = P::new(" Download ").block(block).style(style);

        frame.render_widget(button, area);
    }

    /// # Errors
    /// Render `DownloadButton` component failed.
    pub fn render(
        &self,
        frame: &mut Frame,
        area: &Rect,
        focused: bool,
    ) -> Result<(), anyhow::Error> {
        let style = Self::style(focused);
        Self::render_download_button_style(frame, *area, style);
        Ok(())
    }
}
