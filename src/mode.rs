use std;
use buffer::Buffer;
use ncurses::*;

impl Buffer {
    pub fn handle_normal(&mut self, key: &str) {
        match key {
            "$" => { self.move_eol(); },
            "0" => { self.move_bol(); },
            "A" => {
                self.mode = "insert".to_string();
                self.move_eol();
            },
            "d" => { self.mode = "delete".to_string(); },
            "f" => { self.mode = "find_char".to_string(); },
            "G" => { self.move_eof(); }
            "h" => { self.move_left(); },
            "i" => { self.mode = "insert".to_string(); }
            "j" => { self.move_down(); },
            "k" => { self.move_up(); },
            "l" => { self.move_right(); },
            "r" => { self.mode = "replace".to_string() },
            "x" => {
                let x = self.cursor_x;
                let y = self.cursor_y;
                self.remove(x, y);
                if x == self.eol() + 1 {
                    self.move_left();
                }
            }
            "<C-c>" => { endwin(); std::process::exit(0); },
            "<C-f>" => { self.page_down() },
            "<C-b>" => { self.page_up() },
            "<C-s>" => { self.save(); },
            // "<M-H>" => { self.split_vertical(); },
            // "<M-J>" => { self.split_horizontal(); },
            // "<M-K>" => { self.split_vertical(); },
            // "<M-L>" => { self.split_horizontal(); },
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
                let x = self.cursor_x.clone();
                let y = self.cursor_y.clone();
                if x == 0 {
                    self.move_up();
                    self.move_eol();
                } else {
                    self.move_left();
                }
                self.remove(x - 1, y);
            },
            "<Enter>" => {
                self.insert_line();
                self.move_down();
                self.move_bol();
            },
            _ => {
                self.insert(key);
                self.cursor_x += 1;
                self.col += 1;
            }
        }
    }

    pub fn handle_find_char(&mut self, key: &str) {
        match key {
            "<Escape>" => {
                self.mode = "normal".to_string();
                self.move_left();
            },
            _ => {
                let y = self.cursor_y as usize;
                let x = self.cursor_x as usize;
                match self.lines[y].chars().skip(x + 1).position(|c| char::to_string(&c).as_str() == key) {
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

    pub fn handle_replace(&mut self, key: &str) {
        match key {
            "<Escape>" => {
                self.mode = "normal".to_string();
                self.move_left();
            },
            _ => {
                let x = self.cursor_x;
                let y = self.cursor_y;
                self.remove(x, y);
                self.insert(key);
                self.mode = "normal".to_string();
            },
        }
    }
}
