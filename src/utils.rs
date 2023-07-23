use std::io::Write;

pub fn move_cursor(pos_x: i32, pos_y: i32) {
    let mut height = 0;
    let mut width = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut height, &mut width);

    ncurses::mv(height + pos_y, pos_x);
}

pub fn write(editor: &mut crate::editor::Editor) {
    let mut s = String::new();
    for line in &editor.text {
        s.push_str(line);

        if line != &editor.text[editor.text.len() - 1] {
            s.push_str("\n");
        }
    }

    let mut file = std::fs::File::create(&editor.file_name).unwrap();
    file.write_all(s.as_bytes()).unwrap();
    editor.statusline_message = "File saved".to_string()
}
