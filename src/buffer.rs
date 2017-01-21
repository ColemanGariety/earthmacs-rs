use std::cmp::{min, max};

pub struct Buffer {
    pub lines: Vec<String>,
    pub x: i32,
    pub y: i32,
    pub col: i32,
    pub row: i32,
    pub path: String,
}

impl Buffer {
    pub fn new(path: String) -> Buffer {
        Buffer {
            lines: vec![],
            x: 0,
            y: 0,
            col: 0,
            row: 0,
            path: path,
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
        if self.x <= 0 {
            self.x = 0;
        } else {
            self.x = self.x - 1;
            self.col = self.x;
        }
    }

    pub fn move_down(&mut self) {
        self.y = min((self.lines.len() - 1) as i32, self.y + 1);
        self.row = self.y;
        self.x = min(self.eol(), self.col);
    }

    pub fn move_up(&mut self) {
        self.y = max(0, self.y - 1);
        self.row = self.y;
        self.x = min(self.eol(), self.col);
    }

    pub fn move_right(&mut self) {
        if self.x >= self.eol() {
            self.x = self.eol()
        } else {
            self.x = self.x + 1;
            self.col = self.x;
        }
    }

    pub fn move_bol(&mut self) {
        self.x = 0;
        self.col = 0;
    }

    pub fn move_eol(&mut self) {
        self.x = self.eol();
        self.col = 999999999;
    }

    // private

    fn eol(&self) -> i32 {
        max(0, (self.lines[self.y as usize].len() as i32) - 1)
    }

    fn rem_tabs(line: String) -> String {
        line.replace("\t", "    ")
    }
}
