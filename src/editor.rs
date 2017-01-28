use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use ncurses::*;

use buffer::Buffer;
use window::Window;
use window_tree::WindowTree;

const NORTH: usize = 1;
const SOUTH: usize = 2;
const EAST: usize = 3;
const WEST: usize = 4;

pub struct Editor {
    pub buffers: Vec<Buffer>,
    pub window_tree: WindowTree,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            buffers: vec![],
            window_tree: WindowTree::new(None),
        }
    }

    pub fn handle_input(&mut self, key: &str) {
        match key {
            "<M-h>" => { self.window_tree.focus(WEST) },
            "<M-j>" => { self.window_tree.focus(SOUTH) },
            "<M-k>" => { self.window_tree.focus(NORTH) },
            "<M-l>" => { self.window_tree.focus(EAST) },
            "<M-H>" => { self.split_towards(WEST); },
            "<M-J>" => { self.split_towards(SOUTH); },
            "<M-K>" => { self.split_towards(NORTH); },
            "<M-L>" => { self.split_towards(EAST); },
            "<C-q>" => { self.window_tree.find_active_window_tree().unwrap().destroy() },
            _ => {
                let ref mode = self.window_tree.find_active_window().unwrap().mode.clone();
                match mode.as_str() {
                    "normal" => { self.handle_normal(key); },
                    "delete" => { self.handle_delete(key); },
                    "insert" => { self.handle_insert(key); },
                    "find_char" => { self.handle_find_char(key); },
                    "replace" => { self.handle_replace(key); },
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
                self.window_tree = WindowTree::new(None);
                self.window_tree.leaf = Window::new();
                self.buffers.push(buf);
            },
            Err(_) => ()
        }

        self.window_tree.split_horizontally();
    }

    pub fn draw(&mut self) {
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        self.window_tree.draw(&self.buffers, max_x, max_y, 0, 0);
        let ref active = self.window_tree.find_active_window().unwrap();
        wmove(active.pane, active.cursor_y - active.scroll_y + 1, active.cursor_x + 1);
        wrefresh(active.pane);
    }

    fn split_towards(&mut self, direction: usize) {
        match direction {
            NORTH | SOUTH => {
                // self.window_tree.find_active_window().split_vertically();
            },
            EAST | WEST => {
                self.window_tree.find_active_window_tree().unwrap().split_horizontally();
            },
            _ => ()
        };
    }

}
