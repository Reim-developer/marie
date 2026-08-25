use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

#[derive(Default)]
pub struct Button;

pub struct ButtonProperties {
    border_title: String,
    text: String,
}

impl ButtonProperties {
    #[must_use]
    pub const fn new(border_title: String, text: String) -> Self {
        Self { border_title, text }
    }
}

type BT = ButtonProperties;
impl Button {
    fn style(focused: bool) -> Style {
        if focused {
            Style::default()
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }

    fn render_button_style(
        frame: &mut Frame,
        area: Rect,
        style: Style,
        properties: BT,
    ) {
        type P<'a> = Paragraph<'a>;
        type B<'a> = Block<'a>;

        let block = B::bordered().title(properties.border_title);
        let button = P::new(properties.text).block(block).style(style);

        frame.render_widget(button, area);
    }

    /// # Errors
    /// Render `DownloadButton` component failed.
    pub fn render(
        &self,
        frame: &mut Frame,
        area: &Rect,
        focused: bool,
        properties: BT,
    ) -> Result<(), anyhow::Error> {
        let style = Self::style(focused);
        Self::render_button_style(frame, *area, style, properties);
        Ok(())
    }
}
