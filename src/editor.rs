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
        let mut welcome = Buffer::new();
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
                let ref mut win = self.panes[0];
                let ref mut buf = self.buffers[win.buffer_index];
                buf.move_down();
                if buf.y >= (win.y + Editor::get_max_y()) {
                    win.scroll_down();
                }
            },
            "k" => {
                let ref mut win = self.panes[0];
                let ref mut buf = self.buffers[win.buffer_index];
                buf.move_up();
                if buf.y < win.y {
                    win.scroll_up();
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
                let ref mut win = self.panes[0];
                let ref mut buf = self.buffers[win.buffer_index];
                buf.move_down_by(Editor::get_max_y());
                win.scroll_by(Editor::get_max_y());
            },
            "<C-b>" => {
                let ref mut win = self.panes[0];
                let ref mut buf = self.buffers[win.buffer_index];
                buf.move_down_by(Editor::get_max_y() * -1);
                win.scroll_by(Editor::get_max_y() * -1);
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
            Err(e) => ()
        }
    }

    pub fn draw(&self) {
        let ref buf = self.buffers[1];
        let ref win = self.panes[0];
        let y = win.y - buf.y;
        let x = buf.x;
        let lines = buf.lines.iter().skip(win.y as usize).take(Editor::get_max_y() as usize);
        for (index, line) in lines.enumerate() {
            mv(index as i32, 0);
            clrtoeol();
            addstr(line);
        }
        mv(buf.y - win.y, buf.x);
        refresh();
    }

    // Private

    fn get_max_y() -> i32 {
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        max_y
    }
}
