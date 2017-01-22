use std;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use editor::Editor;
use buffer::Buffer;
use ncurses::*;

impl Buffer {
    pub fn handle_normal(&mut self, key: &str) {
        match key {
            "$" => {
                self.move_eol();
            },
            "0" => {
                self.move_bol();
            },
            "h" => {
                self.move_left();
            },
            "j" => {
                self.move_down();
                if self.cursor_y >= (self.scroll_y + Editor::get_max_y() - 2) {
                    self.scroll_down();
                }
            },
            "k" => {
                self.move_up();
                if self.cursor_y < self.scroll_y {
                    self.scroll_up();
                }
            },
            "l" => {
                self.move_right();
            },
            "<C-c>" => {
                endwin();
                std::process::exit(0);
            },
            "<C-f>" => {
                for _ in 1..(Editor::get_max_y() - 2) {
                    self.move_down();
                    if self.cursor_y >= (self.scroll_y + Editor::get_max_y() - 2) {
                        self.scroll_down();
                    }
                }
            },
            "<C-b>" => {
                for _ in 1..(Editor::get_max_y() - 2) {
                    self.move_up();
                    if self.cursor_y < self.scroll_y {
                        self.scroll_up();
                    }
                }
            },
            "<C-s>" => {
                self.save();
            }
            _ => ()
        }
    }
}
