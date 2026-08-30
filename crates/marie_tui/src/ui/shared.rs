use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

type C = Constraint;

pub const APP_CONSTRAINTS: [C; 3] = [C::Min(1), C::Length(3), C::Length(1)];
pub const CONTENT_CONSTRAINTS: [C; 2] = [C::Percentage(50), C::Percentage(50)];
pub const INPUT_CONSTRAINTS: [C; 2] = [C::Min(1), C::Length(12)];

#[must_use]
pub fn app_layout(frame: &Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(APP_CONSTRAINTS)
        .split(frame.area())
}

#[must_use]
pub fn content_layout(frame: &Frame) -> Rc<[Rect]> {
    let app_layout = app_layout(frame);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(CONTENT_CONSTRAINTS)
        .split(app_layout[0])
}

#[must_use]
pub fn input_layout(frame: &Frame) -> Rc<[Rect]> {
    let app_layout = app_layout(frame);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(INPUT_CONSTRAINTS)
        .split(app_layout[1])
}
