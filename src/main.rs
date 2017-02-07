extern crate syntect;
extern crate ncurses;
extern crate termkey;
extern crate libc;
extern crate ansi_term;
extern crate regex;
#[macro_use]
extern crate lazy_static;

static COLOR_PAIR_DEFAULT: i16 = 1;

use std::{env};
use ncurses::*;
use termkey::*;
use editor::Editor;
use std::path::Path;

mod editor;
mod mode;
mod buffer;
mod poll;
mod window;
mod window_tree;
mod cell;
mod util;
mod drawer;

fn main() {
    initscr();
    noecho();
    cbreak();
    keypad(stdscr(), true);
    start_color();
    use_default_colors();
    refresh();

    for i in 0..255 {
        init_pair(i, i, -1);
    }

    init_pair(COLOR_PAIR_DEFAULT, 3, -1);

    let ed = &mut Editor::new();

    if let Some(filename) = env::args().nth(1) {
        ed.open(Path::new(&filename).to_path_buf());
        ed.draw();

        let mut tk = TermKey::new(0, c::TERMKEY_FLAG_CTRLC);
        let mut wait = -1;
        loop {
            ed.draw();
            let p = poll::poll_rd1(0, wait);
            if p == 0 {
                match tk.getkey_force() {
                    TermKeyResult::Key(key) => {
                        ed.handle_input(&tk.strfkey(key, c::TERMKEY_FORMAT_VIM));
                        ed.draw();
                    }
                    _ => {}
                }
            }
            if p > 0 { tk.advisereadable(); }
            match tk.getkey() {
                TermKeyResult::Key(key) => {
                    ed.handle_input(&tk.strfkey(key, c::TERMKEY_FORMAT_VIM));
                    ed.draw();
                },
                _ => {
                    wait = tk.get_waittime() as i32;
                }
            }
        }
    }
}
