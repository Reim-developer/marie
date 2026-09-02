use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::{
    core::context::AppContext,
    keyboard::KeyboardAction,
    ui::{
        command_palette::CommandPalette, component::Component,
        download_button::DownloadButton, features_table::FeaturesTable,
        log_panel::LogPanel, shared::UiLayout, simple_help::SimpleHelp,
        url_input::UrlInput,
    },
};

pub struct UiRegistry {
    components: Vec<Box<dyn Component>>,
}

impl Default for UiRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl UiRegistry {
    pub fn new() -> Self {
        let components: Vec<Box<dyn Component>> = vec![
            Box::new(UrlInput),
            Box::new(DownloadButton),
            Box::new(FeaturesTable),
            Box::new(LogPanel),
            Box::new(CommandPalette),
            Box::new(SimpleHelp),
        ];

        Self { components }
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        layout: &UiLayout,
        ctx: &AppContext,
    ) {
        for component in &mut self.components {
            component.render(frame, layout, ctx);
        }
    }

    pub fn handle_key(
        &mut self,
        key: KeyCode,
        ctx: &AppContext,
    ) -> Option<KeyboardAction> {
        for component in &mut self.components {
            if let Some(action) = component.handle_key(key, ctx) {
                return Some(action);
            }
        }

        None
    }
}
