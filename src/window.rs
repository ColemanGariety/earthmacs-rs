use ncurses::*;

pub struct Window {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub pane: *mut i8,
    pub scroll_y: i32,
}

impl Window {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Window {
        Window {
            x: x,
            y: y,
            width: width,
            height: height,
            scroll_y: 0,
            pane: subwin(stdscr(), 0, 0, 0, 0),
        }
    }

    pub fn split_horizontally(&mut self) -> Window {
        self.width = self.width / 2;
        Window {
            x: self.width,
            y: self.y,
            width: self.width,
            height: self.height,
            scroll_y: 0,
            pane: subwin(stdscr(), 0, 0, 0, 0)
        }
    }

    pub fn real_height(&mut self) -> i32 {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        return (max_y as f64 * (self.height as f64 / 100.0)) as i32;
    }

    pub fn real_width(&mut self) -> i32 {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        return (max_x as f64 * (self.width as f64 / 100.0)) as i32;
    }

    pub fn real_x(&mut self) -> i32 {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        return (max_x as f64 * (self.x as f64 / 100.0)) as i32;
    }

    pub fn real_y(&mut self) -> i32 {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        return (max_y as f64 * (self.y as f64 / 100.0)) as i32;
    }
}
