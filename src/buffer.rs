use ncurses::*;
use std::cmp::{min, max};

pub struct Buffer {
    pub lines: Vec<String>,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub scroll_y: i32,
    pub col: i32,
    pub row: i32,
    pub path: String,
    pub window: *mut i8,
}

impl Buffer {
    pub fn new(path: String) -> Buffer {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        Buffer {
            lines: vec![],
            cursor_x: 0,
            cursor_y: 0,
            scroll_y: 0,
            col: 0,
            row: 0,
            path: path,
            window: subwin(stdscr(), max_y, max_x, 0, 0),
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
        if self.cursor_x <= 0 {
            self.cursor_x = 0;
        } else {
            self.cursor_x = self.cursor_x - 1;
            self.col = self.cursor_x;
        }
    }

    pub fn move_down(&mut self) {
        self.cursor_y = min((self.lines.len() - 1) as i32, self.cursor_y + 1);
        self.row = self.cursor_y;
        self.cursor_x = min(self.eol(), self.col);
    }

    pub fn move_up(&mut self) {
        self.cursor_y = max(0, self.cursor_y - 1);
        self.row = self.cursor_y;
        self.cursor_x = min(self.eol(), self.col);
    }

    pub fn move_right(&mut self) {
        if self.cursor_x >= self.eol() {
            self.cursor_x = self.eol()
        } else {
            self.cursor_x = self.cursor_x + 1;
            self.col = self.cursor_x;
        }
    }

    pub fn move_bol(&mut self) {
        self.cursor_x = 0;
        self.col = 0;
    }

    pub fn move_eol(&mut self) {
        self.cursor_x = self.eol();
        self.col = 999999999;
    }

    pub fn scroll_down(&mut self) {
        self.scroll_y += 1;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_y -= 1;
    }

    // private

    fn eol(&self) -> i32 {
        max(0, (self.lines[self.cursor_y as usize].len() as i32) - 1)
    }

    fn rem_tabs(line: String) -> String {
        line.replace("\t", "    ")
    }
}
