use std::path::Path;
use syntect;
use syntect::easy::HighlightLines;
use std::cmp::{min, max};
use std::fs::File;
use std::io::Write;
use cell::Cell;
use window::Window;
use util;

pub struct Buffer {
    pub lines: Vec<Vec<Cell>>,
    pub path: String,
    pub highlighter: syntect::parsing::SyntaxDefinition,
    pub ts: syntect::highlighting::ThemeSet,
}

impl Buffer {
    pub fn new(path: String, highlighter: syntect::parsing::SyntaxDefinition, ts: syntect::highlighting::ThemeSet) -> Buffer {
        Buffer {
            lines: vec![],
            path: path,
            highlighter: highlighter,
            ts: ts,
        }
    }

    pub fn save(&self) {
        // match File::create(&self.path) {
        //     Ok(mut f) => {
        //         let mut lns = self.lines.join("\n");
        //         lns.push('\n');
        //         match f.write_all(lns) {
        //             Ok(_) => (),
        //             Err(e) => panic!(e)
        //         };
        //     },
        //     Err(_) => ()
        // }
    }

    pub fn remove(&mut self, x: i32, y: i32) {
        let mut line = self.lines[y as usize].clone();
        if x == -1 || line.len() == 0 {
            self.lines[(y - 1) as usize].append(&mut line);
            self.remove_line(y as usize);
        } else {
            let (a, mut b) = line.split_at(x as usize);
            let mut new = a.to_vec();
            new.append(&mut (&b[1..]).to_vec());
            self.lines[y as usize] = new;
        }
    }

    pub fn insert(&mut self, c: &str, x: i32, y: i32) {
        let mut line = self.lines[y as usize].clone();
        let mut h = HighlightLines::new(&self.highlighter, &self.ts.themes["base16-ocean.dark"]);
        let mut new = vec![];
        if line.len() == 0 {
            line.append(&mut c.chars().map(|ch| Cell::new(ch, 1)).collect());
            new = line;
        } else {
            let (mut a, mut b) = line.split_at(x as usize);
            new.append(&mut a.to_vec());
            new.append(&mut c.chars().map(|ch| Cell::new(ch, 1)).collect());
            new.append(&mut b.to_vec());
        }

        let line_string: String = new.iter().cloned().map(|c| c.ch).collect();
        self.lines[y as usize] = vec![];
        let ranges = h.highlight(line_string.as_str());
        for (style, text) in ranges {
            let color = util::rgb_to_short(format!("{0:02.x}{1:02.x}{2:02.x}", style.foreground.r, style.foreground.g, style.foreground.b).as_str());
            for ch in text.chars() {
                self.lines[y as usize].push(Cell::new(ch, color as i32));
            }
        }
    }

    pub fn insert_newline(&mut self, x: i32, y: i32) {
        let line = self.lines[y as usize].clone();
        let (a, b) = line.split_at(x as usize);
        self.lines[y as usize] = a.to_vec();
        self.lines.insert((y + 1) as usize, b.to_vec());
    }

    pub fn append_line(&mut self, line: String) {
        let mut ln = vec![];
        for ch in line.chars() {
            ln.push(Cell::new(ch, 0));
        }
        self.lines.push(ln);
    }

    pub fn remove_line(&mut self, index: usize) {
        self.lines.remove(index);
    }

    pub fn eol(&self, y: i32) -> i32 {
        max(0, (self.lines[y as usize].len() as i32) - 1)
    }

    pub fn eof(&self) -> i32 {
        self.lines.len() as i32
    }

    // private

    fn rem_tabs(line: String) -> String {
        line.replace("\t", "    ")
    }
}
