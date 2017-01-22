use std::cmp::{min, max};
use std::fs::File;
use std::io::Write;
use window::Window;

pub struct Buffer {
    pub lines: Vec<String>,
    pub path: String,
}

impl Buffer {
    pub fn new(path: String) -> Buffer {
        Buffer {
            lines: vec![],
            path: path,
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

    pub fn insert(&mut self, c: &str, x: i32, y: i32) {
        let line = self.lines[y as usize].clone();
        let (a, b) = line.split_at(x as usize);
        self.lines[y as usize] = format!("{}{}{}", a, c, b);
    }

    pub fn insert_newline(&mut self, x: i32, y: i32) {
        let line = self.lines[y as usize].clone();
        let (a, b) = line.split_at(x as usize);
        self.lines[y as usize] = a.to_string();
        self.lines.insert((y + 1) as usize, b.to_string());
    }

    pub fn append_line(&mut self, line: String) {
        self.lines.push(Buffer::rem_tabs(line));
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
