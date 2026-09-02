use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{
    components::table_list::TableList, download_scope::DownloadScope,
    focus::Focus, ui::shared::UiLayout,
};

pub struct FeaturesTable;

impl FeaturesTable {
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
            vec![Self::line(
                DownloadScope::PageImages,
                "[1] Page Images",
                selected,
            )],
            vec![Self::line(
                DownloadScope::SiteImages,
                "[2] Site Images",
                selected,
            )],
        ]
    }

    pub fn render(
        frame: &mut Frame,
        focus: &Focus,
        selected: DownloadScope,
        ui_layout: &UiLayout,
    ) {
        let focused = matches!(focus, Focus::FeaturesTable);

        let app_layout = ui_layout.content_left;
        let mut table_list = TableList::default();

        table_list
            .title("Download Options".into())
            .rows(Self::rows(selected))
            .border_color(if focused { Color::Cyan } else { Color::White })
            .description(Self::description(selected))
            .render(frame, app_layout, focused);
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
}
