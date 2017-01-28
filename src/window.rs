use std::cmp::{min, max};
use ncurses::*;
use editor::Editor;
use buffer::Buffer;

#[derive(Clone)]
pub struct Window {
    pub id: i32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub col: i32,
    pub row: i32,
    pub scroll_y: i32,
    pub pane: *mut i8,
    pub buffer_index: i32,
    pub mode: String,
    pub active: bool,
}

impl Window {
    pub fn new() -> Window {
        Window {
            cursor_x: 0,
            cursor_y: 0,
            col: 0,
            row: 0,
            scroll_y: 0,
            pane: subwin(stdscr(), 1, 1, 0, 0),
            buffer_index: 0,
            mode: "normal".to_string(),
            active: false,
            id: 0,
        }
    }

    pub fn move_left(&mut self) {
        self.cursor_x = max(0, self.cursor_x - 1);
        self.col = max(0, self.cursor_x);
    }

    pub fn move_down(&mut self) {
        self.cursor_y = self.cursor_y + 1;
        self.row = self.cursor_y;
    }

    pub fn move_up(&mut self) {
        self.cursor_y = max(0, self.cursor_y - 1);
        self.row = self.cursor_y;
        if self.cursor_y < self.scroll_y {
            self.scroll_up();
        }
    }

    pub fn move_right(&mut self) {
        self.cursor_x = self.cursor_x + 1;
        self.col = self.cursor_x;
    }

    pub fn move_bol(&mut self) {
        self.cursor_x = 0;
        self.col = 0;
    }

    pub fn scroll_down(&mut self) {
        self.scroll_y += 1;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_y -= 1;
    }
}
