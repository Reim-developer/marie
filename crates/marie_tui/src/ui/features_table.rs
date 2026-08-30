use ratatui::Frame;

use crate::{
    components::table_list::TableList, focus::Focus, ui::shared::app_layout,
};

pub struct FeaturesTable;

impl FeaturesTable {
    pub fn render(frame: &mut Frame, focus: &Focus) {
        let focused = matches!(focus, Focus::FeaturesTable);

        let app_layout = app_layout(frame);
        let mut table_list = TableList::default();

        table_list
            .title("Download Options".into())
            .rows(vec![
                vec![" 1. ALL Image".into()],
                vec![" 2. ALL Image in URL(s)".into()],
            ])
            .render(frame, app_layout[0], focused);
    }
}
