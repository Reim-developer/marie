use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Cell, Row, Table},
};

type Rows = Vec<Vec<Line<'static>>>;
#[derive(Default)]
pub struct TableList {
    title: String,
    rows: Rows,
    border_color: Option<Color>,
    description: Option<Line<'static>>,
    cell_aligment: Alignment,
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

    pub const fn cell_aligment(&mut self, aligment: Alignment) -> &mut Self {
        self.cell_aligment = aligment;

        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let rows: Vec<Row> = self
            .rows
            .iter()
            .map(|cells| {
                let cells: Vec<Cell> = cells
                    .iter()
                    .map(|line| {
                        let mut aligned = line.clone();
                        aligned.alignment = Some(self.cell_aligment);

                        Cell::from(aligned)
                    })
                    .collect();

                Row::new(cells)
            })
            .collect();

        let column_count = self.rows.first().map_or(1, |r| r.len().max(1));
        let widths: Vec<Constraint> =
            (0..column_count).map(|_| Constraint::Min(1)).collect();

        let border_color = self.border_color.unwrap_or(if focused {
            Color::Cyan
        } else {
            Color::White
        });

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(self.title.clone());

        if let Some(desc) = &self.description {
            block = block.title_bottom(desc.clone());
        }

        let table = Table::new(rows, &widths).block(block).column_spacing(1);
        frame.render_widget(table, area);
    }
}
