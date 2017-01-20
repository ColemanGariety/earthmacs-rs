extern crate libc;
extern crate ncurses;
extern crate termkey;

use std::default::Default;
use std::{env};
use std::io::{Write, stdout, stdin};
use ncurses::*;
use termkey::*;

use editor::Editor;

mod editor;
mod buffer;
mod window;

fn main() {
    initscr();
    noecho();
    cbreak();
    keypad(stdscr(), true);

    let mut tk = TermKey::new(0, c::TERMKEY_FLAG_CTRLC);
    let ed = &mut Editor::new();

    if let Some(filename) = env::args().nth(1) {
        ed.open(filename);
        ed.draw();

        loop {
            match tk.waitkey() {
                TermKeyResult::Key(key) => {
                    ed.handle_input(&tk.strfkey(key, c::TERMKEY_FORMAT_VIM));
                    ed.draw();
                },
                _ => ()
            }
        }
    }
}
