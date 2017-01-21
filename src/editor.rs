use std;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use ncurses::*;

use buffer::Buffer;

static COLOR_PAIR_DEFAULT: i16 = 1;

pub struct Editor {
    buffers: Vec<Buffer>,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            buffers: vec![],
        }
    }

    pub fn handle_input(&mut self, key: &str) {
        match key {
            "$" => {
                self.buffers[0].move_eol();
            },
            "0" => {
                self.buffers[0].move_bol();
            },
            "h" => {
                self.buffers[0].move_left();
            },
            "j" => {
                let ref mut buf = self.buffers[0];
                buf.move_down();
                if buf.cursor_y >= (buf.scroll_y + Editor::get_max_y() - 2) {
                    buf.scroll_down();
                }
            },
            "k" => {
                let ref mut buf = self.buffers[0];
                buf.move_up();
                if buf.cursor_y < buf.scroll_y {
                    buf.scroll_up();
                }
            },
            "l" => {
                self.buffers[0].move_right();
            },
            "<C-c>" => {
                endwin();
                std::process::exit(0);
            },
            "<C-f>" => {
                let ref mut buf = self.buffers[0];
                buf.cursor_y = buf.scroll_y + Editor::get_max_y() - 3;
                for _ in 2..(Editor::get_max_y()) {
                    buf.move_down();
                    if buf.cursor_y >= (buf.scroll_y + Editor::get_max_y() - 2) {
                        buf.scroll_down();
                    }
                }
            },
            "<C-b>" => {
                let ref mut buf = self.buffers[0];
                buf.cursor_y = buf.scroll_y;
                for _ in 1..(Editor::get_max_y()) {
                    buf.move_up();
                    if buf.cursor_y < buf.scroll_y {
                        buf.scroll_up();
                    }
                }
            },
            "<C-s>" => {
                self.save()
            }
            _ => ()
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
                self.buffers.push(buf);
            },
            Err(_) => ()
        }
    }

    pub fn save(&mut self) {
        let ref buf = self.buffers[1];
        match File::create(&buf.path) {
            Ok(mut f) => {
                f.write_all(buf.lines.join("\n").as_bytes());
            },
            Err(_) => ()
        }
    }

    pub fn draw(&self) {
        init_pair(COLOR_PAIR_DEFAULT, 3, -1);

        let ref buf = self.buffers[0];
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let lines = buf.lines.iter().skip(buf.scroll_y as usize).take(max_y as usize);

        for (index, line) in lines.enumerate() {
            wmove(buf.window, (index + 1) as i32, 0);
            wclrtoeol(buf.window);
            waddstr(buf.window, format!(" {}", line).as_str());
        }

        wattron(buf.window, COLOR_PAIR(COLOR_PAIR_DEFAULT));
        box_(buf.window, 0, 0);
        wattroff(buf.window, COLOR_PAIR(COLOR_PAIR_DEFAULT));

        // update cursor
        wresize(buf.window, max_y, max_x);
        wmove(buf.window, (buf.cursor_y - buf.scroll_y) + 1, buf.cursor_x + 1);
        wrefresh(buf.window);
    }

    pub fn get_max_y() -> i32 {
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        max_y
    }
}
