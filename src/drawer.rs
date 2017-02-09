use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::cmp::{min, max};
use ncurses::*;
use fuzzyrusty::fuzz;

pub struct Drawer {
    pub prompt: String,
    pub value: String,
    pub lines: Vec<String>,
    pub active_line_index: i32,
    pub scroll_index: i32,
}

static COLOR_PAIR_DEFAULT: i16 = 1;

impl Drawer {
    pub fn new_find_files(path: &PathBuf) -> Drawer {
        let dir;
        if path.is_dir() { dir = path.to_owned(); }
        else { dir = path.parent().unwrap().to_path_buf(); }
        let paths: Vec<String> = fs::read_dir(dir.clone()).unwrap().map(|res| res.unwrap().file_name().to_string_lossy().into_owned()).collect();
        let index = paths.len();
        
        Drawer{
            prompt: "Find files: ".to_string(),
            value: dir.to_str().unwrap().to_string() + "/",
            lines: paths,
            active_line_index: 0,
            scroll_index: 0,
        }
    }

    pub fn draw(&self, max_x: i32, max_y: i32) {
        let height = min(self.lines.len(), ((max_y / 2) - 3) as usize);
        let top_border = (0..max_x).map(|_| "-").collect::<String>();
        let mut y = max_y - height as i32 - 3;

        mv(y, 0);
        attron(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        addstr(top_border.as_str());
        attroff(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        y += 1;

        for (index, line) in self.lines.iter().enumerate() {
            if index >= self.scroll_index as usize && index < self.scroll_index as usize + height {
                mv(y, 0);
                clrtoeol();
                if index == self.active_line_index as usize {attron(COLOR_PAIR(204));}
                addstr(line.as_str());
                if index == self.active_line_index as usize {attroff(COLOR_PAIR(204));}
                y += 1;
            }
        }

        mv(y, 0);
        attron(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        addstr(top_border.as_str());
        attroff(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        y += 1;

        mv(y, 0);
        clrtoeol();
        let ln = format!("{}{}", self.prompt, self.value);
        addstr(ln.as_str());
        y += 1;

    }

    pub fn next_item(&mut self) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let height = min(self.lines.len(), ((max_y / 2) - 3) as usize);

        if self.active_line_index == self.lines.len() as i32 - 1 {
            self.active_line_index = 0;
            self.scroll_index = 0;
        } else {
            self.active_line_index += 1;
            if self.active_line_index == self.scroll_index + height as i32 {
                self.scroll_index += 1;
            }
        }
    }

    pub fn prev_item(&mut self) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let height = min(self.lines.len(), ((max_y / 2) - 3) as usize);

        if self.active_line_index == 0 {
            self.active_line_index = self.lines.len() as i32 - 1;
            self.scroll_index = self.active_line_index + 1 - height as i32;
        } else {
            self.active_line_index = max(0, self.active_line_index - 1);
            if self.active_line_index == self.scroll_index - 1 {
                self.scroll_index = max(0, self.scroll_index - 1);
            }
        }
    }

    pub fn update_list(&mut self) {
        let v = &self.value;
        let pb = PathBuf::from(v);
        let mut dir = pb.clone();
        if pb.to_string_lossy().to_owned().chars().last().unwrap() != '/' {
            dir = pb.parent().unwrap().to_owned();
        }
        let file = Path::new(v).file_name().unwrap().to_str().to_owned().unwrap();
        let paths: Vec<String> = fs::read_dir(dir).unwrap().map(|res| res.unwrap().file_name().to_string_lossy().into_owned()).collect();
        if Path::new(v).to_str().unwrap().chars().last().unwrap() == '/' {
            self.lines = paths;
        } else {
            self.lines = paths.iter().filter(|path| (fuzz::ratio(file, path) > 10 && fuzz::partial_ratio(file, path) > 80) || fuzz::token_sort_ratio(file, path, true, true) > 50).cloned().collect();
        }
        self.active_line_index = 0;
        self.scroll_index = 0;
    }

    pub fn handle_key(&mut self, key: &str) {
        match key {
            "<Backspace>" | "<DEL>" => {
                if self.value.len() != 0 {
                    self.value = self.value[..(self.value.len() - 1)].to_string();
                }
                self.update_list();
            },
            "<Tab>" => {
                let v = PathBuf::from(&self.value);
                let mut p;
                if !(v.to_str().unwrap().chars().last().unwrap() == '/') { p = v.parent().unwrap().to_path_buf(); }
                else { p = v; }
                p = p.join(&self.lines[self.active_line_index as usize]);
                if p.is_dir() {
                    self.value = p.to_str().unwrap().to_string() + "/";
                } else {
                    self.value = p.to_str().unwrap().to_string();
                }
                self.update_list();
            },
            "<C-l>" => {
                let old = self.value.clone();
                let mut p = PathBuf::from(&old);
                p.pop();
                self.value = p.to_str().unwrap().to_string() + "/";
                self.update_list();
            },
            _ => {
                self.value = self.value.clone() + key;
                self.update_list();
            }
        }
    }
}
