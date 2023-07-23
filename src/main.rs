mod normal;
mod input;
mod editor;
mod mode;
mod statusline;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file_name = &args[1];

    let mut editor = editor::Editor::new(file_name.to_string());
    editor.run();
}

