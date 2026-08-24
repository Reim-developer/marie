#![deny(clippy::pedantic, clippy::all, clippy::nursery, clippy::perf)]

pub mod app;
pub mod boostrap;
pub mod focus;
pub mod components {
    pub mod download_button;
    pub mod simple_help;
    pub mod url_input;
}
