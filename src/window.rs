use std::cmp::{min, max};
use ncurses::*;
use buffer::Buffer;

pub struct Window {
    pub buffer_index: usize,
    pub x: i32,
    pub y: i32,
}

impl Window {
    pub fn new(buffer_index: usize) -> Window {
        Window {
            buffer_index: buffer_index,
            x: 0,
            y: 0,
        }
    }

    pub fn bol(&mut self) {
        self.x = 0;
    }

    pub fn eol(&mut self) {
        // not implemented
    }

    pub fn move_to(&mut self, y: i32, x: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn move_left(&mut self) {
        self.x = max(0, self.x - 1);
    }

    pub fn move_down(&mut self) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        self.y = min(max_y - 1, self.y + 1);
    }

    pub fn move_up(&mut self) {
        self.y = max(0, self.y - 1);
    }

    pub fn move_right(&mut self) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        self.x = min(max_x - 1, self.x + 1);
    }
}
