use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

type C = Constraint;

pub struct UiLayout {
    pub content_left: Rect,  /* Features Table */
    pub content_right: Rect, /* Log Panel */
    pub url_input: Rect,
    pub download_button: Rect,
    pub command_palette: Option<Rect>,
    pub simple_help: Rect,
}

pub const CONTENT_CONSTRAINTS: [C; 2] = [C::Percentage(50), C::Percentage(50)];
pub const INPUT_CONSTRAINTS: [C; 2] = [C::Min(1), C::Length(12)];

impl UiLayout {
    pub fn new(frame: &Frame, command_palette_visible: bool) -> Self {
        let vertical_constraints = if command_palette_visible {
            vec![C::Min(1), C::Length(3), C::Length(3), C::Length(1)]
        } else {
            vec![C::Min(1), C::Length(3), C::Length(1)]
        };

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vertical_constraints)
            .split(frame.area());

        let content_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(CONTENT_CONSTRAINTS)
            .split(vertical[0]);

        let input_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(INPUT_CONSTRAINTS)
            .split(vertical[1]);
        let command_palette = if command_palette_visible {
            Some(vertical[2])
        } else {
            None
        };

        let help_index = if command_palette_visible { 3 } else { 2 };
        let simple_help = vertical[help_index];

        Self {
            content_left: content_row[0],
            content_right: content_row[1],
            url_input: input_row[0],
            download_button: input_row[1],
            command_palette,
            simple_help,
        }
    }
}
