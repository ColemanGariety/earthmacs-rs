use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use ncurses::*;
use buffer::Buffer;
use window::Window;

pub struct Editor {
    buffers: Vec<Buffer>,
    windows: Vec<Window>,
}

impl Editor {
    pub fn new() -> Editor {
        let mut welcome = Buffer::new();
        Editor {
            windows: vec![Window::new(0)],
            buffers: vec![welcome],
        }
    }

    pub fn handle_input(&mut self, input: i32) {
        match input {
            36 => { // $
                for window in self.windows.iter_mut() {
                    window.eol();
                }
            },
            48 => { // 0
                for window in self.windows.iter_mut() {
                    window.bol();
                }
            },
            104 => { // h
                for window in self.windows.iter_mut() {
                    window.move_left();
                }
            },
            106 => { // j
                for window in self.windows.iter_mut() {
                    window.move_down();
                }
            },
            107 => { // k
                for window in self.windows.iter_mut() {
                    window.move_up();
                }
            },
            108 => { // l
                for window in self.windows.iter_mut() {
                    window.move_right();
                }
            },
            _ => ()
        }
    }

    pub fn open(&mut self, path: String) {
        match File::open(path) {
            Ok(f) => {
                let mut buf = Buffer::new();
                let reader = BufReader::new(f);
                for line in reader.lines() {
                    buf.append_line(line.unwrap());
                }
                self.buffers.push(buf);
            },
            Err(e) => ()
        }
    }

    pub fn draw(&self) {
        let ref buf = self.buffers[1];
        let ref win = self.windows[0];
        for (index, line) in buf.lines.iter().enumerate() {
            mvprintw(index as i32, 0, line);
        }
        mv(win.y, win.x);
    }
}
