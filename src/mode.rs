use std;
use std::io::Write;
use editor::Editor;
use buffer::Buffer;
use ncurses::*;

impl Buffer {
    pub fn handle_normal(&mut self, key: &str) {
        match key {
            "$" => {
                self.move_eol();
            },
            "0" => {
                self.move_bol();
            },
            "A" => {
                self.mode = "insert".to_string();
                let y = self.cursor_y;
                self.move_eol();
            },
            "d" => {
                self.mode = "delete".to_string();
            }
            "h" => {
                self.move_left();
            },
            "i" => {
                self.mode = "insert".to_string();
            }
            "j" => {
                self.move_down();
                if self.cursor_y >= (self.scroll_y + Editor::get_max_y() - 2) {
                    self.scroll_down();
                }
            },
            "k" => {
                self.move_up();
                if self.cursor_y < self.scroll_y {
                    self.scroll_up();
                }
            },
            "l" => {
                self.move_right();
            },
            "<C-c>" => {
                endwin();
                std::process::exit(0);
            },
            "<C-f>" => {
                for _ in 1..(Editor::get_max_y() - 2) {
                    self.move_down();
                    if self.cursor_y >= (self.scroll_y + Editor::get_max_y() - 2) {
                        self.scroll_down();
                    }
                }
            },
            "<C-b>" => {
                for _ in 1..(Editor::get_max_y() - 2) {
                    self.move_up();
                    if self.cursor_y < self.scroll_y {
                        self.scroll_up();
                    }
                }
            },
            "<C-s>" => {
                self.save();
            }
            _ => ()
        }
    }

    pub fn handle_delete(&mut self, key: &str) {
        match key {
            "<Escape>" => {
                self.mode = "normal".to_string();
                self.move_left();
            },
            "d" => {
                let row = self.cursor_y;
                self.remove_line(row as usize);
                self.mode = "normal".to_string();
            },
            _ => ()
        }
    }

    pub fn handle_insert(&mut self, key: &str) {
        match key {
            "<Escape>" => {
                self.mode = "normal".to_string();
                self.move_left();
            },
            "<DEL>" => {
                let y = self.cursor_y as usize;
                let x = self.cursor_x as usize;
                let line = self.lines[y].clone();
                if x == 0 {
                    self.remove_line(y as usize);
                    self.move_up();
                    self.move_eol();
                } else {
                    let (a, b) = line.split_at(x - 1);
                    self.lines[y] = a.to_string() + &(b.to_string())[1..];
                    self.move_left();
                }
            },
            "<Enter>" => {
                let y = self.cursor_y;
                self.insert_line("".to_string(), (y + 1) as usize);
                self.move_down();
            },
            _ => {
                let y = self.cursor_y as usize;
                let x = self.cursor_x as usize;
                let line = self.lines[y].clone();
                let (a, b) = line.split_at(x);
                self.lines[y] = format!("{}{}{}", a, key, b);
                self.cursor_x += 1;
                self.col += 1;
            }
        }
    }
}
