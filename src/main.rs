extern crate ncurses;

use std::{char, env, ptr};
use ncurses::*;
use editor::Editor;

mod editor;
mod buffer;
mod window;

fn main() {
    initscr();
    noecho();
    cbreak();
    keypad(stdscr(), true);

    refresh();
    endwin();

    if let Some(filename) = env::args().nth(1) {
        let ed = &mut Editor::new();
        ed.open(filename);
        loop {
            let input: i32 = getch();
            ed.handleInput(input);
        }
    }
}
