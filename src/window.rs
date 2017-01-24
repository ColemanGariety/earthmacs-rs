use std::cmp::{min, max};
use ncurses::*;

pub struct Window {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub col: i32,
    pub row: i32,
    pub pane: *mut i8,
    pub scroll_y: i32,
    pub split: String,
    pub buffer: i32,
    pub mode: String,
}

impl Window {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Window {
        Window {
            x: x,
            y: y,
            width: width,
            height: height,
            cursor_x: 0,
            cursor_y: 0,
            col: 0,
            row: 0,
            scroll_y: 0,
            pane: subwin(stdscr(), 1, 1, 0, 0),
            split: "none".to_string(),
            buffer: 0,
            mode: "normal".to_string(),
        }
    }

    pub fn split_horizontally(&mut self) -> Window {
        self.width = self.width / 2;
        self.split = "horizontal".to_string();
        Window {
            x: self.width,
            y: self.y,
            width: self.width,
            height: self.height,
            cursor_x: 0,
            cursor_y: 0,
            col: 0,
            row: 0,
            scroll_y: 0,
            pane: subwin(stdscr(), 1, 1, 0, 0),
            split: "horizontal".to_string(),
            buffer: self.buffer,
            mode: "normal".to_string(),
        }
    }

    pub fn split_vertically(&mut self) -> Window {
        self.height = self.height / 2;
        self.split = "vertical".to_string();
        Window {
            x: self.x,
            y: self.height,
            width: self.width,
            height: self.height,
            cursor_x: 0,
            cursor_y: 0,
            col: 0,
            row: 0,
            scroll_y: self.scroll_y,
            pane: subwin(stdscr(), 1, 1, 0, 0),
            split: "horizontal".to_string(),
            buffer: self.buffer,
            mode: "normal".to_string(),
        }
    }

    pub fn unsplit_horizontally(&mut self) {
        self.split = "none".to_string();
        self.width = self.width * 2;
    }

    pub fn unsplit_vertically(&mut self) {
        self.split = "none".to_string();
        self.height = self.height * 2;
    }

    pub fn real_height(&mut self) -> i32 {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        return (max_y as f64 * (self.height as f64 / 100.0)) as i32;
    }

    pub fn real_width(&mut self) -> i32 {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        return (max_x as f64 * (self.width as f64 / 100.0)) as i32;
    }

    pub fn real_x(&mut self) -> i32 {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        return (max_x as f64 * (self.x as f64 / 100.0)) as i32;
    }

    pub fn real_y(&mut self) -> i32 {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        return (max_y as f64 * (self.y as f64 / 100.0)) as i32;
    }

    pub fn move_left(&mut self) {
        self.cursor_x = max(0, self.cursor_x - 1);
        self.col = max(0, self.cursor_x);
    }

    pub fn move_down(&mut self) {
        self.cursor_y = self.cursor_y + 1;
        self.row = self.cursor_y;
        if self.cursor_y >= (self.scroll_y + (self.real_height()) - 2) {
            self.scroll_down();
        }
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
