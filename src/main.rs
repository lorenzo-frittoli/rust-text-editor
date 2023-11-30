use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

mod editor;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {panic!("Wrong input format. Correct input format has the filename as the only arg.")}
    let path_string: OsString = OsString::from(args[1].clone());
    let path: &Path = Path::new(&path_string);
    let mut editor = editor::Editor::new();

    editor.open_file(path);
    editor.run();

    Ok(())
}