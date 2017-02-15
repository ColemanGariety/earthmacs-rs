use std::cmp::{max};
use ncurses::*;

#[derive(Clone)]
pub struct Window {
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub col: i32,
    pub row: i32,
    pub scroll_y: i32,
    pub pane: *mut i8,
    pub buffer_index: i32,
    pub mode: String,
    pub active: bool,
    pub mark: Option<(i32, i32)>,
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
            mark: None,
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

    pub fn calc_mark_region(&self) -> Option<((i32, i32), (i32, i32))> {
        match self.mark {
            Some(mark) => {
                let starts_with_mark;
                if mark.0 == self.cursor_y { starts_with_mark = self.mark.unwrap().1 <= self.cursor_x; }
                else { starts_with_mark = mark.0 < self.cursor_y; }
                let x;
                let y;
                let endx;
                let endy;
                if starts_with_mark {
                    y = self.cursor_y;
                    x = self.cursor_x;
                    endy = mark.0;
                    endx = mark.1;
                } else {
                    y = mark.0;
                    x = mark.1;
                    endy = self.cursor_y;
                    endx = self.cursor_x;
                }
                Some(((x, y), (endx, endy)))
            },
            None => { None }
        }
    }
}
