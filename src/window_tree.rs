use std::cmp::{max};
use buffer::Buffer;
use window::Window;
use cell::Cell;
use ncurses::*;

static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_HIGHLIGHT: i16 = 2;

const NORTH: usize = 1;
const SOUTH: usize = 2;
const EAST: usize = 3;
const WEST: usize = 4;

#[derive(Clone)]
pub struct WindowTree {
    pub branches: Vec<WindowTree>,
    pub leaf: Window,
    pub parent: Option<*mut WindowTree>,
    pub direction: String,
}

pub struct VirtualWindow {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

impl WindowTree {
    pub fn new(parent: Option<*mut WindowTree>) -> WindowTree {
        WindowTree {
            branches: vec![],
            leaf: Window::new(),
            parent: parent,
            direction: "horizontal".to_string(),
        }
    }

    pub fn find_active_window(&mut self) -> Option<&mut Window> {
        if self.leaf.active {
            return Some(&mut self.leaf);
        } else {
            for branch in &mut self.branches {
                match branch.find_active_window() {
                    Some(leaf) => { return Some(leaf); },
                    None => ()
                }
            }
        }
        None
    }

    pub fn find_active_window_tree(&mut self) -> Option<&mut WindowTree> {
        if self.leaf.active {
            return Some(self);
        } else {
            for branch in &mut self.branches {
                match branch.find_active_window_tree() {
                    Some(tree) => { return Some(tree); },
                    None => ()
                }
            }
        }
        None
    }

    pub fn find_active_window_height(&mut self, width: i32, height: i32, x: i32, y: i32) -> i32 {
        let mut virtual_windows = vec![];
        virtual_draw(self, width, height, x, y, &mut virtual_windows);

        return virtual_windows.iter().find(|&&(ref leaf, _)| leaf.active).unwrap().1.height;

        fn virtual_draw<'a>(window: &'a mut WindowTree, width: i32, height: i32, x: i32, y: i32, windows: &mut Vec<(&'a mut Window, VirtualWindow)>) {
            let n = window.branches.len() as i32;
            if n > 0 {
                let mut extra_width = 0;
                let mut extra_height = 0;
                for (i, branch) in &mut window.branches.iter_mut().enumerate() {
                    if i == (n - 1) as usize {
                        extra_width = width % n;
                        extra_height = height % n;
                    }
                    if window.direction.as_str() == "horizontal" {
                        virtual_draw(branch, (width / n) + extra_width, height, x + ((width / n) * (i as i32)), y, windows);
                    } else {
                        virtual_draw(branch, width, (height / n) + extra_height, x, y + ((height / n) * (i as i32)), windows)
                    }
                }
            } else {
                windows.push((&mut window.leaf,VirtualWindow{
                    width: width,
                    height: height,
                    x: x,
                    y: y,
                }));
            }
        }
    }

