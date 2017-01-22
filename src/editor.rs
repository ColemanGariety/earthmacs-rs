use std;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use ncurses::*;

use buffer::Buffer;

static COLOR_PAIR_DEFAULT: i16 = 1;

pub struct Editor {
    pub buffers: Vec<Buffer>,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            buffers: vec![],
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
