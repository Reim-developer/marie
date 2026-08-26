use std::any::Any;

use ratatui::style::{Color, Style};

pub const fn not_used(var: &dyn Any) {
    _ = var;
}

#[cfg(debug_assertions)]
/// # Panics
/// Open or write file failed (permission denied, etc ...)
pub fn debug_to_file(msg: &str, file_name: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .unwrap();

    writeln!(file, "{msg}").unwrap();
}

#[cfg(debug_assertions)]
/// # Panics
/// Remove file failed.
pub fn clean_debug_file(file_name: &str) {
    use std::fs::remove_file;

    remove_file(file_name).unwrap();
}

#[must_use]
pub fn focused_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::LightCyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}
