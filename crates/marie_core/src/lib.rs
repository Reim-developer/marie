#![deny(clippy::pedantic, clippy::all, clippy::nursery, clippy::perf)]

pub mod net;
pub mod scraper;
pub mod selector;

pub mod downloader {
    pub mod images;
}
