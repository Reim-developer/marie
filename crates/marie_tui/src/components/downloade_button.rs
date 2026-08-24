use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

#[derive(Default)]
pub struct DownloadButton;

impl DownloadButton {
    /// # Errors
    /// Render `DownloadButton` component failed.
    pub fn render(
        &self,
        frame: &mut Frame,
        area: &Rect,
    ) -> Result<(), anyhow::Error> {
        type P<'a> = Paragraph<'a>;
        type B<'a> = Block<'a>;

        let block = B::bordered().title("Action");
        let button = P::new(" Download ").block(block);

        frame.render_widget(button, *area);
        Ok(())
    }
}
