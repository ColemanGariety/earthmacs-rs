use std::cmp::{min, max};
use std;
use window::Window;
use buffer::Buffer;
use ncurses::*;

impl Window {
    pub fn handle_normal(&mut self, key: &str, buffer: &mut Buffer) {
        match key {
            "$" => {
                self.cursor_x = buffer.eol(self.cursor_y);
                self.col = 99999999;
            },
            "0" => { self.move_bol(); },
            "A" => {
                self.mode = "insert".to_string();
                while self.cursor_x < buffer.lines[self.cursor_y as usize].len() as i32 {
                    self.move_right();
                }
            },
            "d" => { self.mode = "delete".to_string(); },
            "f" => { self.mode = "find_char".to_string(); },
            "G" => {
                while self.scroll_y < buffer.lines.len() as i32 { // 
                    self.move_down();
                }
            }
            "h" => {
                if self.cursor_x > 0 {
                    self.move_left();
                }
            },
            "i" => { self.mode = "insert".to_string(); }
            "j" => {
                if self.cursor_y < (buffer.eof() - 1) {
                    self.move_down();
                    self.cursor_x  = min(buffer.eol(self.cursor_y), self.col);
                }
            },
            "k" => {
                if self.cursor_y > 0 {
                    self.move_up();
                    self.cursor_x  = min(buffer.eol(self.cursor_y), self.col);
                }
            },
            "l" => {
                if self.cursor_x < buffer.eol(self.cursor_y) {
                    self.move_right();
                }
            },
            "r" => { self.mode = "replace".to_string() },
            "x" => {
                let x = self.cursor_x;
                let y = self.cursor_y;
                buffer.remove(x, y);
                if x == buffer.eol(y) + 1 {
                    self.move_left();
                }
            }
            "<C-b>" => {
                for _ in 1..(self.real_height() - 2) {
                    self.move_up();
                    if self.cursor_y < self.scroll_y {
                        self.scroll_up();
                    }
                }
            },
            "<C-c>" => { endwin(); std::process::exit(0); },
            "<C-f>" => {
                for _ in 1..((self.real_height()) - 2) {
                    if self.cursor_y < (buffer.eof() - 1) {
                        self.move_down();
                        self.cursor_x  = min(buffer.eol(self.cursor_y), self.col);
                        if self.cursor_y >= (self.scroll_y + self.height - 2) {
                            self.scroll_down();
                        }
                    }
                }
            },
            // "<C-q>" => { self.destroy_active_window()},
            "<C-s>" => { buffer.save(); },
            _ => ()
        }
    }

    pub fn handle_delete(&mut self, key: &str, buffer: &mut Buffer) {
        match key {
            "<Escape>" => {
                self.mode = "normal".to_string();
                self.move_left();
            },
            "d" => {
                let row = self.cursor_y;
                buffer.remove_line(row as usize);
                self.mode = "normal".to_string();
            },
            _ => ()
        }
    }

    pub fn handle_insert(&mut self, key: &str, buffer: &mut Buffer) {
        match key {
            "<Escape>" => {
                self.mode = "normal".to_string();
                self.move_left();
            },
            "<DEL>" => {
                let x = self.cursor_x.clone();
                let y = self.cursor_y.clone();
                if x == 0 {
                    self.move_up();
                    while self.cursor_x < buffer.lines[self.cursor_y as usize].len() as i32 {
                        self.move_right();
                    }
                } else {
                    self.move_left();
                }
                buffer.remove(x - 1, y);
            },
            "<Enter>" => {
                buffer.insert_newline(self.cursor_x, self.cursor_y);
                self.move_down();
                self.move_bol();
            },
            _ => {
                buffer.insert(key, self.cursor_x, self.cursor_y);
                self.cursor_x += 1;
                self.col += 1;
            }
        }
    }

    pub fn handle_find_char(&mut self, key: &str, buffer: &mut Buffer) {
        match key {
            "<Escape>" => {
                self.mode = "normal".to_string();
                self.move_left();
            },
            _ => {
                let y = self.cursor_y as usize;
                let x = self.cursor_x as usize;
                match buffer.lines[y].chars().skip(x + 1).position(|c| char::to_string(&c).as_str() == key) {
                    Some(i) => {
                        self.cursor_x += (i + 1) as i32;
                        self.col += (i + 1) as i32;
                    },
                    _ => ()
                }
                self.mode = "normal".to_string();
            }
        }
    }

    pub fn handle_replace(&mut self, key: &str, buffer: &mut Buffer) {
        match key {
            "<Escape>" => {
                self.mode = "normal".to_string();
                self.move_left();
            },
            _ => {
                let x = self.cursor_x;
                let y = self.cursor_y;
                buffer.remove(x, y);
                buffer.insert(key, x, y);
                self.mode = "normal".to_string();
            },
        }
    }
}
