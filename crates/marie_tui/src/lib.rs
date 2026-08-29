#![deny(clippy::pedantic, clippy::all, clippy::nursery, clippy::perf)]

pub mod app;
pub mod boostrap;
pub mod focus;
pub mod keyboard;
pub mod utils;

pub mod components {
    pub mod button;
    pub mod input_textbox;
    pub mod simple_help;
    pub mod table_list;
}

pub(crate) mod ui {
    pub mod download_button;
    pub mod shared;
}

pub mod core {
    pub mod app;
    pub mod sender;
    pub mod signal;
}
