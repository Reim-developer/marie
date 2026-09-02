use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::Paragraph,
};

const SIMPLE_HELP_PARAG: &str =
    "← → ↑ ↓: Navigate    ENTER: Select    ESC: Exit    : Open Command Palette
";

pub fn render_simple_help(frame: &mut Frame, area: &Rect) {
    let help = Paragraph::new(SIMPLE_HELP_PARAG)
        .style(Style::default().add_modifier(Modifier::DIM));

    frame.render_widget(help, *area);
}
