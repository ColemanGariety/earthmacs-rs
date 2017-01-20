use std::cmp::{min, max};
use ncurses::*;
use buffer::Buffer;
use mode::Mode;

pub struct Pane {
    pub buffer_index: usize,
    pub mode: Mode,
    pub y: i32,
}

impl Pane {
    pub fn new(buffer_index: usize) -> Pane {
        Pane {
            buffer_index: buffer_index,
            mode: Mode::new(),
            y: 0,
        }
    }

    pub fn scroll_down(&mut self) {
        self.y += 1;
    }

    pub fn scroll_up(&mut self) {
        self.y -= 1;
    }

    pub fn scroll_by(&mut self, y: i32) {
        self.y = max(0, self.y + y);
    }
}