    pub fn focus(&mut self, direction: usize, width: i32, height: i32, x: i32, y: i32) {
        let mut virtual_windows = vec![];
        virtual_draw(self, width, height, x, y, &mut virtual_windows);

        let active_index = virtual_windows.iter().position(|&(ref leaf, _)| leaf.active).unwrap();
        let next_index = virtual_windows.iter().position(|&(_, ref wm)| {
            let active = &virtual_windows[active_index];
            match direction {
                NORTH => {
                    wm.y + wm.height == active.1.y &&
                        wm.x <= active.1.x + active.0.cursor_x + 2 &&
                        wm.x + wm.width >= active.1.x + active.0.cursor_x + 2
                },
                SOUTH => {
                    wm.y == active.1.y + active.1.height &&
                        wm.x <= active.1.x + active.0.cursor_x + 2 &&
                        wm.x + wm.width >= active.1.x + active.0.cursor_x + 2
                },
                EAST => {
                    wm.x == active.1.x + active.1.width &&
                        wm.y <= active.1.y + (active.0.cursor_y - active.0.scroll_y) + 2 &&
                        wm.y + wm.height >= active.1.y + (active.0.cursor_y - active.0.scroll_y) + 2
                },
                WEST => {
                    wm.x + wm.width == active.1.x &&
                        wm.y <= active.1.y + (active.0.cursor_y - active.0.scroll_y) + 2 &&
                        wm.y + wm.height >= active.1.y + (active.0.cursor_y - active.0.scroll_y) + 2
                },
                _ => { false }
            }
        });

        match next_index {
            Some(next_index) => {
                virtual_windows[active_index].0.active = false;
                virtual_windows[next_index].0.active = true;
            },
            None => ()
        }

        fn virtual_draw<'a>(window: &'a mut WindowTree, width: i32, height: i32, x: i32, y: i32, windows: &mut Vec<(&'a mut Window, VirtualWindow)>) {
            let n = window.branches.len() as i32;
            if n > 0 {
                let mut extra_width = 0;
                let mut extra_height = 0;
                for (i, branch) in &mut window.branches.iter_mut().enumerate() {
                    if i == (n - 1) as usize {
                        extra_width = width % n;
                        extra_height = height % n;
                    }
                    if window.direction.as_str() == "horizontal" {
                        virtual_draw(branch, (width / n) + extra_width, height, x + ((width / n) * (i as i32)), y, windows);
                    } else {
                        virtual_draw(branch, width, (height / n) + extra_height, x, y + ((height / n) * (i as i32)), windows)
                    }
                }
            } else {
                windows.push((&mut window.leaf,VirtualWindow{
                    width: width,
                    height: height,
                    x: x,
                    y: y,
                }));
            }
        }
    }

    pub fn draw(&mut self, buffers: &Vec<Buffer>, width: i32, height: i32, x: i32, y: i32) {
        let n = self.branches.len() as i32;
        if n > 0 {
            let mut extra_width = 0;
            let mut extra_height = 0;
            for (i, branch) in &mut self.branches.iter_mut().enumerate() {
                if i == (n - 1) as usize {
                    extra_width = width % n;
                    extra_height = height % n;
                }
                if self.direction.as_str() == "horizontal" {
                    branch.draw(buffers, (width / n) + extra_width, height, x + ((width / n) * (i as i32)), y);
                } else {
                    branch.draw(buffers, width, (height / n) + extra_height, x, y + ((height / n) * (i as i32)))
                }
            }
        } else {
            let ref buffer = buffers[self.leaf.buffer_index as usize];
            let mut lines = buffer.lines.iter().skip(self.leaf.scroll_y as usize).take(height as usize);
            let ref spare = vec![Cell::new('\n', 0)];

            let mut has_mark = false;
            let mut starts_with_mark = false;
            match self.leaf.mark {
                Some(mark) => {
                    has_mark = true;
                    if mark.0 == self.leaf.row {
                        starts_with_mark = mark.1 <= self.leaf.col;
                    } else {
                        starts_with_mark = mark.0 < self.leaf.row;
                    }
                },
                None => ()
            }

            let mut marking = false;
            for y in 0..height {
                wmove(self.leaf.pane, (y + 1) as i32, 0);
                waddstr(self.leaf.pane, " ");
                wclrtoeol(self.leaf.pane);

                match lines.next() {
                    Some(mut line) => {
                        let mut cells = line.iter();

                        for x in 0..width {

                            // calc mark region
                            if has_mark {
                                let mark = self.leaf.mark.unwrap();
                                if starts_with_mark && y + self.leaf.scroll_y == mark.0 && x as i32 == mark.1 ||
                                    !starts_with_mark && y + self.leaf.scroll_y == self.leaf.row && x as i32 == max(0, self.leaf.cursor_x) ||
                                    starts_with_mark && self.leaf.scroll_y > mark.0 && y == 0
                                {
                                    marking = true;
                                }

                                if starts_with_mark && y + self.leaf.scroll_y == self.leaf.row && x as i32 == max(0, self.leaf.cursor_x) ||
                                    !starts_with_mark && y + self.leaf.scroll_y == mark.0 && x as i32 == mark.1 + 1
                                {
                                    marking = false;
                                }
                            }

                            // highlight mark region
                            // and print cell
                            if let Some(ch) = cells.next() {
                                if marking {
                                    wattroff(self.leaf.pane, COLOR_PAIR(ch.fg as i16));
                                    wattron(self.leaf.pane, COLOR_PAIR(COLOR_PAIR_HIGHLIGHT));
                                } else {
                                    wattroff(self.leaf.pane, COLOR_PAIR(COLOR_PAIR_HIGHLIGHT));
                                    wattron(self.leaf.pane, COLOR_PAIR(ch.fg as i16));
                                }
                                waddstr(self.leaf.pane, ch.ch.to_string().as_str());
                            } else {
                                if marking {
                                    wattron(self.leaf.pane, COLOR_PAIR(COLOR_PAIR_HIGHLIGHT));
                                } else {
                                    wattroff(self.leaf.pane, COLOR_PAIR(COLOR_PAIR_HIGHLIGHT));
                                }
                                waddstr(self.leaf.pane, " ");
                            }
                        }
                    },
                    None => ()
                }
            }

            // position/size
            wresize(self.leaf.pane, height, width);
            mvwin(self.leaf.pane, y, x);

            // border
            if self.leaf.active {
                wattron(self.leaf.pane, COLOR_PAIR(COLOR_PAIR_DEFAULT));
            }
            box_(self.leaf.pane, 0, 0);

            // name label
            let name = buffer.path.file_name().unwrap().to_str().unwrap();
            if width >= name.len() as i32 + 4 {
                wmove(self.leaf.pane, height - 1, 4);
                waddstr(self.leaf.pane, name);
            }

            // (x,y) label
            let xy_label = format!("({},{})", self.leaf.cursor_y, self.leaf.cursor_x);
            if width >= (4 + name.len() + 4) as i32 + xy_label.len() as i32 {
                wmove(self.leaf.pane, height - 1, (4 + name.len() + 4) as i32);
                waddstr(self.leaf.pane, xy_label.as_str());
            }

            if width >= self.leaf.mode.len() as i32 + 4 {
                // mode label
                wmove(self.leaf.pane, height - 1, width - 4 - self.leaf.mode.len() as i32);
                waddstr(self.leaf.pane, self.leaf.mode.as_str());
            }

            wattroff(self.leaf.pane, COLOR_PAIR(COLOR_PAIR_DEFAULT));

            // refresh
            wnoutrefresh(self.leaf.pane);
        }
    }

