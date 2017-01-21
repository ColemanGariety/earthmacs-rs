use std;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use ncurses::*;

use buffer::Buffer;
use pane::Pane;

pub struct Editor {
    buffers: Vec<Buffer>,
    panes: Vec<Pane>,
}


impl Editor {
    pub fn new() -> Editor {
        let welcome = Buffer::new();
        Editor {
            panes: vec![Pane::new(1)],
            buffers: vec![welcome],
        }
    }

    pub fn handle_input(&mut self, key: &str) {
        match key {
            "$" => {
                let ref mut pane = self.panes[0];
                self.buffers[pane.buffer_index].move_eol();
            },
            "0" => {
                let ref mut pane = self.panes[0];
                self.buffers[pane.buffer_index].move_bol();
            },
            "h" => {
                let ref mut pane = self.panes[0];
                self.buffers[pane.buffer_index].move_left();
            },
            "j" => {
                let ref mut pane = self.panes[0];
                let ref mut buf = self.buffers[pane.buffer_index];
                buf.move_down();
                if buf.y >= (pane.y + Editor::get_max_y() - 2) {
                    pane.scroll_down();
                }
            },
            "k" => {
                let ref mut pane = self.panes[0];
                let ref mut buf = self.buffers[pane.buffer_index];
                buf.move_up();
                if buf.y < pane.y {
                    pane.scroll_up();
                }
            },
            "l" => {
                let ref mut pane = self.panes[0];
                self.buffers[pane.buffer_index].move_right();
            },
            "<C-c>" => {
                endwin();
                std::process::exit(0);
            },
            "<C-f>" => {
                let ref mut pane = self.panes[0];
                let ref mut buf = self.buffers[pane.buffer_index];
                for _ in 1..(Editor::get_max_y()) {
                    buf.move_down();
                    if buf.y >= (pane.y + Editor::get_max_y() - 2) {
                        pane.scroll_down();
                    }
                }
            },
            "<C-b>" => {
                let ref mut pane = self.panes[0];
                let ref mut buf = self.buffers[pane.buffer_index];
                for _ in 1..(Editor::get_max_y()) {
                    buf.move_up();
                    if buf.y < pane.y {
                        pane.scroll_up();
                    }
                }
            }
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
            Err(_) => ()
        }
    }

    pub fn draw(&self) {
        let ref buf = self.buffers[1];
        let ref pane = self.panes[0];
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let lines = buf.lines.iter().skip(pane.y as usize).take(max_y as usize);
        for (index, line) in lines.enumerate() {
            wmove(pane.window, (index + 1) as i32, 0);
            wclrtoeol(pane.window);
            waddstr(pane.window, format!(" {}", line).as_str());
        }
        box_(pane.window, 0, 0);
        wresize(pane.window, max_y, max_x);
        wmove(pane.window, (buf.y - pane.y) + 1, buf.x + 1);
        wrefresh(pane.window);
    }

    pub fn get_max_y() -> i32 {
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        max_y
    }
}
