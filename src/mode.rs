use std::cmp::{min};
use std;
use editor::Editor;
use drawer::Drawer;
use std::path::Path;
use ncurses::*;

impl Editor {
    pub fn handle_normal(&mut self, key: &str) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        // let window_height = self.window_tree.active_window_height(max_x, max_y).unwrap();
        let window_height = max_y;
        let window = self.window_tree.find_active_window().unwrap();
        let ref mut buffer = self.buffers[window.buffer_index as usize];

        match key {
            "$" => {
                window.cursor_x = buffer.eol(window.cursor_y);
                window.col = 99999999;
            },
            "0" => { window.move_bol(); },
            "A" => {
                window.mode = "insert".to_string();
                while window.cursor_x < buffer.lines[window.cursor_y as usize].len() as i32 {
                    window.move_right();
                }
            },
            "d" => { window.mode = "delete".to_string(); },
            "f" => { window.mode = "find_char".to_string(); },
            "G" => {
                while window.scroll_y < buffer.lines.len() as i32 { // 
                    window.move_down();
                }
            }
            "h" => {
                if window.cursor_x > 0 {
                    window.move_left();
                }
            },
            "i" => { window.mode = "insert".to_string(); }
            "j" => {
                if window.cursor_y < (buffer.eof() - 1) {
                    window.move_down();
                    if window.cursor_y >= (window.scroll_y + window_height) - 2 {
                        window.scroll_down()
                    }
                    window.cursor_x  = min(buffer.eol(window.cursor_y), window.col);
                }

            },
            "k" => {
                if window.cursor_y > 0 {
                    window.move_up();
                    window.cursor_x  = min(buffer.eol(window.cursor_y), window.col);
                }
            },
            "l" => {
                if window.cursor_x < buffer.eol(window.cursor_y) {
                    window.move_right();
                }
            },
            "r" => { window.mode = "replace".to_string() },
            "x" => {
                let x = window.cursor_x;
                let y = window.cursor_y;
                buffer.remove(x, y);
                if x == buffer.eol(y) + 1 {
                    window.move_left();
                }
            }
            "<C-b>" => {
                for _ in 1..(window_height - 2) {
                    window.move_up();
                    if window.cursor_y < window.scroll_y {
                        window.scroll_up();
                    }
                }
            },
            "<C-x>" => {
                window.mode = "execute".to_string();
            },
            "<C-c>" => { endwin(); std::process::exit(0); },
            "<C-f>" => {
                for _ in 1..(window_height - 2) {
                    if window.cursor_y < (buffer.eof() - 1) {
                        window.move_down();
                        window.cursor_x  = min(buffer.eol(window.cursor_y), window.col);
                        if window.cursor_y >= (window.scroll_y + window_height - 2) {
                            window.scroll_down();
                        }
                    }
                }
            },
            // "<C-q>" => { window.destroy_active_window()},
            "<C-s>" => { buffer.save(); },
            _ => ()
        }
    }

    pub fn handle_delete(&mut self, key: &str) {
        let ref mut window_tree = self.window_tree;
        let ref mut window = window_tree.find_active_window().unwrap();
        let ref mut buffer = self.buffers[window.buffer_index as usize];

        match key {
            "<Escape>" => {
                window.mode = "normal".to_string();
                window.move_left();
            },
            "d" => {
                let row = window.cursor_y;
                buffer.remove_line(row as usize);
                window.mode = "normal".to_string();
            },
            _ => ()
        }
    }

    pub fn handle_insert(&mut self, key: &str) {
        let ref mut window_tree = self.window_tree;
        let ref mut window = window_tree.find_active_window().unwrap();
        let ref mut buffer = self.buffers[window.buffer_index as usize];

        match key {
            "<Escape>" => {
                window.mode = "normal".to_string();
                window.move_left();
            },
            "<DEL>" | "<Backspace>" => {
                let x = window.cursor_x.clone();
                let y = window.cursor_y.clone();
                if x == 0 {
                    window.move_up();
                    while window.cursor_x < buffer.lines[window.cursor_y as usize].len() as i32 {
                        window.move_right();
                    }
                } else {
                    window.move_left();
                }
                buffer.remove(x - 1, y);
            },
            "<Enter>" => {
                buffer.insert_newline(window.cursor_x, window.cursor_y);
                window.move_down();
                window.move_bol();
            },
            _ => {
                buffer.insert(key, window.cursor_x, window.cursor_y);
                window.cursor_x += 1;
                window.col += 1;
            }
        }
    }

    pub fn handle_find_char(&mut self, key: &str) {
        let ref mut window_tree = self.window_tree;
        let ref mut window = window_tree.find_active_window().unwrap();
        let ref mut buffer = self.buffers[window.buffer_index as usize];

        match key {
            "<Escape>" => {
                window.mode = "normal".to_string();
                window.move_left();
            },
            _ => {
                let y = window.cursor_y as usize;
                let x = window.cursor_x as usize;
                match buffer.lines[y].iter().skip(x + 1).position(|c| char::to_string(&c.ch).as_str() == key) {
                    Some(i) => {
                        window.cursor_x += (i + 1) as i32;
                        window.col += (i + 1) as i32;
                    },
                    _ => ()
                }
                window.mode = "normal".to_string();
            }
        }
    }

    pub fn handle_replace(&mut self, key: &str) {
        let ref mut window_tree = self.window_tree;
        let ref mut window = window_tree.find_active_window().unwrap();
        let ref mut buffer = self.buffers[window.buffer_index as usize];

        match key {
            "<Escape>" => {
                window.mode = "normal".to_string();
                window.move_left();
            },
            _ => {
                let x = window.cursor_x;
                let y = window.cursor_y;
                buffer.remove(x, y);
                buffer.insert(key, x, y);
                window.mode = "normal".to_string();
            },
        }
    }

    pub fn handle_execute(&mut self, key: &str) {
        let ref mut window_tree = self.window_tree;
        let ref mut window = window_tree.find_active_window().unwrap();

        match key {
            "<C-f>" => {
                window.mode = "find_files".to_string();
                self.drawer = Some(Drawer::new_find_files());
            },
            _ => {
                window.mode = "normal".to_string();
            }
        }
    }

    pub fn handle_find_files(&mut self, key: &str) {
        match key {
            "<C-g>" => {
                let ref mut window = self.window_tree.find_active_window().unwrap();
                window.mode = "normal".to_string();
            },
            "<Enter>" => {
                let ref folder = self.drawer.as_ref().unwrap().value.clone();
                let ref filename = self.drawer.as_ref().unwrap().lines[self.drawer.as_ref().unwrap().active_line_index as usize].clone();
                self.open(Path::new(folder).join(filename));
                let ref mut active = self.window_tree.find_active_window().unwrap();
                active.buffer_index = (self.buffers.len() - 1) as i32;
                active.mode = "normal".to_string();
            },
            "<C-n>" => {
                self.drawer.as_mut().unwrap().next_item();
            },
            "<C-p>" => {
                self.drawer.as_mut().unwrap().prev_item();
            },
            _ => {}
        }
    }
}
