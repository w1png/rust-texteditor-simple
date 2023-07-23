use crate::editor::Editor;
use ncurses;

pub fn handle_input(input: i32, editor: &mut Editor) {
    match input {
        27 => editor.mode = crate::mode::Mode::Normal, // Escape
        10 => {
            editor.current_line += 1;
            editor.current_column = 0;
            editor.text.push(String::new());
        }, // Enter
        127 => {
            if editor.current_column > 0 {
                editor.current_column -= 1;
                editor.text[editor.current_line as usize].pop();
            }
            ncurses::mvaddch(editor.current_line as i32, editor.current_column as i32, ' '.try_into().unwrap());
        }, // Backspace
        _ => {
            ncurses::addstr(&format!("{}", input as u8 as char));
            editor.current_column += 1;
            editor.text[editor.current_line as usize].push(input as u8 as char);

            return;
        }
    }
}

