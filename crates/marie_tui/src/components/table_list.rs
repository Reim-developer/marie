use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    widgets::{Block, Borders, Row, Table},
};

#[derive(Default)]
pub struct TableList {
    title: String,
    rows: Vec<Vec<String>>,
}

impl TableList {
    pub fn rows(&mut self, rows: Vec<Vec<String>>) -> &mut Self {
        self.rows = rows;

        self
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;

        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let rows: Vec<Row> = self
            .rows
            .iter()
            .map(|cells| Row::new(cells.clone()))
            .collect();

        let column_count = self.rows.first().map_or(1, |r| r.len().max(1));
        let widths: Vec<Constraint> =
            (0..column_count).map(|_| Constraint::Min(1)).collect();

        let table = Table::new(rows, &widths)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.title.clone()),
            )
            .column_spacing(1);

        frame.render_widget(table, area);
    }
}
