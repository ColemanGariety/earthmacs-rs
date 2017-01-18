use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use buffer::Buffer;
use window::Window;

pub struct Editor {
    buffers: Vec<Buffer>,
    windows: Vec<Window>,
}

impl Editor {
    pub fn new() -> Editor {
        let welcome = Buffer::new();
        Editor {
            buffers: vec![welcome],
            windows: vec![Window::new(0)],
        }
    }

    pub fn handleInput(&mut self, input: i32) {
        match input {
            106 => { // j
                for window in self.windows.iter() {
                    self.buffers[window.buffer_index].moveDown();
                }
            },
            _ => ()
        }
    }

    pub fn open(&mut self, path: String) {
        match File::open(path) {
            Ok(f) => {
                let mut buf = Buffer::new();
                let mut reader = BufReader::new(f);
                for line in reader.lines() {
                    buf.appendLine(line.unwrap());
                }
                self.buffers.push(buf);
            },
            Err(e) => ()
        }
    }
}
