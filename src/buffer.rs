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

    pub fn insertLine(&mut self, line: String, index: usize) {
        self.lines.insert(index, Buffer::remTabs(line));
    }

    pub fn appendLine(&mut self, line: String) {
        self.lines.push(Buffer::remTabs(line));
    }

    pub fn removeLine(&mut self, index: usize) {
        self.lines.remove(index);
    }

    pub fn moveLeft(&mut self) {
        self.x = max(0, self.x - 1);
        mv(self.y, self.x);
    }

    pub fn moveDown(&mut self) {
        self.y = self.y + 1;
        mv(self.y, self.x);
    }

    pub fn moveUp(&mut self) {
        self.y = max(0, self.y - 1);
        mv(self.y, self.x);
    }

    pub fn moveRight(&mut self) {
        self.x = self.x + 1;
        mv(self.y, self.x);
    }

    // private
    
    fn remTabs(line: String) -> String {
        line.replace("\t", "    ")
    }
}
