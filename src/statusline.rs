use ncurses::{self, addstr};

use crate::utils;

pub fn draw(editor: &mut crate::editor::Editor) {
    let editor_mode = editor.mode.to_string();

    let statusline = format!("{} | {}:{} {}",
        editor_mode,
        editor.current_line,
        editor.current_column,
        editor.statusline_message
    );

    editor.statusline_message = String::new();

    utils::move_cursor(0, -1);

    ncurses::clrtoeol();

    addstr(&statusline);

    ncurses::mv(editor.current_line as i32, editor.current_column as i32);

}
