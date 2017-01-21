use ncurses::*;

use mode::Mode;

pub struct Pane {
    pub buffer_index: usize,
    pub mode: Mode,
    pub y: i32,
    pub window: *mut i8,
}

impl Pane {
    pub fn new(buffer_index: usize) -> Pane {
        Pane {
            buffer_index: buffer_index,
            mode: Mode::normal(),
            y: 0,
            window: subwin(stdscr(), 0, 0, 0, 0),
        }
    }

    pub fn scroll_down(&mut self) {
        self.y += 1;
    }

    pub fn scroll_up(&mut self) {
        self.y -= 1;
    }
}
