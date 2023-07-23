use crate::{input::handle_input, normal::handle_normal, statusline::draw};

use super::mode;
use ncurses;

pub struct Editor {
    pub mode: mode::Mode,
    pub current_line: u32,
    pub current_column: u32,

    pub text: Vec<String>,
    pub statusline_message: String,

    pub file_name: String,
}

impl Editor {
    pub fn new(file_name: String) -> Editor {
        Editor {
            mode: mode::Mode::Normal,
            current_line: 0,
            current_column: 0,

            text: vec![String::new()],
            statusline_message: String::new(),

            file_name: file_name,
        }
    }

    pub fn run(&mut self) {
        ncurses::initscr();
        ncurses::noecho();

        ncurses::clear();
        loop {
            let ch = ncurses::getch();

            match self.mode {
                mode::Mode::Input => handle_input(ch, self),
                mode::Mode::Normal => handle_normal(ch, self),
            }

            draw(self);

        }
    }
}

