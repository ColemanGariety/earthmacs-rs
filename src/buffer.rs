use std::cmp::{min, max};
use ncurses::*;

pub struct Buffer {
    pub lines: Vec<String>,
    pub x: i32,
    pub y: i32,
    pub mode: String,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            lines: vec![],
            x: 0,
            y: 0,
            mode: "normal".to_string(),
        }
    }

    pub fn insert_line(&mut self, line: String, index: usize) {
        self.lines.insert(index, Buffer::rem_tabs(line));
    }

    pub fn append_line(&mut self, line: String) {
        self.lines.push(Buffer::rem_tabs(line));
    }

    pub fn remove_line(&mut self, index: usize) {
        self.lines.remove(index);
    }

    pub fn move_left(&mut self) {
        self.x = max(0, self.x - 1);
    }

    pub fn move_down(&mut self) {
        self.y = min(self.lines.len() as i32, self.y + 1);
        self.x = min(self.lines[self.y as usize].len() as i32, self.x);
    }

    pub fn move_up(&mut self) {
        self.y = max(0, self.y - 1);
        self.x = min(self.lines[self.y as usize].len() as i32, self.x);
    }

    pub fn move_right(&mut self) {
        self.x = min(self.lines[self.y as usize].len() as i32, self.x + 1);
    }

    pub fn bol(&mut self) {
        self.x = 0;
    }

    pub fn eol(&mut self) {
        // not implemented
    }

    // private

    fn rem_tabs(line: String) -> String {
        line.replace("\t", "    ")
    }
}
