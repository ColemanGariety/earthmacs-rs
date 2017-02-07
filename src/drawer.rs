use std::cmp::{min, max};
use ncurses::*;

pub struct Drawer {
    pub prompt: String,
    pub value: String,
    pub lines: Vec<String>,
    pub active_line_index: i32,
}

static COLOR_PAIR_DEFAULT: i16 = 1;

impl Drawer {
    pub fn new_find_files() -> Drawer {
        let lines = vec!["mode.rs".to_string(), "editor.rs".to_string(), "drawer.rs".to_string(), "cell.rs".to_string(), "poll.rs".to_string()];
        let index = lines.len();
        Drawer{
            prompt: "Find files: ".to_string(),
            value: "/home/coleman/Git/earthmacs/src".to_string(),
            lines: lines,
            active_line_index: (index - 1) as i32,
        }
    }

    pub fn draw(&self, max_x: i32, max_y: i32) {
        let mut y = max_y - 1;
        let top_border = (0..max_x).map(|_| "-").collect::<String>();
        for (index, line) in self.lines.iter().enumerate() {
            mv(y, 0);
            clrtoeol();
            if index == self.active_line_index as usize {attron(COLOR_PAIR(50));}
            addstr(line.as_str());
            if index == self.active_line_index as usize {attroff(COLOR_PAIR(50));}
            y -= 1;
        }
        mv(y, 0);
        attron(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        addstr(top_border.as_str());
        attroff(COLOR_PAIR(COLOR_PAIR_DEFAULT));
    }

    pub fn next_item(&mut self) {
        self.active_line_index = max(0, self.active_line_index - 1);
    }

    pub fn prev_item(&mut self) {
        self.active_line_index = min((self.lines.len() - 1) as i32, self.active_line_index + 1);
    }
}
