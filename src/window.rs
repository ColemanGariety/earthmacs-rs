use std::cmp::{min, max};
use ncurses::*;
use buffer::Buffer;

pub struct Window {
    pub buffer_index: usize,
    pub y: i32,
}

impl Window {
    pub fn new(buffer_index: usize) -> Window {
        Window {
            buffer_index: buffer_index,
            y: 0,
        }
    }

    pub fn scroll_down(&mut self) {
        self.y += 1;
    }

    pub fn scroll_up(&mut self) {
        self.y -= 1;
    }

}