    pub fn split_horizontally(&mut self) {
        self.direction = "horizontal".to_string();
        let mut left = WindowTree::new(Some(self));
        left.leaf = self.leaf.clone();
        left.leaf.active = true;
        let mut right = WindowTree::new(Some(self));
        right.leaf = self.leaf.clone();
        right.leaf.pane = newwin(1,1,0,0);
        right.leaf.active = false;
        self.branches.push(left);
        self.branches.push(right);
        self.leaf = Window::new();
    }

    pub fn split_vertically(&mut self) {
        self.split_horizontally();
        self.direction = "vertical".to_string();
    }

    pub fn find_leaf(&mut self) -> Option<&mut Window> {
        let len = self.branches.len();
        if self.branches.len() > 0 {
            match self.branches[len - 1].find_leaf() {
                Some(leaf) => { return Some(leaf) },
                None => ()
            }
        } else {
            return Some(&mut self.leaf)
        }
        None
    }

    pub fn reparent_branches(&mut self) {
        let parent = self as *mut WindowTree;
        for branch in &mut self.branches {
            branch.parent = Some(parent);
            branch.reparent_branches();
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            match self.parent {
                Some(tree) => {
                    let ref mut parent = *tree;
                    for branch in &parent.branches {
                        if branch.leaf.active == false {
                            if branch.branches.len() > 0 {
                                (*self.parent.unwrap()).branches = branch.branches.clone();
                            } else {
                                (*self.parent.unwrap()).leaf = branch.leaf.clone();
                                (*self.parent.unwrap()).branches = vec![];
                            }
                        }
                    }
                    (*self.parent.unwrap()).reparent_branches();
                    (*self.parent.unwrap()).find_leaf().unwrap().active = true;
                },
                None => ()
            }
        }
    }

}
