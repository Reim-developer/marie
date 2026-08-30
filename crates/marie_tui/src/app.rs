use crate::focus::Focus;
use crate::ui::download_button::DownloadButton;
use crate::ui::features_table::FeaturesTable;
use crate::ui::simple_help::SimpleHelp;
use crate::ui::url_input::UrlInput;
use ratatui::Frame;

#[derive(Default)]
pub struct App {
    pub url_value: String,
    pub focus: Focus,
    pub features_selected: Option<usize>,
}

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        UrlInput::render(frame, &self.focus, &self.url_value);
        DownloadButton::render(frame, &self.focus);
        FeaturesTable::render(frame, &self.focus, self.features_selected);
        SimpleHelp::render(frame);
    }
}
