use ncurses::*;
use std::cmp::{min, max};
use std::fs::File;
use std::io::Write;
use editor::Editor;

pub struct Buffer {
    pub lines: Vec<String>,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub scroll_y: i32,
    pub col: i32,
    pub row: i32,
    pub path: String,
    pub window: *mut i8,
    pub mode: String,
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
            mode: "normal".to_string(),
        }
    }

    pub fn save(&self) {
        match File::create(&self.path) {
            Ok(mut f) => {
                match f.write_all(self.lines.join("\n").as_bytes()) {
                    Ok(_) => (),
                    Err(e) => panic!(e)
                };
            },
            Err(_) => ()
        }
    }

    pub fn handle_input(&mut self, key: &str) {
        match self.mode.as_str() {
            "normal" => { self.handle_normal(key); },
            "delete" => { self.handle_delete(key); },
            "insert" => { self.handle_insert(key); },
            "find_char" => { self.handle_find_char(key); },
            "replace" => { self.handle_replace(key); },
            _ => ()
        }
    }

    pub fn remove(&mut self, x: i32, y: i32) {
        let line = self.lines[y as usize].clone();
        if x == -1 || line.len() == 0 {
            self.lines[(y - 1) as usize] += line.as_str();
            self.remove_line(y as usize);
        } else {
            let (a, b) = line.split_at(x as usize);
            self.lines[y as usize] = a.to_string() + &(b.to_string())[1..];
        }
    }

    pub fn insert(&mut self, c: &str) {
        let y = self.cursor_y as usize;
        let x = self.cursor_x as usize;
        let line = self.lines[y].clone();
        let (a, b) = line.split_at(x);
        self.lines[y] = format!("{}{}{}", a, c, b);
    }

    pub fn insert_line(&mut self) {
        let y = self.cursor_y as usize;
        let x = self.cursor_x as usize;
        let line = self.lines[y].clone();
        let (a, b) = line.split_at(x);
        self.lines[y] = a.to_string();
        self.lines.insert((y + 1), b.to_string());
    }

    pub fn append_line(&mut self, line: String) {
        self.lines.push(Buffer::rem_tabs(line));
    }

    pub fn remove_line(&mut self, index: usize) {
        self.lines.remove(index);
    }

    pub fn move_left(&mut self) {
        self.cursor_x = max(0, self.cursor_x - 1);
        self.col = self.cursor_x;
    }

    pub fn move_down(&mut self) {
        self.cursor_y = min((self.lines.len() - 1) as i32, self.cursor_y + 1);
        self.row = self.cursor_y;
        self.cursor_x = min(self.eol(), self.col);
        if self.cursor_y >= (self.scroll_y + Editor::height() - 2) {
            self.scroll_down();
        }
    }

    pub fn move_up(&mut self) {
        self.cursor_y = max(0, self.cursor_y - 1);
        self.row = self.cursor_y;
        self.cursor_x = min(self.eol(), self.col);
        if self.cursor_y < self.scroll_y {
            self.scroll_up();
        }
    }

    pub fn move_right(&mut self) {
        self.cursor_x = min(self.eol(), self.cursor_x + 1);
        self.col = self.cursor_x;
    }

    pub fn move_bol(&mut self) {
        self.cursor_x = 0;
        self.col = 0;
    }

    pub fn move_eol(&mut self) {
        let y = self.cursor_y;
        self.cursor_x = self.eol();
        self.col = 999999999;
        if self.lines[y as usize].len() != 0 {
            self.cursor_x += 1;
        }
    }

    pub fn move_eof(&mut self) {
        for _ in 0..(self.lines.len() - self.scroll_y as usize) {
            self.move_down();
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll_y += 1;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_y -= 1;
    }

    pub fn page_down(&mut self) {
        for _ in 1..(Editor::height() - 2) {
            self.move_down();
            if self.cursor_y >= (self.scroll_y + Editor::height() - 2) {
                self.scroll_down();
            }
        }
    }

    pub fn page_up(&mut self) {
        for _ in 1..(Editor::height() - 2) {
            self.move_up();
            if self.cursor_y < self.scroll_y {
                self.scroll_up();
            }
        }
    }

    pub fn eol(&self) -> i32 {
        max(0, (self.lines[self.cursor_y as usize].len() as i32) - 1)
    }

    // private

    fn rem_tabs(line: String) -> String {
        line.replace("\t", "    ")
    }
}
