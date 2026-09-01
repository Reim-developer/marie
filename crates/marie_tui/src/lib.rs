#![deny(clippy::pedantic, clippy::all, clippy::nursery, clippy::perf)]

pub mod app;
pub mod boostrap;
pub mod download_scope;
pub mod event_loop;
pub mod focus;
pub mod keyboard;
pub mod log_entry;
pub mod tui;
pub mod utils;

pub mod components {
    pub mod button;
    pub mod input_textbox;
    pub mod simple_help;
    pub mod table_list;
    pub mod text_panel;
}

pub(crate) mod ui {
    pub mod download_button;
    pub mod features_table;
    pub mod log_panel;
    pub mod shared;
    pub mod simple_help;
    pub mod url_input;
}

pub(crate) mod core {
    pub mod app;
    pub mod event;
    pub mod sender;
    pub mod signal;
}
