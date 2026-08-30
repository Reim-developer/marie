use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{
    components::table_list::TableList, focus::Focus, ui::shared::app_layout,
};

pub struct FeaturesTable;

impl FeaturesTable {
    pub fn render(frame: &mut Frame, focus: &Focus, selected: Option<usize>) {
        let focused = matches!(focus, Focus::FeaturesTable);

        let app_layout = app_layout(frame);
        let mut table_list = TableList::default();

        let rows = vec![
            vec![Self::line(0, "[1] Page Images", selected)],
            vec![Self::line(1, "[2] Site Images", selected)],
        ];
        let description = match selected {
            Some(0) => Line::from(Span::styled(
                "Download all images from current page only.",
                Style::default().fg(Color::Gray),
            )),
            Some(1) => Line::from(Span::styled(
                "Follow all internal links and download image across the entire site.",
                Style::default().fg(Color::Gray),
            )),
            _ => Line::from(Span::styled("", Style::default().fg(Color::Gray))),
        };

        table_list
            .title("Download Options".into())
            .rows(rows)
            .border_color(if focused { Color::Cyan } else { Color::White })
            .description(description)
            .render(frame, app_layout[0], focused);
    }

    fn line(idx: usize, text: &str, selected: Option<usize>) -> Line<'static> {
        let is_selected = selected == Some(idx);
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
