use std::cmp::{min,max};
use std::process::{Command, Stdio};
use std::io::{Read, Write};
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
        let window_height = self.window_tree.find_active_window_height(max_x, max_y, 0, 0);
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
            "F" => { window.mode = "find_char_backwards".to_string(); },
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
            "O" => {
                buffer.insert_newline(0, window.cursor_y);
                window.move_bol();
                window.mode = "insert".to_string();
            },
            "o" => {
                buffer.insert_newline(0, window.cursor_y + 1);
                window.move_down();
                window.move_bol();
                window.mode = "insert".to_string();
            },
            "p" => {
                let p = Command::new("xsel")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .arg("--clipboard")
                    .arg("--output")
                    .spawn()
                    .expect("Failed to grab from clipboard. Is xsel installed?");

                let mut s = String::new();
                p.stdout.unwrap().read_to_string(&mut s).expect("Failed to grab from clipboard. Make sure xsel is working properly.");
                let mut lines = s.split("\n");
                if lines.clone().count() == 1 {
                    buffer.insert(lines.next().unwrap(), window.cursor_x, window.cursor_y);
                } else {
                    let length = lines.clone().count();
                    for (index, line) in lines.enumerate() {
                        if index != length - 1 {
                            buffer.insert_newline(window.cursor_x, window.cursor_y + index as i32);
                        }
                        buffer.insert(line, window.cursor_x, window.cursor_y + index as i32);
                    }
                }
            },
            "r" => { window.mode = "replace".to_string() },
            "v" => {
                window.mode = "visual".to_string();
                window.mark = Some((window.cursor_y, window.cursor_x))
            }
            "x" => {
                let x = window.cursor_x;
                let y = window.cursor_y;
                buffer.remove(x, y);
                if x == buffer.eol(y) + 1 {
                    window.move_left();
                }
            },
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
                window.cursor_x  = min(buffer.eol(window.cursor_y), window.cursor_x);
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

                if y == 0 && x == 0 { return; }

                if x == 0 {
                    window.move_up();
                    while window.cursor_x < buffer.lines[window.cursor_y as usize].len() as i32 {
                        window.move_right();
                    }
                } else {
                    window.move_left();
                }
                match buffer.char_at(x, y) {
                    Some(ch1) => {
                        match buffer.char_at(x - 1, y) {
                            Some (ch2) => {
                                match (ch2, ch1) {
                                    ('"', '"') |
                                    ('\'', '\'') |
                                    ('(', ')') |
                                    ('{', '}') |
                                    ('[', ']') => { buffer.remove(x, y); },
                                    (_, _) => ()
                                }
                            },
                            None => ()
                        }
                    },
                    None => ()
                }

                buffer.remove(x - 1, y);
            },
            "<Enter>" => {
                buffer.insert_newline(window.cursor_x, window.cursor_y);
                window.move_down();
                window.move_bol();
            },
            "\"" | "\'" => {
                buffer.insert(key, window.cursor_x, window.cursor_y);
                window.move_right();
                buffer.insert(key, window.cursor_x, window.cursor_y);
            },
            "(" => {
                buffer.insert("(", window.cursor_x, window.cursor_y);
                window.move_right();
                buffer.insert(")", window.cursor_x, window.cursor_y);
            },
            "{" => {
                buffer.insert("{", window.cursor_x, window.cursor_y);
                window.move_right();
                buffer.insert("}", window.cursor_x, window.cursor_y);
            },
            "[" => {
                buffer.insert("[", window.cursor_x, window.cursor_y);
                window.move_right();
                buffer.insert("]", window.cursor_x, window.cursor_y);
            },
            _ => {
                buffer.insert(key, window.cursor_x, window.cursor_y);
                window.move_right();
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

    pub fn handle_find_char_backwards(&mut self, key: &str) {
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
                match buffer.lines[y].iter().rev().skip(buffer.lines[y].len() - x).position(|c| char::to_string(&c.ch).as_str() == key) {
                    Some(i) => {
                        window.cursor_x -= (i + 1) as i32;
                        window.col -= (i + 1) as i32;
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
        let ref mut buffer = self.buffers[window.buffer_index as usize];

        match key {
            "<C-f>" => {
                window.mode = "find_files".to_string();
                self.drawer = Some(Drawer::new_find_files(&buffer.path));
            },
            _ => {
                window.mode = "normal".to_string();
            }
        }
    }

    pub fn handle_visual(&mut self, key: &str) {
        match key {
            "<Escape>" => {
                let window = self.window_tree.find_active_window().unwrap();
                window.mode = "normal".to_string();
                window.mark = None;
            },
            "d" | "x" => {
                let window = self.window_tree.find_active_window().unwrap();
                let ref mut buffer = self.buffers[window.buffer_index as usize];
                if let Some(((mut x, mut y), (endx, endy))) = window.calc_mark_region() {
                    while y > endy || x >= endx {
                        let length;
                        if y != endy {
                            length = (max(1, buffer.lines[(y - 1) as usize].len()) - 1) as i32;
                        } else {
                            length = 0;
                        }
                        buffer.remove(x, y);
                        if x == -1 && y != endy {
                            y -= 1;
                            x = length;
                        } else {
                            x -= 1;
                        }
                    }

                    window.cursor_x = max(0, x);
                    window.col = max(0, x);
                    window.cursor_y = max(0, y);
                    window.row = max(0, y);
                    window.mode = "normal".to_string();
                    window.mark = None;
                }
            },
            "y" => {
                let window = self.window_tree.find_active_window().unwrap();
                let ref mut buffer = self.buffers[window.buffer_index as usize];
                if let Some(((mut x, mut y), (endx, endy))) = window.calc_mark_region() {
                    let mut region = String::new();
                    while y > endy || x >= endx {
                        let length;
                        if y != endy {
                            length = (max(1, buffer.lines[(y - 1) as usize].len()) - 1) as i32;
                        } else {
                            length = 0;
                        }
                        let mut new;
                        match buffer.char_at(x, y) {
                            Some(ch) => { new = ch.to_string(); },
                            None => { new = "\n".to_string(); }
                        }
                        new.push_str(region.as_str());
                        region = new;
                        if x == -1 && y != endy {
                            y -= 1;
                            x = length;
                        } else {
                            x -= 1;
                        }
                    }

                    let mut p = Command::new("xsel")
                        .arg("--clipboard")
                        .arg("--input")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .ok().expect("Faled to set clipborad. Is xsel installed?");

                    window.mark = None;
                    window.mode = "normal".to_string();

                    p.stdin.as_mut().unwrap().write_all(region.as_bytes()).expect("Failed to set clipboard. Make sure xsel is working properly.");
                }
            },
            _ => { self.handle_normal(key); }
        }
    }

    pub fn handle_find_files(&mut self, key: &str) {
        match key {
            "<C-g>" => {
                let ref mut window = self.window_tree.find_active_window().unwrap();
                window.mode = "normal".to_string();
            },
            "<Enter>" => {
                let v = self.drawer.as_ref().unwrap().value.clone();
                let mut p = Path::new(&v);
                if !p.is_dir() {
                    p = p.parent().unwrap();
                }
                let ref value = p.join(self.drawer.as_ref().unwrap().lines[self.drawer.as_ref().unwrap().active_line_index as usize].as_str());
                self.open(Path::new(&value).to_path_buf());
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
            _ => {
                self.drawer.as_mut().unwrap().handle_key(key);
            }
        }
    }
}
