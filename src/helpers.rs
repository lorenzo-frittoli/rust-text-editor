use std::fs::{self, OpenOptions};
use std::io::stdout;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::{
    execute,
    terminal,
    style::{Color, Print, ResetColor, SetForegroundColor},
};


pub fn load_file(path: &str) -> String {
    let exists = fs::metadata(path).is_ok();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(!exists)
        .open(path)
        .expect("Unable to open file");

    if !exists {
        println!("File created!")
    }
    
    return fs::read_to_string(path).expect("Unable to read file");
}

pub fn runtime_loop(content: &mut String) {
    let current_char_index: usize = 0;
    let mut char_vec: Vec<char> = (*content.chars().collect::<Vec<_>>()).to_vec();
    loop {
        draw(content, 0, 0);

        if let Event::Key(event) = event::read().unwrap() {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::CONTROL,
                    kind,
                    state,
                } => break,
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: event::KeyModifiers::NONE,
                    kind,
                    state,
                } => 
                    if let Some(current_char_index) = char_vec.get_mut(current_char_index) {
                        *current_char_index = c;
                        print!("{}", c);
                }
                _ => {
                    // TODO
                }
            }
        };
    }
}

pub fn draw(content: &str, starting_line: u32, current_char_index: i32) {
    // stdout().execute(Clear());
    for (i,l) in content.lines().enumerate() {
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            // Print(format!("{}\t", i)),
            // ResetColor,
            // Print(format!("{}\n", l))
            Print(format!("{}\r", l))
        ).unwrap();
    }
}