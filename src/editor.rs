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
            windows: vec![Window::new(1)],
            buffers: vec![welcome],
        }
    }

    pub fn handle_input(&mut self, input: i32) {
        match input {
            36 => { // $
                let ref mut window = self.windows[0];
                self.buffers[window.buffer_index].eol();
            },
            48 => { // 0
                let ref mut window = self.windows[0];
                self.buffers[window.buffer_index].bol();
            },
            104 => { // h
                let ref mut window = self.windows[0];
                self.buffers[window.buffer_index].move_left();
            },
            106 => { // j
                let ref mut win = self.windows[0];
                let ref mut buf = self.buffers[win.buffer_index];
                buf.move_down();
                if buf.y >= (win.y + Editor::get_max_y()) {
                    win.scroll_down();
                }
            },
            107 => { // k
                let ref mut win = self.windows[0];
                let ref mut buf = self.buffers[win.buffer_index];
                buf.move_up();
                if buf.y < win.y {
                    win.scroll_up();
                }
            },
            108 => { // l
                let ref mut window = self.windows[0];
                self.buffers[window.buffer_index].move_right();
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
        let y = win.y - buf.y;
        let x = buf.x;
        let lines = buf.lines.iter().skip(win.y as usize).take(Editor::get_max_y() as usize);
        for (index, line) in lines.enumerate() {
            mv(index as i32, 0);
            clrtoeol();
            printw(line);
        }
        mv(buf.y - win.y, buf.x);
    }

    // Private

    fn get_max_y() -> i32 {
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        max_y
    }

}
