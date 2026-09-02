use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{
    components::table_list::TableList,
    core::context::AppContext,
    download_scope::DownloadScope,
    focus::Focus,
    keyboard::KeyboardAction,
    ui::{component::Component, shared::UiLayout},
};

pub struct FeaturesTable;

fn description(selected: DownloadScope) -> Line<'static> {
    match selected {
        DownloadScope::PageImages => Line::from(Span::styled(
            "Download all images from current page only.",
            Style::default().fg(Color::Gray),
        )),
        DownloadScope::SiteImages => Line::from(Span::styled(
            "Follow all internal links and download image across the entire site.",
            Style::default().fg(Color::Gray),
        )),
    }
}

fn rows(selected: DownloadScope) -> Vec<Vec<Line<'static>>> {
    vec![
        vec![line(DownloadScope::PageImages, "[1] Page Images", selected)],
        vec![line(DownloadScope::SiteImages, "[2] Site Images", selected)],
    ]
}

fn line(
    scope: DownloadScope,
    text: &str,
    selected: DownloadScope,
) -> Line<'static> {
    let is_selected = selected == scope;
    let marker = if is_selected { "> " } else { "  " };
    let content = format!("{marker}{text}");

    if is_selected {
        Line::from(Span::styled(
            content,
            Style::default().fg(Color::Rgb(255, 165, 0)),
        ))
    } else {
        Line::from(content)
    }
}

impl Component for FeaturesTable {
    fn render(
        &mut self,
        frame: &mut Frame,
        ui_layout: &UiLayout,
        ctx: &AppContext,
    ) {
        let focused = matches!(ctx.focus(), Focus::FeaturesTable);

        let app_layout = ui_layout.content_left;
        let mut table_list = TableList::default();
        let selected = ctx.features_selected();

        table_list
            .title("Download Options".into())
            .rows(rows(selected))
            .border_color(if focused { Color::Cyan } else { Color::White })
            .description(description(selected))
            .render(frame, app_layout, focused);
    }

    fn handle_key(
        &mut self,
        key: crossterm::event::KeyCode,
        ctx: &AppContext,
    ) -> Option<crate::keyboard::KeyboardAction> {
        if ctx.focus() != Focus::FeaturesTable {
            return None;
        }

        match key {
            KeyCode::Char('1') => {
                ctx.set_features_selected(DownloadScope::PageImages);

                Some(KeyboardAction::None)
            }
            KeyCode::Char('2') => {
                ctx.set_features_selected(DownloadScope::SiteImages);

                Some(KeyboardAction::None)
            }
            _ => Some(KeyboardAction::None),
        }
    }
}
