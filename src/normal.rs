use crate::{editor::Editor, utils};

pub fn handle_normal(input: i32, editor: &mut Editor) {
    match input {
        105 => {
            editor.mode = crate::mode::Mode::Input;
        }, // i
        119 => {
            utils::write(editor);
        } // s
        120 => {
            utils::write(editor);
            ncurses::endwin();
            std::process::exit(0);
        } // x
        113 => {
            ncurses::endwin();
            std::process::exit(0);
        }, // q
        3 => {
            ncurses::endwin();
        }, // Ctrl-C
        _ => (),
    }
}
