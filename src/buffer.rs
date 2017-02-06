use std::cmp::{min, max};
use std::fs::File;
use std::io::Write;
use cell::Cell;
use window::Window;

use std::path::Path;
use syntect::parsing::syntax_definition::SyntaxDefinition;
use syntect::parsing::SyntaxSet;

pub struct Buffer {
    pub lines: Vec<Vec<Cell>>,
    pub path: String,
    pub syntax: SyntaxDefinition,
}

impl Buffer {
    pub fn new(path: String) -> Buffer {
        let mut ps = SyntaxSet::load_defaults_nonewlines();
        ps.link_syntaxes();
        let p = &path.clone();
        let ext = Path::new(p).extension().unwrap().to_str().unwrap();
        Buffer {
            lines: vec![],
            path: path,
            syntax: ps.find_syntax_by_extension(ext).unwrap().clone()
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
            new.append(&mut b.to_vec());
            self.lines[y as usize] = new;
        }
    }

    pub fn insert(&mut self, c: &str, x: i32, y: i32) {
        let line = self.lines[y as usize].clone();
        let (mut a, mut b) = line.split_at(x as usize);
        let mut new = a.to_owned();
        // new.append(&mut c.to_string().chars());
        new.append(&mut b.to_vec());
        self.lines[y as usize] = new;
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
