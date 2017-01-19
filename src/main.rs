extern crate ncurses;

use std::{env};
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
        ed.draw();
        loop {
            let input: i32 = getch();
            ed.handle_input(input);
            ed.draw();
        }
    }
}
