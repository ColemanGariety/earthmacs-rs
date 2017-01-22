use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use ncurses::*;

use buffer::Buffer;
use window::Window;

static COLOR_PAIR_DEFAULT: i16 = 1;

const NORTH: usize = 1;
const SOUTH: usize = 2;
const EAST: usize = 3;
const WEST: usize = 4;

pub struct Editor {
    pub buffers: Vec<Buffer>,
    pub windows: Vec<Window>,
    pub focus_window_id: i32,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            buffers: vec![],
            windows: vec![],
            focus_window_id: 0,
        }
    }

    pub fn handle_input(&mut self, key: &str) {
        match key {
            "<M-h>" => { self.focus_towards(WEST) },
            "<M-j>" => { self.focus_towards(SOUTH) },
            "<M-k>" => { self.focus_towards(NORTH) },
            "<M-l>" => { self.focus_towards(EAST) },
            _ => {
                let ref mut window = self.windows[self.focus_window_id as usize];
                let ref mut buffer = self.buffers[window.buffer as usize];
                match window.mode.as_str() {
                    "normal" => { window.handle_normal(key, buffer); },
                    "delete" => { window.handle_delete(key, buffer); },
                    "insert" => { window.handle_insert(key, buffer); },
                    "find_char" => { window.handle_find_char(key, buffer); },
                    "replace" => { window.handle_replace(key, buffer); },
                    _ => ()
                }
            }
        }
    }


    pub fn open(&mut self, path: String) {
        match File::open(&path) {
            Ok(f) => {
                let mut buf = Buffer::new(path);
                let reader = BufReader::new(f);
                for line in reader.lines() {
                    buf.append_line(line.unwrap());
                }
                self.windows.push(Window::new(0, 0, 100, 100));
                self.buffers.push(buf);
            },
            Err(_) => ()
        }

        let new_win = self.windows[0].split_horizontally();
        self.windows.push(new_win);
    }

    pub fn draw(&mut self) {
        for (id, window) in self.windows.iter_mut().enumerate() {
            refresh();
            init_pair(COLOR_PAIR_DEFAULT, 3, -1);

            let ref buffer = self.buffers[window.buffer as usize];
            let lines = buffer.lines.iter().skip(window.scroll_y as usize).take(window.height as usize);

            for (index, line) in lines.enumerate() {
                wmove(window.pane, (index + 1) as i32, 0);
                wclrtoeol(window.pane);
                waddstr(window.pane, format!(" {}", line).as_str());
            }

            // // update cursor
            wresize(window.pane, window.real_height(), window.real_width());
            mvwin(window.pane, window.real_y(), window.real_x());
            if id == self.focus_window_id as usize {
                wattron(window.pane, COLOR_PAIR(COLOR_PAIR_DEFAULT));
            }
            box_(window.pane, 0, 0);
            wattroff(window.pane, COLOR_PAIR(COLOR_PAIR_DEFAULT));
            wrefresh(window.pane);
        }

        for (id, window) in self.windows.iter().enumerate() {
            if id == self.focus_window_id as usize {
                wmove(window.pane, (window.cursor_y - window.scroll_y) + 1, window.cursor_x + 1);
                wrefresh(window.pane);
            }
        }
    }

    fn focus_towards(&mut self, direction: usize) {
        let focus_x = self.windows[self.focus_window_id as usize].x.clone();
        let focus_y = self.windows[self.focus_window_id as usize].y.clone();
        let focus_width = self.windows[self.focus_window_id as usize].width.clone();
        let focus_height = self.windows[self.focus_window_id as usize].height.clone();
        for (index, window) in self.windows.iter_mut().enumerate() {
            let found = match direction {
                NORTH => window.y + window.height == focus_y,
                SOUTH => window.y == focus_y + focus_height,
                EAST => window.x == focus_x + focus_width,
                WEST => window.x + window.width == focus_x,
                _ => false,
            };
            if found {
                self.focus_window_id = index as i32;
            }
        }
    }

    fn get_focus_window_id(&mut self) -> &mut Window {
        &mut self.windows[self.focus_window_id as usize]
    }
}
