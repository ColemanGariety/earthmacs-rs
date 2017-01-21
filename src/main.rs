extern crate ncurses;
extern crate termkey;
extern crate libc;

use std::{env};
use ncurses::*;
use termkey::*;
use libc::c_int;
use editor::Editor;

mod editor;
mod buffer;

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
        let mut running: bool = true;
        let mut nextwait = -1;

        loop {
            ed.draw();

            let p = poll_rd1(0, nextwait);
            if p > 0 {
                tk.advisereadable();
            }
            loop {
                match tk.getkey() {
                    TermKeyResult::Key(key) => {
                        ed.handle_input(&tk.strfkey(key, c::TERMKEY_FORMAT_VIM));
                    },
                    Again => {
                        nextwait = tk.get_waittime() as i32; break;
                    },
                    _ => { nextwait = -1; break; }
                }
            }
        }
    }
}

pub mod poll_
{
    use libc::c_short;
    use libc::c_int;
    use libc::c_ulong;

    pub static POLLIN: c_short = 0x001;
    pub static POLLPRI: c_short = 0x002;
    pub static POLLOUT: c_short = 0x004;

    pub static POLLRDNORM: c_short = 0x040;
    pub static POLLRDBAND: c_short = 0x080;
    pub static POLLWRNORM: c_short = 0x100;
    pub static POLLWRBAND: c_short = 0x200;

    pub static POLLMSG: c_short = 0x400;
    pub static POLLREMOVE: c_short = 0x1000;
    pub static POLLRDHUP: c_short = 0x2000;

    pub static POLLERR: c_short = 0x008;
    pub static POLLHUP: c_short = 0x010;
    pub static POLLNVAL: c_short = 0x020;

    #[repr(C)]
    #[allow(non_camel_case_types)]
    pub struct pollfd
    {
        pub fd: c_int,
        pub events: c_short,
        pub revents: c_short,
    }
    extern
    {
        pub fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    }
}

pub fn poll_rd1(fd: i32, waittime: i32) -> i32
{
    let mut pfd = poll_::pollfd{fd: fd as c_int, events: poll_::POLLIN, revents: 0};
    unsafe
    {
        poll_::poll(&mut pfd, 1, waittime as c_int) as i32
    }
}
