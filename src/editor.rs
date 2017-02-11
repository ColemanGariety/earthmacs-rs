use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use ncurses::*;
use std::path::PathBuf;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::easy::HighlightLines;
use std::path::Path;
use util;

use buffer::Buffer;
use window::Window;
use window_tree::WindowTree;
use cell::Cell;
use drawer::Drawer;

const NORTH: usize = 1;
const SOUTH: usize = 2;
const EAST: usize = 3;
const WEST: usize = 4;

pub struct Editor {
    pub buffers: Vec<Buffer>,
    pub window_tree: WindowTree,
    pub drawer: Option<Drawer>,
}


impl Editor {
    pub fn new() -> Editor {
        let mut window_tree = WindowTree::new(None);
        window_tree.leaf = Window::new();
        window_tree.leaf.active = true;
        Editor {
            buffers: vec![],
            window_tree: window_tree,
            drawer: None,
        }
    }

    pub fn handle_input(&mut self, key: &str) {
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        match key {
            "<M-h>" => { self.window_tree.focus(WEST, max_x, max_y, 0, 0) },
            "<M-j>" => { self.window_tree.focus(SOUTH, max_x, max_y, 0, 0) },
            "<M-k>" => { self.window_tree.focus(NORTH, max_x, max_y, 0, 0) },
            "<M-l>" => { self.window_tree.focus(EAST, max_x, max_y, 0, 0) },
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
                    "execute" => { self.handle_execute(key); },
                    "find_files" => { self.handle_find_files(key); },
                    _ => ()
                }
            }
        }
    }


    pub fn open(&mut self, path: PathBuf) {
        match File::open(&path) {
            Ok(f) => {
                let reader = BufReader::new(f);
                let mut ps = SyntaxSet::load_defaults_nonewlines();
                let ts = ThemeSet::load_defaults();
                let p = Path::new(&path);
                ps.link_syntaxes();
                match p.extension() {
                    Some(e) => {
                        if let Some(inst) = ps.find_syntax_by_extension(e.to_str().unwrap()) {
                            let mut buf = Buffer::new(path.clone(), Some(inst.clone()), Some(ts));
                            let theme = buf.ts.as_ref().unwrap().themes["base16-ocean.dark"].clone();
                            let mut h = HighlightLines::new(inst, &theme);
                            for (index, line) in reader.lines().enumerate() {
                                let ln = line.unwrap();
                                let ranges = h.highlight(&ln);
                                buf.lines.push(vec![]);
                                for (style, text) in ranges {
                                    let color = util::rgb_to_short(format!("{0:02.x}{1:02.x}{2:02.x}", style.foreground.r, style.foreground.g, style.foreground.b).as_str());
                                    for ch in text.chars() {
                                        buf.lines[index].push(Cell::new(ch, color as i32));
                                    }
                                }
                            }
                            self.buffers.push(buf);
                        }
                    }
                    None => {
                        let mut buf = Buffer::new(path.clone(), None, None);
                        for (index, line) in reader.lines().enumerate() {
                            buf.lines.push(vec![]);
                            let ln = line.unwrap();
                            for ch in ln.chars() {
                                buf.lines[index].push(Cell::new(ch, 0));
                            }
                        }
                        self.buffers.push(buf);
                    }
                };
            },
            Err(_) => ()
        }
    }

    pub fn draw(&mut self) {
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        self.window_tree.draw(&self.buffers, max_x, max_y, 0, 0);
        let ref active = self.window_tree.find_active_window().unwrap();

        match active.mode.clone().as_str() {
            "find_files" => {
                self.drawer.as_ref().unwrap().draw(max_x, max_y);
                refresh();
            },
            _ => {
                wmove(active.pane, active.cursor_y - active.scroll_y + 1, active.cursor_x + 1);
                wnoutrefresh(active.pane);
                doupdate();
            }
        }
    }

    fn split_towards(&mut self, direction: usize) {
        match direction {
            NORTH | SOUTH => {
                self.window_tree.find_active_window_tree().unwrap().split_vertically();
            },
            EAST | WEST => {
                self.window_tree.find_active_window_tree().unwrap().split_horizontally();
            },
            _ => ()
        };
    }

}

