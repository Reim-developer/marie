use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
};

type Rows = Vec<Vec<Line<'static>>>;
#[derive(Default)]
pub struct TableList {
    title: String,
    rows: Rows,
    border_color: Option<Color>,
    description: Option<Line<'static>>,
}

impl TableList {
    pub fn rows(&mut self, rows: Rows) -> &mut Self {
        self.rows = rows;

        self
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;

        self
    }

    pub const fn border_color(&mut self, color: Color) -> &mut Self {
        self.border_color = Some(color);

        self
    }

    pub fn description(&mut self, desc: impl Into<Line<'static>>) -> &mut Self {
        self.description = Some(desc.into());

        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let (table_area, desc_area) = if self.description.is_some() {
            let chunks =
                Layout::vertical([Constraint::Min(3), Constraint::Length(1)])
                    .split(area);
            (chunks[0], Some(chunks[1]))
        } else {
            (area, None)
        };

        let rows: Vec<Row> = self
            .rows
            .iter()
            .map(|line| Row::new(line.clone()))
            .collect();

        let column_count = self.rows.first().map_or(1, |r| r.len().max(1));
        let widths: Vec<Constraint> =
            (0..column_count).map(|_| Constraint::Min(1)).collect();

        let border_color = self.border_color.unwrap_or(if focused {
            Color::Cyan
        } else {
            Color::White
        });

        let table = Table::new(rows, &widths)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
                    .title(self.title.clone()),
            )
            .column_spacing(1);

        frame.render_widget(table, table_area);

        if let (Some(desc), Some(area)) = (self.description.clone(), desc_area)
        {
            let paragraph = Paragraph::new(desc)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });

            frame.render_widget(paragraph, area);
        }
    }
}
