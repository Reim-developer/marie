use ratatui::Frame;

use crate::{
    components::input_textbox::InputTextbox, focus::Focus, ui::shared::UiLayout,
};

pub struct CommandPalette;

impl CommandPalette {
    pub fn render(
        frame: &mut Frame,
        focus: &Focus,
        value: &str,
        ui_layout: &UiLayout,
    ) {
        if let Some(area) = ui_layout.command_palette {
            let focused = *focus == Focus::CommandPalette;

            InputTextbox::default()
                .set_title("Command Palette: ESC To Exit")
                .render(frame, &area, focused, value);
        }
    }
}
