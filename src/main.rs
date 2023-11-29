use std::env;
use crossterm::terminal;

mod helpers;


struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {panic!("Wrong input format. Correct input format has the filename as the only arg.")}
    
    let path: String = args[1].clone();

    let mut content: String = helpers::load_file(&path);
    let current_char_index = 0;
    // let current_char = content[0] as char;

    terminal::enable_raw_mode().unwrap();
    helpers::runtime_loop(&mut content);

    // let mut buf = [0; 1];
    
    // while stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
    //     let c = buf[0] as char;
    //     println!("{}\r", c);
    //     // helpers::draw(&content, 0, 0);
    // }
    
    Ok(())
}