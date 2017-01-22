extern crate ncurses;
extern crate termkey;
extern crate libc;

use std::{env};
use ncurses::*;
use termkey::*;
use editor::Editor;

mod editor;
mod mode;
mod buffer;
mod poll;

fn main() {
    initscr();
    noecho();
    cbreak();
    keypad(stdscr(), true);
    start_color();
    use_default_colors();

    let ed = &mut Editor::new();

    if let Some(filename) = env::args().nth(1) {
        ed.open(filename);
        let mut tk = TermKey::new(0, c::TERMKEY_FLAG_CTRLC);
        let mut wait = -1;

        loop {
            ed.draw();
            if poll::poll_rd1(0, wait) > 0 { tk.advisereadable(); }
            match tk.getkey() {
                TermKeyResult::Key(key) => {
                    ed.buffers[0].handle_input(&tk.strfkey(key, c::TERMKEY_FORMAT_VIM));
                },
                _ => {
                    wait = tk.get_waittime() as i32;
                }
            }
        }
    }
}
